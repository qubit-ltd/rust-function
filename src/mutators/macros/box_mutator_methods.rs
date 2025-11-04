/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Box Mutator Methods Macro
//!
//! Generates when and and_then method implementations for Box-based Mutator
//!
//! Generates conditional execution when method and chaining and_then method
//! for Box-based mutators that consume self (because Box cannot be cloned).
//!
//! This macro supports single-parameter mutators through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxMutator<T>`
//! * `$conditional_type` - The conditional mutator type for when (e.g., BoxConditionalMutator)
//! * `$mutator_trait` - Mutator trait name (e.g., Mutator, MutatorOnce)
//!
//! # Parameter Usage Comparison
//!
//! | Mutator Type | Struct Signature | `$conditional_type` | `$mutator_trait` |
//! |--------------|-----------------|----------------|-------------------|
//! | **Mutator** | `BoxMutator<T>` | BoxConditionalMutator | Mutator |
//! | **MutatorOnce** | `BoxMutatorOnce<T>` | BoxConditionalMutatorOnce | MutatorOnce |
//! | **StatefulMutator** | `BoxStatefulMutator<T>` | BoxConditionalStatefulMutator | StatefulMutator |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter mutator
//! impl_box_mutator_methods!(
//!     BoxMutator<T>,
//!     BoxConditionalMutator,
//!     Mutator
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Box-based Mutator
///
/// This macro should be used inside an impl block to generate conditional
/// execution when method and chaining and_then method for Box-based mutators
/// that consume self (because Box cannot be cloned).
///
/// This macro supports single-parameter mutators through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxMutator<T>`
/// * `$conditional_type` - The conditional mutator type for when (e.g., BoxConditionalMutator)
/// * `$mutator_trait` - Mutator trait name (e.g., Mutator, MutatorOnce)
///
/// # Parameter Usage Comparison
///
/// | Mutator Type | Struct Signature | `$conditional_type` | `$mutator_trait` |
/// |--------------|-----------------|----------------|-------------------|
/// | **Mutator** | `BoxMutator<T>` | BoxConditionalMutator | Mutator |
/// | **MutatorOnce** | `BoxMutatorOnce<T>` | BoxConditionalMutatorOnce | MutatorOnce |
/// | **StatefulMutator** | `BoxStatefulMutator<T>` | BoxConditionalStatefulMutator | StatefulMutator |
///
/// # Usage Location
///
/// This macro should be used inside an impl block for the struct type.
///
/// # Examples
///
/// ```ignore
/// impl<T> BoxMutator<T> {
///     // Inside an impl block
///     impl_box_mutator_methods!(
///         BoxMutator<T>,
///         BoxConditionalMutator,
///         Mutator
///     );
/// }
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_mutator_methods {
    // Single generic parameter - Mutator
    ($struct_name:ident < $t:ident >, $conditional_type:ident, $mutator_trait:ident) => {
        /// Creates a conditional mutator that executes based on predicate
        /// result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the mutation operation
        ///
        /// # Returns
        ///
        /// Returns a conditional mutator that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::sync::Arc;
        /// use std::sync::atomic::{AtomicI32, Ordering};
        /// use prism3_rust_function::mutators::*;
        ///
        /// let counter = Arc::new(AtomicI32::new(0));
        /// let mutator = BoxMutator::new({
        ///     let counter = Arc::clone(&counter);
        ///     move |value: &mut i32| {
        ///         *value += counter.fetch_add(1, Ordering::SeqCst);
        ///     }
        /// });
        ///
        /// let conditional = mutator.when(|value: &i32| *value > 0);
        /// let mut val = 1;
        /// conditional.apply(&mut val);  // val = 2 (1 + 1)
        /// let mut val2 = -1;
        /// conditional.apply(&mut val2); // not executed, val2 remains -1
        /// ```
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t>
        where
            P: Predicate<$t> + 'static,
        {
            $conditional_type {
                mutator: self,
                predicate: predicate.into_box(),
            }
        }

        /// Chains execution with another mutator, executing the current
        /// mutator first, then the subsequent mutator.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent mutator to execute after the current
        ///   mutator completes
        ///
        /// # Returns
        ///
        /// Returns a new mutator that executes the current mutator and
        /// the subsequent mutator in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::sync::Arc;
        /// use std::sync::atomic::{AtomicI32, Ordering};
        /// use prism3_rust_function::mutators::*;
        ///
        /// let counter1 = Arc::new(AtomicI32::new(0));
        /// let counter2 = Arc::new(AtomicI32::new(0));
        ///
        /// let mutator1 = BoxMutator::new({
        ///     let counter = Arc::clone(&counter1);
        ///     move |value: &mut i32| {
        ///         *value += counter.fetch_add(1, Ordering::SeqCst);
        ///     }
        /// });
        ///
        /// let mutator2 = BoxMutator::new({
        ///     let counter = Arc::clone(&counter2);
        ///     move |value: &mut i32| {
        ///         *value += counter.fetch_add(1, Ordering::SeqCst);
        ///     }
        /// });
        ///
        /// let chained = mutator1.and_then(mutator2);
        /// let mut val = 0;
        /// chained.apply(&mut val);
        /// // val = 2 (0 + 1 + 1)
        /// ```
        #[allow(unused_mut)]
        pub fn and_then<M>(self, mut after: M) -> $struct_name<$t>
        where
            Self: Sized + 'static,
            $t: 'static,
            M: $mutator_trait<$t> + 'static,
        {
            let mut first = self;
            $struct_name::new(move |t: &mut $t| {
                first.apply(t);
                after.apply(t);
            })
        }
    };
}

pub(crate) use impl_box_mutator_methods;
