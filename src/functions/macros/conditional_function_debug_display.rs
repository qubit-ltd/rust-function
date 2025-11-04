/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Function Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Conditional Function structs
//!
//! Generates standard Debug and Display trait implementations for Conditional
//! Function structs that have `function` and `predicate` fields but no `name` field.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (two or three type parameters)
//!
//! # Examples
//!
//! ```ignore
//! // For two type parameters
//! impl_conditional_function_debug_display!(BoxConditionalFunction<T, R>);
//!
//! // For three type parameters
//! impl_conditional_function_debug_display!(BoxConditionalBiFunction<T, U, R>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Conditional Function structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Conditional Function structs that have `function` and `predicate` fields
/// but no `name` field.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (two or three type parameters)
///
/// # Examples
///
/// ```ignore
/// // For two type parameters
/// impl_conditional_function_debug_display!(BoxConditionalFunction<T, R>);
///
/// // For three type parameters
/// impl_conditional_function_debug_display!(BoxConditionalBiFunction<T, U, R>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_function_debug_display {
    // Two generic parameters - Function types
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> std::fmt::Debug for $struct_name<$t, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("function", &self.function)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$t, $r> std::fmt::Display for $struct_name<$t, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.function,
                    self.predicate
                )
            }
        }
    };
    // Three generic parameters - BiFunction types
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> std::fmt::Debug for $struct_name<$t, $u, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("function", &self.function)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$t, $u, $r> std::fmt::Display for $struct_name<$t, $u, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.function,
                    self.predicate
                )
            }
        }
    };
}

pub(crate) use impl_conditional_function_debug_display;
