/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Box Supplier Methods Macro
//!
//! Generates when and and_then method implementations for Box-based Supplier
//!
//! Generates conditional execution when method and chaining and_then method
//! for Box-based suppliers that consume self (because Box cannot be cloned).
//!
//! This macro supports single-parameter suppliers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxSupplier<T>`
//! * `$conditional_type` - The conditional supplier type for when (e.g., BoxConditionalSupplier)
//! * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
//!
//! # Parameter Usage Comparison
//!
//! | Supplier Type | Struct Signature | `$conditional_type` | `$supplier_trait` |
//! |---------------|-----------------|----------------|------------------|
//! | **Supplier** | `BoxSupplier<T>` | BoxConditionalSupplier | Supplier |
//! | **SupplierOnce** | `BoxSupplierOnce<T>` | BoxConditionalSupplierOnce | SupplierOnce |
//! | **StatefulSupplier** | `BoxStatefulSupplier<T>` | BoxConditionalStatefulSupplier | StatefulSupplier |
//!
//! # Examples
//!
//! ```ignore
//! impl_box_supplier_methods!(
//!     BoxSupplier<T>,
//!     BoxConditionalSupplier,
//!     Supplier
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Box-based Supplier
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// Generates conditional execution when method and chaining and_then method
/// for Box-based suppliers that consume self (because Box cannot be cloned).
///
/// This macro supports single-parameter suppliers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxSupplier<T>`
/// * `$conditional_type` - The conditional supplier type for when (e.g., BoxConditionalSupplier)
/// * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
///
/// # Parameter Usage Comparison
///
/// | Supplier Type | Struct Signature | `$conditional_type` | `$supplier_trait` |
/// |---------------|-----------------|----------------|------------------|
/// | **Supplier** | `BoxSupplier<T>` | BoxConditionalSupplier | Supplier |
/// | **SupplierOnce** | `BoxSupplierOnce<T>` | BoxConditionalSupplierOnce | SupplierOnce |
/// | **StatefulSupplier** | `BoxStatefulSupplier<T>` | BoxConditionalStatefulSupplier | StatefulSupplier |
///
/// # Examples
///
/// ```ignore
/// // Single-parameter supplier
/// impl_box_supplier_methods!(
///     BoxSupplier<T>,
///     BoxConditionalSupplier,
///     Supplier
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_supplier_methods {
    // Single generic parameter - Supplier
    ($struct_name:ident < $t:ident >, $conditional_type:ident, $supplier_trait:ident) => {
        /// Creates a conditional supplier that executes based on predicate
        /// result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the supply operation
        ///
        /// # Returns
        ///
        /// Returns a conditional supplier that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::sync::Arc;
        /// use std::sync::atomic::{AtomicI32, Ordering};
        /// use prism3_rust_function::suppliers::*;
        ///
        /// let counter = Arc::new(AtomicI32::new(0));
        /// let supplier = BoxSupplier::new({
        ///     let counter = Arc::clone(&counter);
        ///     move || {
        ///         counter.fetch_add(1, Ordering::SeqCst) as i32
        ///     }
        /// });
        ///
        /// let conditional = supplier.when(|value: &i32| *value > 0);
        /// let result = conditional.get();
        /// // Only executes if predicate returns true
        /// ```
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t>
        where
            P: Predicate<$t> + 'static,
        {
            $conditional_type {
                supplier: self,
                predicate: predicate.into_box(),
            }
        }

        /// Chains execution with another supplier, executing the current
        /// supplier first, then the subsequent supplier.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent supplier to execute after the current
        ///   supplier completes
        ///
        /// # Returns
        ///
        /// Returns a new supplier that executes the current supplier and
        /// the subsequent supplier in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::sync::Arc;
        /// use std::sync::atomic::{AtomicI32, Ordering};
        /// use prism3_rust_function::suppliers::*;
        ///
        /// let counter1 = Arc::new(AtomicI32::new(0));
        /// let counter2 = Arc::new(AtomicI32::new(0));
        ///
        /// let supplier1 = BoxSupplier::new({
        ///     let counter = Arc::clone(&counter1);
        ///     move || {
        ///         counter.fetch_add(1, Ordering::SeqCst) as i32
        ///     }
        /// });
        ///
        /// let supplier2 = BoxSupplier::new({
        ///     let counter = Arc::clone(&counter2);
        ///     move || {
        ///         counter.fetch_add(2, Ordering::SeqCst) as i32
        ///     }
        /// });
        ///
        /// let chained = supplier1.and_then(supplier2);
        /// let result = chained.get();
        /// // supplier1 executed first, then supplier2
        /// ```
        #[allow(unused_mut)]
        pub fn and_then<S>(self, mut after: S) -> $struct_name<$t>
        where
            Self: Sized + 'static,
            $t: 'static,
            S: $supplier_trait<$t> + 'static,
        {
            let mut first = self;
            $struct_name::new(move || {
                let _ = first.get();
                after.get()
            })
        }
    };
}

pub(crate) use impl_box_supplier_methods;
