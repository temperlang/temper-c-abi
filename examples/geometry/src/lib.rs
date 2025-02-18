pub mod extras;
mod r#mod;
mod mod_c;
mod support;
pub use r#mod::*;
pub fn init(config: Option<support::Config>) -> support::Result<support::AsyncRunner> {
    support::CONFIG.get_or_init(|| config.unwrap_or_else(|| support::Config::default()));
    extras::init()?;
    r#mod::init()?;
    Ok(support::config().runner().clone())
}

#[no_mangle]
unsafe extern "C" fn geometry_init() {
    init(None).unwrap();
}
