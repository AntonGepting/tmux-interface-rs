//! The [`format`][`crate::format`] module contains functions for working with tmux formats
//!
//! Two steps
//!
//! * Building format string
//! * Parsing variables output
//!
//!
//! # Build
//!
//!
//! ## Example
//!
//! ```text
//! #{window_active}
//! ```
//!
//! ```
//! let mut f = Format::new();
//! f.push(Variable::WindowActive);
//! f.to_string();
//! ```
//!
//! # Parse
//!
//! VariableOutput
//! FormatOutput
//! Format
//! Variable
//!
//!
//! # See Also
//! * [Tmux Manual -> Formats](https://man7.org/linux/man-pages/man1/tmux.1.html#FORMATS)
//!
pub mod formats;
pub mod formats_output;
pub mod variable;
pub mod variable_output;

#[cfg(test)]
#[path = "."]
mod formats_tests {
    mod formats_output_tests;
    mod formats_tests;
    mod variable_output_tests;
    mod variable_tests;
}