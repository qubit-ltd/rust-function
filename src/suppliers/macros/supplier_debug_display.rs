/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Supplier Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Supplier structs
//!
//! Generates standard Debug and Display trait implementations for Supplier
//! structs that have a `name: Option<String>` field.
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
//! impl_supplier_debug_display!(BoxSupplier<T>);
//!
//! // For stateful supplier
//! impl_supplier_debug_display!(BoxStatefulSupplier<T>);
//!
//! // For supplier once
//! impl_supplier_debug_display!(BoxSupplierOnce<T>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Supplier structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Supplier structs that have a `name: Option<String>` field.
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
/// impl_supplier_debug_display!(BoxSupplier<T>);
///
/// // For stateful supplier
/// impl_supplier_debug_display!(BoxStatefulSupplier<T>);
///
/// // For supplier once
/// impl_supplier_debug_display!(BoxSupplierOnce<T>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_supplier_debug_display {
    // Single generic parameter
    ($struct_name:ident < $t:ident >) => {
        impl<$t> std::fmt::Debug for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("name", &self.name)
                    .field("function", &"<function>")
                    .finish()
            }
        }

        impl<$t> std::fmt::Display for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self.name {
                    Some(name) => write!(f, "{}({})", stringify!($struct_name), name),
                    None => write!(f, "{}", stringify!($struct_name)),
                }
            }
        }
    };
}

pub(crate) use impl_supplier_debug_display;