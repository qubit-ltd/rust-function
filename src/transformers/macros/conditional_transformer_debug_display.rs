/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Transformer Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Conditional Transformer structs
//!
//! Generates standard Debug and Display trait implementations for Conditional
//! Transformer structs that have `transformer` and `predicate` fields but no `name` field.
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
//! impl_conditional_transformer_debug_display!(BoxConditionalTransformer<T, U>);
//!
//! // For three type parameters
//! impl_conditional_transformer_debug_display!(BoxConditionalBiTransformer<T, U, V>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Conditional Transformer structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Conditional Transformer structs that have `transformer` and `predicate` fields
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
/// impl_conditional_transformer_debug_display!(BoxConditionalTransformer<T, U>);
///
/// // For three type parameters
/// impl_conditional_transformer_debug_display!(BoxConditionalBiTransformer<T, U, V>);
// ```
macro_rules! impl_conditional_transformer_debug_display {
    // Two generic parameters
    ($struct_name:ident < $generic1:ident, $generic2:ident >) => {
        impl<$generic1, $generic2> std::fmt::Debug for $struct_name<$generic1, $generic2> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("transformer", &self.transformer)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$generic1, $generic2> std::fmt::Display for $struct_name<$generic1, $generic2> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.transformer,
                    self.predicate
                )
            }
        }
    };
    // Three generic parameters
    ($struct_name:ident < $generic1:ident, $generic2:ident, $generic3:ident >) => {
        impl<$generic1, $generic2, $generic3> std::fmt::Debug for $struct_name<$generic1, $generic2, $generic3> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("transformer", &self.transformer)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$generic1, $generic2, $generic3> std::fmt::Display for $struct_name<$generic1, $generic2, $generic3> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.transformer,
                    self.predicate
                )
            }
        }
    };
}

pub(crate) use impl_conditional_transformer_debug_display;
