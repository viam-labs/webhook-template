watch-server:
  cargo watch -q -c -w src/ -x run

watch-client:
  cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

build-mac tag:
  docker buildx build --platform linux/arm64 -t {{tag}} .

build-x86 tag:
  docker buildx build --platform linux/amd64 -t {{tag}} .

build-utils:
  docker pull rust:latest
  cd ~/rust-utils && docker run -it --rm -v ${PWD}:/src ghcr.io/viamrobotics/micro-rdk-canon cargo build --release

build-wheel:
  cp ~/rust-utils/target/release/libviam_rust_utils.so ~/viam-python-sdk/src/viam/rpc/
  sudo cp -r ~/viam-python-sdk ~/hook/ 

build: build-utils build-wheel
