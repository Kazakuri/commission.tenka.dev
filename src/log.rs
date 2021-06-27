pub use tracing::{info, warn, debug, error, trace};

pub fn init() {
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "info");
  }
  
  tracing_subscriber::fmt::init();
}