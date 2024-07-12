pub mod compound;
pub mod file;
pub mod parser;
pub mod text;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "tui")]
pub mod tui;
