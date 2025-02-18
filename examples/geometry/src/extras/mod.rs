mod mod_c;
use crate::support::Pair;
pub(crate) fn init() -> crate::support::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<crate::support::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE
        .get_or_init(|| {
            std::f64::consts::PI;
            {
                TAU.get_or_init(|| 6.283185307179586f64);
            }
            Ok(())
        })
        .clone()
}
static TAU: std::sync::OnceLock<f64> = std::sync::OnceLock::new();
pub fn tau() -> f64 {
    *TAU.get().unwrap()
}
