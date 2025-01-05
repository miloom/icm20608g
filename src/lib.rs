#[cfg(all(feature = "rpi", feature = "embedded"))]
compile_error!("feature \"rpi\" and feature \"embedded\" cannot be enabled at the same time");
#[cfg(all(feature = "embedded", feature = "visualize"))]
compile_error!("feature \"embedded\" and feature \"visualize\" cannot be enabled at the same time");
#[cfg(all(not(feature = "embedded"), not(feature = "rpi")))]
compile_error!("feature \"embedded\" or feature \"rpi\" have to be enabled");

#[allow(dead_code)]
pub mod structs;
