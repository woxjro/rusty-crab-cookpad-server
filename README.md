[![CI](https://github.com/woxjro/rusty-crab-cookpad-server/actions/workflows/main.yml/badge.svg)](https://github.com/woxjro/rusty-crab-cookpad-server/actions/workflows/main.yml)
# Rusty Crab Cookpad Server 🦀



### How to build
- use `Nightly Rust`

`$ rustup toolchain list`

`$ rustup override add nightly`


- database setup

`$ diesel migration run`

- launch API server

`$ cargo watch -x run`

https://diesel.rs/guides/getting-started




`psql -d rusty_crab_cookpad -U postgres`   
`\i ./data/init/user_types.sql`


