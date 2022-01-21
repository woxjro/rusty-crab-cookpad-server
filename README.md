[![CI](https://github.com/woxjro/rusty-crab-cookpad-server/actions/workflows/main.yml/badge.svg)](https://github.com/woxjro/rusty-crab-cookpad-server/actions/workflows/main.yml)
# Rusty Crab Cookpad Server 🦀

(frontend: https://github.com/woxjro/rusty-crab-cookpad-front)

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



https://user-images.githubusercontent.com/63214188/150461377-0a01ef17-0b07-48ad-904a-8cd5cc4b506d.mov





