FROM rust:1.70.0 as builder

RUN USER=root cargo new --bin viam-webhook
WORKDIR ./viam-webhook
COPY . .
RUN cargo build --release
RUN rm src/*.rs

# should have python
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

COPY --from=builder /viam-webhook/target/release/viam-webhook ${APP}/viam-webhook

# install python reqs
ADD ./requirements.txt ./requirements.txt
RUN pip install -r requirements.txt

# copy over script
ADD ./hook.py ./hook.py

# launch hook server
EXPOSE 8080

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./viam-webhook"]
