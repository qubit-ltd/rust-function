/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Box Conditional Supplier Macro
//!
//! Generates Box-based Conditional Supplier implementations
//!
//! For Box-based conditional suppliers, generates `and_then` and `or_else` methods,
//! as well as complete Supplier trait implementations.
//!
//! Box type characteristics:
//! - `and_then` and `or_else` consume self (because Box cannot Clone)
//! - Does not implement `into_arc()` (because Box types are not Send + Sync)
//! - Does not implement `to_xxx()` methods (because Box types cannot Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$supplier_type` - Supplier wrapper type name
//! * `$supplier_trait` - Supplier trait name
//!
//! # Usage Examples
//!
//! ```ignore
//! // Single-parameter Supplier
//! impl_box_conditional_supplier!(
//!     BoxConditionalSupplier<T>,
//!     BoxSupplier,
//!     Supplier
//! );
//!
//! // Stateful Supplier
//! impl_box_conditional_supplier!(
//!     BoxConditionalStatefulSupplier<T>,
//!     BoxStatefulSupplier,
//!     StatefulSupplier
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Box-based Conditional Supplier implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Box-based conditional suppliers, generates `and_then` and `or_else` methods,
/// as well as complete Supplier trait implementations.
///
/// Box type characteristics:
/// - `and_then` and `or_else` consume self (because Box cannot Clone)
/// - Does not implement `into_arc()` (because Box types are not Send + Sync)
/// - Does not implement `to_xxx()` methods (because Box types cannot Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$supplier_type` - Supplier wrapper type name
/// * `$supplier_trait` - Supplier trait name
///
/// # Usage Examples
///
/// ```ignore
/// // Single-parameter Supplier
/// impl_box_conditional_supplier!(
///     BoxConditionalSupplier<T>,
///     BoxSupplier,
///     Supplier
/// );
///
/// // Stateful Supplier
/// impl_box_conditional_supplier!(
///     BoxConditionalStatefulSupplier<T>,
///     BoxStatefulSupplier,
///     StatefulSupplier
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_conditional_supplier {
    // Single generic parameter - Supplier
    (
        $struct_name:ident<$t:ident>,
        $supplier_type:ident,
        $supplier_trait:ident
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
            /// let supplier1 = BoxSupplier::new(|| {
            ///     counter.fetch_add(1, Ordering::SeqCst) as i32
            /// });
            ///
            /// let supplier2 = BoxSupplier::new(|| {
            ///     counter.fetch_add(2, Ordering::SeqCst) as i32
            /// });
            ///
            /// let conditional = supplier1.when(|x| x > 0);
            /// let chained = conditional.and_then(supplier2);
            ///
            /// let result = chained.get();  // supplier1 executed (if condition met), supplier2 always executed, returns supplier2 result
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<S>(self, mut next: S) -> $supplier_type<$t>
            where
                S: $supplier_trait<$t> + 'static,
            {
                let first_predicate = self.predicate;
                let mut first_supplier = self.supplier;
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
            pub fn or_else<S>(self, mut else_supplier: S) -> $supplier_type<$t>
            where
                S: $supplier_trait<$t> + 'static,
            {
                let predicate = self.predicate;
                let mut then_supplier = self.supplier;
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

pub(crate) use impl_box_conditional_supplier;
