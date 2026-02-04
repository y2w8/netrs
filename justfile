build mode="":
  cargo build --{{mode}}

run:
  cargo run

deploy:
  cargo build --release
