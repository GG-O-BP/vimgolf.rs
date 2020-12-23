[package]
name = "rust-web"
version = "0.1.0"
authors = ["The Rust Developers"]
edition = "2018"

[dependencies]
lazy_static = "1.2.0"
fluent = "0.13"
fluent-bundle = "0.6.0"
fluent-syntax = "0.10.0"
fluent-locale = "0.10.1"
handlebars-fluent = "0.2.0"
rand = "0.8"
regex = "1"
rocket = "0.4.6"
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.8.14"
sass-rs = "0.2.1"
reqwest = { version = "0.10.10", features = ["blocking", "json"] }
toml = "0.5"
serde_json = "1.0"
rust_team_data = { git = "https://github.com/rust-lang/team" }
handlebars = "1.1.0"
siphasher = "0.3.3"
percent-encoding = "2.1.0"

[dependencies.rocket_contrib]
version = "0.4"
default-features = false
features = ["handlebars_templates"]
