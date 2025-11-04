/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Supplier Clone Macro
//!
//! Generates Clone trait implementation for Conditional Supplier types
//!
//! Generates Clone implementation for Conditional Supplier structs that have
//! `supplier` and `predicate` fields. Both fields are cloned using their
//! respective Clone implementations.
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
//! impl_conditional_supplier_clone!(ArcConditionalSupplier<T>);
//! impl_conditional_supplier_clone!(RcConditionalSupplier<T>);
//!
//! // For stateful supplier
//! impl_conditional_supplier_clone!(ArcConditionalStatefulSupplier<T>);
//! impl_conditional_supplier_clone!(RcConditionalStatefulSupplier<T>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for Conditional Supplier types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. Generates
/// Clone implementation for Conditional Supplier structs that have `supplier`
/// and `predicate` fields. Both fields are cloned using their respective
/// Clone implementations.
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
/// impl_conditional_supplier_clone!(ArcConditionalSupplier<T>);
/// impl_conditional_supplier_clone!(RcConditionalSupplier<T>);
///
/// // For stateful supplier
/// impl_conditional_supplier_clone!(ArcConditionalStatefulSupplier<T>);
/// impl_conditional_supplier_clone!(RcConditionalStatefulSupplier<T>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_supplier_clone {
    // Single generic parameter
    ($struct_name:ident < $t:ident >) => {
        impl<$t> Clone for $struct_name<$t> {
            fn clone(&self) -> Self {
                Self {
                    supplier: self.supplier.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_conditional_supplier_clone;
