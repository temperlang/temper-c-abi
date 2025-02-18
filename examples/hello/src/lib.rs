mod support;
mod r#mod;
mod mod_c;
pub use r#mod::*;
pub fn init(config: Option<support::Config>) -> support::Result<support::AsyncRunner> {
    support::CONFIG.get_or_init(|| config.unwrap_or_else(|| support::Config::default()));
    r#mod::init()?;
    Ok(support::config().runner().clone())
}

#[no_mangle]
unsafe extern "C" fn hello_init() {
    init(None).unwrap();
}
