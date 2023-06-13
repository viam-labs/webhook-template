watch-server:
  cargo watch -q -c -w src/ -x run

watch-client:
  cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

build-mac tag:
  docker buildx build --platform linux/arm64 -t {{tag}} .

build-x86 tag:
  docker buildx build --platform linux/amd64 -t {{tag}} .
