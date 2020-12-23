[package]
name = "rust-web"
version = "0.1.0"
authors = ["The Rust Developers"]
edition = "2018"

[dependencies]
lazy_static = "*"
fluent = "*"
fluent-bundle = "*"
fluent-syntax = "*"
fluent-locale = "*"
handlebars-fluent = "*"
rand = "*"
regex = "*"
rocket = "*"
serde = { version = "*", features = ["derive"] }
serde_yaml = "*"
sass-rs = "*"
reqwest = { version = "*", features = ["blocking", "json"] }
toml = "*"
serde_json = "*"
rust_team_data = { git = "https://github.com/rust-lang/team" }
handlebars = "*"
siphasher = "*"
percent-encoding = "*"

[dependencies.rocket_contrib]
version = "*"
default-features = false
features = ["handlebars_templates"]
