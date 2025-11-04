/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Supplier Methods Macro
//!
//! Generates when and and_then method implementations for Arc/Rc-based Supplier
//!
//! Generates conditional execution when method and chaining and_then method
//! for Arc/Rc-based suppliers that borrow &self (because Arc/Rc can be cloned).
//!
//! This macro supports single-parameter suppliers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `ArcSupplier<T>`
//! * `$return_type` - The return type for when (e.g., ArcConditionalSupplier)
//! * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
//! * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
//! * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Supplier Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$supplier_trait` | `$extra_bounds` |
//! |---------------|-----------------|----------------|------------------------|------------------|----------------|
//! | **ArcSupplier** | `ArcSupplier<T>` | ArcConditionalSupplier | into_arc | Supplier | Send + Sync + 'static |
//! | **RcSupplier** | `RcSupplier<T>` | RcConditionalSupplier | into_rc | Supplier | 'static |
//! | **ArcStatefulSupplier** | `ArcStatefulSupplier<T>` | ArcConditionalStatefulSupplier | into_arc | StatefulSupplier | Send + Sync + 'static |
//! | **RcStatefulSupplier** | `RcStatefulSupplier<T>` | RcConditionalStatefulSupplier | into_rc | StatefulSupplier | 'static |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter with Arc
//! impl_shared_supplier_methods!(
//!     ArcSupplier<T>,
//!     ArcConditionalSupplier,
//!     into_arc,
//!     Supplier,
//!     Send + Sync + 'static
//! );
//!
//! // Single-parameter with Rc
//! impl_shared_supplier_methods!(
//!     RcSupplier<T>,
//!     RcConditionalSupplier,
//!     into_rc,
//!     Supplier,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Arc/Rc-based Supplier
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates conditional execution when method and chaining
/// and_then method for Arc/Rc-based suppliers that borrow &self (because Arc/Rc
/// can be cloned).
///
/// This macro supports single-parameter suppliers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `ArcSupplier<T>`
/// * `$return_type` - The return type for when (e.g., ArcConditionalSupplier)
/// * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
/// * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
/// * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Supplier Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$supplier_trait` | `$extra_bounds` |
/// |---------------|-----------------|----------------|------------------------|------------------|----------------|
/// | **ArcSupplier** | `ArcSupplier<T>` | ArcConditionalSupplier | into_arc | Supplier | Send + Sync + 'static |
/// | **RcSupplier** | `RcSupplier<T>` | RcConditionalSupplier | into_rc | Supplier | 'static |
/// | **ArcStatefulSupplier** | `ArcStatefulSupplier<T>` | ArcConditionalStatefulSupplier | into_arc | StatefulSupplier | Send + Sync + 'static |
/// | **RcStatefulSupplier** | `RcStatefulSupplier<T>` | RcConditionalStatefulSupplier | into_rc | StatefulSupplier | 'static |
///
/// # Examples
///
/// ```ignore
/// // Single-parameter with Arc
/// impl_shared_supplier_methods!(
///     ArcSupplier<T>,
///     ArcConditionalSupplier,
///     into_arc,
///     Supplier,
///     Send + Sync + 'static
/// );
///
/// // Single-parameter with Rc
/// impl_shared_supplier_methods!(
///     RcSupplier<T>,
///     RcConditionalSupplier,
///     into_rc,
///     Supplier,
///     'static
/// );
/// ```
/// # Author
///
/// Haixing Hu
macro_rules! impl_shared_supplier_methods {
    // Single generic parameter
    ($struct_name:ident < $t:ident >, $return_type:ident, $predicate_conversion:ident, $supplier_trait:ident, $($extra_bounds:tt)+) => {
        pub fn when<P>(&self, predicate: P) -> $return_type<$t>
        where
            P: Predicate<$t> + $($extra_bounds)+,
        {
            $return_type {
                supplier: self.clone(),
                predicate: predicate.$predicate_conversion(),
            }
        }

        #[allow(unused_mut)]
        pub fn and_then<S>(&self, mut after: S) -> $struct_name<$t>
        where
            $t: 'static,
            S: $supplier_trait<$t> + $($extra_bounds)+,
        {
            let mut first = self.clone();
            $struct_name::new(move || {
                let _ = first.get();
                after.get()
            })
        }
    };
}

pub(crate) use impl_shared_supplier_methods;
