/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Consumer Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Conditional Consumer structs
//!
//! Generates standard Debug and Display trait implementations for Conditional
//! Consumer structs that have `consumer` and `predicate` fields but no `name` field.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one or more type parameters)
//!
//! # Examples
//!
//! ```ignore
//! // For single type parameter
//! impl_conditional_consumer_debug_display!(BoxConditionalConsumer<T>);
//!
//! // For two type parameters
//! impl_conditional_consumer_debug_display!(BoxConditionalBiConsumer<T, U>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Conditional Consumer structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Conditional Consumer structs that have `consumer` and `predicate` fields
/// but no `name` field.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (one or more type parameters)
/// * `u` - Generic parameter list (one or more type parameters)
///
/// # Examples
///
/// ```ignore
/// // For single type parameter
/// impl_conditional_consumer_debug_display!(BoxConditionalConsumer<T>);
///
/// // For two type parameters
/// impl_conditional_consumer_debug_display!(BoxConditionalBiConsumer<T, U>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_consumer_debug_display {
    // Single generic parameter - Consumer types
    ($struct_name:ident < $t:ident >) => {
        impl<$t> std::fmt::Debug for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("consumer", &self.consumer)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$t> std::fmt::Display for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.consumer,
                    self.predicate
                )
            }
        }
    };

    // Two generic parameters - BiConsumer types
    ($struct_name:ident < $t:ident, $u:ident >) => {
        impl<$t, $u> std::fmt::Debug for $struct_name<$t, $u> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("consumer", &self.consumer)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$t, $u> std::fmt::Display for $struct_name<$t, $u> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.consumer,
                    self.predicate
                )
            }
        }
    };
}

pub(crate) use impl_conditional_consumer_debug_display;
