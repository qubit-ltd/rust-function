/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Supplier Clone Macro
//!
//! Generates Clone trait implementation for basic Supplier types
//!
//! Generates Clone implementation for Supplier structs that have `function`
//! and `name` fields. The function field is cloned using its inherent `clone`
//! method, which performs a shallow clone for smart pointers like `Arc` or `Rc`.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one type parameter)
//!
//! # Examples
//!
//! ```ignore
//! //!For single type parameter
//! impl_supplier_clone!(ArcSupplier<T>);
//!
//! // For single type parameter with Rc
//! impl_supplier_clone!(RcSupplier<T>);
//!
//! // For stateful supplier
//! impl_supplier_clone!(ArcStatefulSupplier<T>);
//!
//! // For stateful supplier with Rc
//! impl_supplier_clone!(RcStatefulSupplier<T>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for basic Supplier types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. It generates
/// Clone implementation for Supplier structs that have `function` and `name`
/// fields. The function field is cloned using its inherent `clone` method,
/// which performs a shallow clone for smart pointers like `Arc` or `Rc`.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (one type parameter)
///
/// # Examples
///
/// ```ignore
/// // For single type parameter with Arc
/// impl_supplier_clone!(ArcSupplier<T>);
///
/// // For single type parameter with Rc
/// impl_supplier_clone!(RcSupplier<T>);
///
/// // For stateful supplier with Arc
/// impl_supplier_clone!(ArcStatefulSupplier<T>);
///
/// // For stateful supplier with Rc
/// impl_supplier_clone!(RcStatefulSupplier<T>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_supplier_clone {
    // Single generic parameter
    ($struct_name:ident < $t:ident >) => {
        impl<$t> Clone for $struct_name<$t> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    name: self.name.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_supplier_clone;
