/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Conditional Supplier Macro
//!
//! Generates Arc/Rc-based Conditional Supplier implementations
//!
//! For Arc/Rc-based conditional suppliers, generates `and_then` and `or_else` methods,
//! as well as complete Supplier trait implementations.
//!
//! Arc/Rc type characteristics:
//! - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
//! - Uses trait default implementations for `into_arc()` and `to_arc()`
//! - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
//! - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
//! - Implement complete `to_xxx()` methods (because they can Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$supplier_type` - Supplier wrapper type name
//! * `$supplier_trait` - Supplier trait name
//! * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
//! * `$extra_bounds` - Extra trait bounds
//!
//! # Usage Examples
//!
//! ```ignore
//! // Arc single-parameter Supplier
//! impl_shared_conditional_supplier!(
//!     ArcConditionalSupplier<T>,
//!     ArcSupplier,
//!     Supplier,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc single-parameter Supplier
//! impl_shared_conditional_supplier!(
//!     RcConditionalSupplier<T>,
//!     RcSupplier,
//!     Supplier,
//!     into_rc,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Arc/Rc-based Conditional Supplier implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Arc/Rc-based conditional suppliers, generates `and_then` and `or_else` methods,
/// as well as complete Supplier trait implementations.
///
/// Arc/Rc type characteristics:
/// - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
/// - Uses trait default implementations for `into_arc()` and `to_arc()`
/// - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
/// - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
/// - Implement complete `to_xxx()` methods (because they can Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$supplier_type` - Supplier wrapper type name
/// * `$supplier_trait` - Supplier trait name
/// * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
/// * `$extra_bounds` - Extra trait bounds
///
/// # Usage Examples
///
/// ```ignore
/// // Arc single-parameter Supplier
/// impl_shared_conditional_supplier!(
///     ArcConditionalSupplier<T>,
///     ArcSupplier,
///     Supplier,
///     into_arc,
///     Send + Sync + 'static
/// );
///
/// // Rc single-parameter Supplier
/// impl_shared_conditional_supplier!(
///     RcConditionalSupplier<T>,
///     RcSupplier,
///     Supplier,
///     into_rc,
///     'static
/// );
/// ```
/// # Author
///
/// Haixing Hu
macro_rules! impl_shared_conditional_supplier {
    // Single generic parameter - Supplier
    (
        $struct_name:ident < $t:ident >,
        $supplier_type:ident,
        $supplier_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
    ) => {
        impl<$t> $struct_name<$t>
        where
            $t: 'static,
        {
            /// Chains another supplier in sequence
            ///
            /// Combines the current conditional supplier with another supplier
            /// into a new supplier that implements the following semantics:
            ///
            /// When the returned supplier is called:
            /// 1. First, it checks the predicate of this conditional supplier
            /// 2. If the predicate is satisfied, it executes the internal
            ///    supplier of this conditional supplier
            /// 3. Then, **regardless of whether the predicate was satisfied**,
            ///    it unconditionally executes the `next` supplier and returns its result
            ///
            /// In other words, this creates a supplier that conditionally
            /// executes the first action (based on the predicate), and then
            /// always executes the second action and returns its result.
            ///
            /// # Parameters
            ///
            /// * `next` - The next supplier to execute (always executed, result returned)
            ///
            /// # Returns
            ///
            /// Returns a new combined supplier
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use std::sync::atomic::{AtomicI32, Ordering};
            ///
            /// let counter = AtomicI32::new(0);
            ///
            /// let supplier1 = ArcSupplier::new(|| {
            ///     counter.fetch_add(1, Ordering::SeqCst) as i32
            /// });
            ///
            /// let supplier2 = ArcSupplier::new(|| {
            ///     counter.fetch_add(2, Ordering::SeqCst) as i32
            /// });
            ///
            /// let conditional = supplier1.when(|x| x > 0);
            /// let chained = conditional.and_then(supplier2);
            ///
            /// let result = chained.get();  // supplier1 executed (if condition met), supplier2 always executed, returns supplier2 result
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<S>(&self, mut next: S) -> $supplier_type<$t>
            where
                S: $supplier_trait<$t> + $($extra_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                let mut first_supplier = self.supplier.clone();
                $supplier_type::new(move || {
                    if let Some(value) = first_predicate.test(&first_supplier.get()) {
                        if value {
                            let _ = first_supplier.get();
                        }
                    }
                    next.get()
                })
            }

            /// Adds an else branch
            ///
            /// Executes the original supplier when the condition is satisfied, otherwise
            /// executes else_supplier.
            ///
            /// # Parameters
            ///
            /// * `else_supplier` - The supplier for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new supplier with if-then-else logic
            #[allow(unused_mut)]
            pub fn or_else<S>(&self, mut else_supplier: S) -> $supplier_type<$t>
            where
                S: $supplier_trait<$t> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                let mut then_supplier = self.supplier.clone();
                $supplier_type::new(move || {
                    let test_value = then_supplier.get();
                    if predicate.test(&test_value) {
                        test_value
                    } else {
                        else_supplier.get()
                    }
                })
            }
        }
    };
}

pub(crate) use impl_shared_conditional_supplier;
