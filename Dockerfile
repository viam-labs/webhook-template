# build webhook
FROM rust:1.70.0 as rbuilder

RUN USER=root cargo new --bin viam-webhook
WORKDIR ./viam-webhook
COPY ./viam-webhook .
RUN cargo build
RUN rm src/*.rs

# build go sdk script
FROM golang:bullseye as gobuilder
WORKDIR ./gohook

COPY gohook/ ./
RUN go mod download
RUN go build -o /gohook

# intstall python deps and run webhook
FROM python:3.11.4-slim-bullseye 
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=rbuilder /viam-webhook/target/debug/viam-webhook ${APP}/viam-webhook

COPY --from=gobuilder /gohook ${APP}/gohook

ADD ./requirements.txt ./requirements.txt
RUN pip install -r requirements.txt

# copy over script
ADD ./hook.py ${APP}/hook.py

# launch hook server
EXPOSE 8080

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

# Jaeger configs
# ENV OTEL_EXPORTER_JAEGER_AGENT_HOST="my-jaeger-instance.internal" \
#     OTEL_EXPORTER_JAEGER_AGENT_PORT="6831" \
#     OTEL_EXPORTER_SERVICE_NAME="hook"

CMD ["./viam-webhook"]
