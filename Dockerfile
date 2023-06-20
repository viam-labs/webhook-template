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

# ADD ./requirements.txt ./requirements.txt
#  RUN pip install -r requirements.txt
ADD ./viam-python-sdk ./viam-python-sdk
RUN pip install poetry
RUN cd viam-python-sdk && poetry build && cd ..
RUN pip install viam-python-sdk/dist/viam_sdk-0.4.1-py3-none-any.whl
RUN cp viam-python-sdk/src/viam/rpc/libviam_rust_utils.so /usr/local/lib/python3.11/site-packages/viam/rpc/


# copy over script
ADD ./hook.py ${APP}/hook.py

# launch hook server
EXPOSE 8080

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./viam-webhook"]
