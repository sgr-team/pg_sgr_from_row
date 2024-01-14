docker compose -f ./tests/docker-compose.yml up -d
cargo test --no-default-features --features=sync
cargo test --no-default-features --features=tokio