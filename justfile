watch-server:
  cargo watch -q -c -w src/ -x run

watch-client:
  cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
  
