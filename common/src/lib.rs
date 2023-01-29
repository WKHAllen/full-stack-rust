#[cfg(not(any(feature = "frontend", feature = "backend")))]
compile_error!("one of features \"frontend\" or \"backend\" must be enabled");

#[cfg(all(feature = "frontend", feature = "backend"))]
compile_error!("features \"frontend\" and \"backend\" cannot be enabled at the same time");

pub mod commands;
