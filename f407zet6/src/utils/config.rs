//!
//! # Config
//!

#[::toml_cfg::toml_config]
struct Config {
    #[default("")]
    version: &'static str,
}
