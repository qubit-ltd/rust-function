/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Mutator Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Conditional Mutator structs
//!
//! Generates standard Debug and Display trait implementations for Conditional
//! Mutator structs that have `mutator` and `predicate` fields but no `name` field.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one type parameter)
//!
//! # Examples
//!
//! ```ignore
//! // For single type parameter
//! impl_conditional_mutator_debug_display!(BoxConditionalMutator<T>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Conditional Mutator structs
///
/// This macro should be used at the top level (outside of any impl block)
/// to generate Debug and Display trait implementations for Conditional Mutator
/// structs. It generates standard Debug and Display trait implementations for
/// Conditional Mutator structs that have `mutator` and `predicate` fields but
/// no `name` field.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (one type parameter)
///
/// # Usage Location
///
/// This macro should be used at the top level, outside of any impl block,
/// typically in the same file as the struct definition.
///
/// # Examples
///
/// ```ignore
/// // At the top level, outside of any impl block
/// impl_conditional_mutator_debug_display!(BoxConditionalMutator<T>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_mutator_debug_display {
    // Single generic parameter
    ($struct_name:ident < $generic:ident >) => {
        impl<$generic> std::fmt::Debug for $struct_name<$generic> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("mutator", &self.mutator)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$generic> std::fmt::Display for $struct_name<$generic> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.mutator,
                    self.predicate
                )
            }
        }
    };
}

pub(crate) use impl_conditional_mutator_debug_display;
