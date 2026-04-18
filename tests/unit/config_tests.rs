use nuggies::core::config::Config;

#[test]
fn default_config_loads() {
    let config = Config::load();

    // Just ensure it doesn't panic and returns valid struct
    assert!(config.default_format.len() > 0);
}
