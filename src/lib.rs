#![no_std]

//! A traditional C-style switch expression. In contrast to traditional `match` expressions, the
//! [switch!](switch) macro will test against expressions, not patterns. The API of this crate is
//! tiny, with just a single item.
//!
//! # Example
//! ```
//! use easy_switch::switch;
//!
//! let num = 50;
//! let case1 = 20;
//! let case2 = 30;
//! let matching = switch! { num;
//!     case1 => "case1",
//!     case2 => "case2",
//!     25 + 25 => "unnamed case",
//!     _ => "default case",
//! };
//! assert_eq!("unnamed case", matching);
//! ```
//!
//! # Multiple Conditions
//! Multiple conditions can be chained together with a comma.
//! ```
//! use easy_switch::switch;
//!
//! #[derive(PartialEq, Eq)]
//! struct Example {
//!     field: i32,
//!     field2: i32,
//! }
//!
//! impl Example {
//!     pub fn new(field2: i32) -> Self {
//!         Self {
//!             field: 10,
//!             field2,
//!         }
//!     }
//! }
//!
//! let switchable = Example::new(10);
//! let result = switch! { switchable;
//!     Example::new(50), 50 == 50 => 50,
//!     Example::new(20), 50 == 50, 20 == 20 => 20,
//!     _ => 0,
//! };
//! assert_eq!(0, result);
//! ```

/// The macro used to emulate switch statements.
///
/// See the [crate level docs](../easy_switch/index.html) for more.
///
/// # Example
///
/// ```
/// use easy_switch::switch;
///
/// let var = 30;
/// let matched = switch! { var;
///     15 + 15 => true,
///     22 => {
///         println!("22?");
///         false
///     },
///     _ => false,
/// };
/// assert!(matched);
#[macro_export]
macro_rules! switch {
    ($input:expr; $first:expr$(, $($conditions:expr),*)? => $execute:expr, $($rest:expr$(, $($conditions_rest:expr),*)? => $exec:expr),*$(, _ => $execute_last:expr)?$(,)?) => {
        if $first == $input $(&& $($conditions)&&*)? { $execute }
        $(else if $rest == $input $(&& $($conditions_rest)&&*)? { $exec })*
        $(else { $execute_last })?
    };
    ($input:expr; $first:expr$(, $($conditions:expr),*)? => $execute:expr, _ => $execute_last:expr$(,)?) => {
        if $first == $input $(&& $($conditions)&&*)? { $execute }
        else { $execute_last }
    }
}
