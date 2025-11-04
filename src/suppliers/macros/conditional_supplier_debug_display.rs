/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Conditional Supplier Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Conditional Supplier structs
//!
//! Generates standard Debug and Display trait implementations for Conditional
//! Supplier structs that have `supplier` and `predicate` fields but no `name` field.
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
//! impl_conditional_supplier_debug_display!(BoxConditionalSupplier<T>);
//!
//! // For stateful supplier
//! impl_conditional_supplier_debug_display!(BoxConditionalStatefulSupplier<T>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Conditional Supplier structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Conditional Supplier structs that have `supplier` and `predicate` fields
/// but no `name` field.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (one type parameter)
///
/// # Examples
///
/// ```ignore
/// // For single type parameter
/// impl_conditional_supplier_debug_display!(BoxConditionalSupplier<T>);
///
/// // For stateful supplier
/// impl_conditional_supplier_debug_display!(BoxConditionalStatefulSupplier<T>);
/// ```
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_supplier_debug_display {
    // Single generic parameter
    ($struct_name:ident < $t:ident >) => {
        impl<$t> std::fmt::Debug for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("supplier", &self.supplier)
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
                    self.supplier,
                    self.predicate
                )
            }
        }
    };
}

pub(crate) use impl_conditional_supplier_debug_display;
