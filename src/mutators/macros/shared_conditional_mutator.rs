/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Shared Conditional Mutator Macro
//!
//! Generates Arc/Rc-based Conditional Mutator implementations
//!
//! For Arc/Rc-based conditional mutators, generates `and_then` and `or_else` methods,
//! as well as complete Mutator trait implementations.
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
//! * `$mutator_type` - Mutator wrapper type name
//! * `$mutator_trait` - Mutator trait name
//! * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
//! * `$extra_bounds` - Extra trait bounds
//!
//! # Usage Examples
//!
//! ```ignore
//! // Arc single-parameter Mutator
//! impl_shared_conditional_mutator!(
//!     ArcConditionalMutator<T>,
//!     ArcMutator,
//!     Mutator,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc single-parameter Mutator
//! impl_shared_conditional_mutator!(
//!     RcConditionalMutator<T>,
//!     RcMutator,
//!     Mutator,
//!     into_rc,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Arc/Rc-based Conditional Mutator implementations
///
/// This macro should be used at the top level (outside of any impl block)
/// to generate complete impl blocks for Arc/Rc-based conditional mutators.
/// It generates `and_then` and `or_else` methods, as well as complete Mutator
/// trait implementations.
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
/// * `$mutator_type` - Mutator wrapper type name
/// * `$mutator_trait` - Mutator trait name
/// * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
/// * `$extra_bounds` - Extra trait bounds
///
/// # Usage Location
///
/// This macro should be used at the top level, outside of any impl block,
/// typically in the same file as the struct definition. It generates a complete
/// impl block internally.
///
/// # Usage Examples
///
/// ```ignore
/// // At the top level, outside of any impl block
/// impl_shared_conditional_mutator!(
///     ArcConditionalMutator<T>,
///     ArcMutator,
///     Mutator,
///     into_arc,
///     Send + Sync + 'static
/// );
///
/// // Rc single-parameter Mutator
/// impl_shared_conditional_mutator!(
///     RcConditionalMutator<T>,
///     RcMutator,
///     Mutator,
///     into_rc,
///     'static
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_shared_conditional_mutator {
    // Single generic parameter - Mutator
    (
        $struct_name:ident < $t:ident >,
        $mutator_type:ident,
        $mutator_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
    ) => {
        impl<$t> $struct_name<$t>
        where
            $t: 'static,
        {
            /// Chains another mutator in sequence
            ///
            /// Combines the current conditional mutator with another mutator
            /// into a new mutator that implements the following semantics:
            ///
            /// When the returned mutator is called with an argument:
            /// 1. First, it checks the predicate of this conditional mutator
            /// 2. If the predicate is satisfied, it executes the internal
            ///    mutator of this conditional mutator
            /// 3. Then, **regardless of whether the predicate was satisfied**,
            ///    it unconditionally executes the `next` mutator
            ///
            /// In other words, this creates a mutator that conditionally
            /// executes the first action (based on the predicate), and then
            /// always executes the second action.
            ///
            /// # Parameters
            ///
            /// * `next` - The next mutator to execute (always executed)
            ///
            /// # Returns
            ///
            /// Returns a new combined mutator
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use std::sync::atomic::{AtomicI32, Ordering};
            ///
            /// let result = AtomicI32::new(0);
            ///
            /// let mutator1 = ArcMutator::new(|x: &mut i32| {
            ///     *x += 1;
            /// });
            ///
            /// let mutator2 = ArcMutator::new(|x: &mut i32| {
            ///     *x += 2;
            /// });
            ///
            /// let conditional = mutator1.when(|x| *x > 0);
            /// let chained = conditional.and_then(mutator2);
            ///
            /// let mut val = 5;
            /// chained.apply(&mut val);  // val = 5 + 1 + 2 = 8
            /// let mut val2 = -1;
            /// chained.apply(&mut val2); // val2 = -1 + 2 = 1 (not -1 + 1 + 2!)
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<M>(&self, mut next: M) -> $mutator_type<$t>
            where
                M: $mutator_trait<$t> + $($extra_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                let mut first_mutator = self.mutator.clone();
                $mutator_type::new(move |t| {
                    if first_predicate.test(t) {
                        first_mutator.apply(t);
                    }
                    next.apply(t);
                })
            }

            /// Adds an else branch
            ///
            /// Executes the original mutator when the condition is satisfied, otherwise
            /// executes else_mutator.
            ///
            /// # Parameters
            ///
            /// * `else_mutator` - The mutator for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new mutator with if-then-else logic
            #[allow(unused_mut)]
            pub fn or_else<M>(&self, mut else_mutator: M) -> $mutator_type<$t>
            where
                M: $mutator_trait<$t> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                let mut then_mutator = self.mutator.clone();
                $mutator_type::new(move |t| {
                    if predicate.test(t) {
                        then_mutator.apply(t);
                    } else {
                        else_mutator.apply(t);
                    }
                })
            }
        }
    };
}

pub(crate) use impl_shared_conditional_mutator;
