use crate::support::Pair;

pub(crate) fn init() -> crate::support::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<crate::support::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(|| Ok(())).clone()
}

pub fn greeting_for(name__1: impl crate::support::ToArcString) -> std::sync::Arc<String> {
    let name__1 = name__1.to_arc_string();
    return std::sync::Arc::new(format!("Hello, {}!", name__1.clone()));
}
