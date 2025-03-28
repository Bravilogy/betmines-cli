use env_logger::{Builder, Env};

pub fn setup_logging() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "auto");

    Builder::from_env(env)
        .format_timestamp(None)
        .format_module_path(false)
        .init();
}
