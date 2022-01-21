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




|                            | o 対 o 　 | 関係(中間テーブル)                   |
| -------------------------- | --------- | ---------------------- |
| 「ユーザ」と「ユーザ種別」 | １対１    |                        |
| 「ユーザ種別」と「権限」   | １対多    | 「付与されている権限」 |
| 「ユーザ」と「レシピ」     | １対多    | 「作成したレシピ」     |
| 「ユーザ」と「レシピ」     | １対多    | 「お気に入りレシピ」   |
| 「レシピ」と「手順」       | １対多    | 「料理手順」           |
| 「レシピ」と「材料・分量」 | １対多    | 「使用する材料」       |
| 「レシピ」と「コメント」   | １対多    | 「レシピへのコメント」 |
| 「レシピ」と「タグ」       | 多対多    | 「属するタグ」         |
| 「レシピ」と「カテゴリ」   | 多対多    | 「属するカテゴリ」     |

- users
- recipes
- comments
- user_types
- authorities
- categories
- ingredients
- procedures
- tags
- recipes_categories
- recipes_tags
- user_types_authorities