//! # UI – Zed UI Primitives & Components
//!
//! This crate provides a set of UI primitives and components that are used to build all of the elements in Zed's UI.
//!
//! ## Related Crates:
//!
//! - [`ui_macros`] - proc_macros support for this crate
//! - `ui_input` - the single line input component

pub mod component_prelude;
mod components;
pub mod prelude;
mod styles;
pub mod tr;
mod tr_more;

/// Yer tutuculu metinleri çevirip biçimlendirir — `format!`'ın çeviri bilen hâli.
///
/// ```ignore
/// tr_format!("Changes since {}", branch)   // → "main dalından beri değişiklikler"
/// ```
///
/// Yalnızca `{}` ve `{0}` biçimindeki yer tutucularla kullanılmalıdır;
/// argümanlar `Display` ile metne çevrilir. Ayrıntı: [`tr::format_translated`].
#[macro_export]
macro_rules! tr_format {
    ($template:literal $(, $arg:expr)* $(,)?) => {
        $crate::tr::format_translated($template, &[$(::std::format!("{}", $arg)),*])
    };
}
mod traits;
pub mod utils;

pub use components::*;
pub use prelude::*;
pub use styles::*;
pub use traits::animation_ext::*;
