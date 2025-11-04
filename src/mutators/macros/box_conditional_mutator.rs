/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Box Conditional Mutator Macro
//!
//! Generates Box-based Conditional Mutator implementations
//!
//! For Box-based conditional mutators, generates `and_then` and `or_else` methods,
//! as well as complete Mutator trait implementations.
//!
//! Box type characteristics:
//! - `and_then` and `or_else` consume self (because Box cannot Clone)
//! - Does not implement `into_arc()` (because Box types are not Send + Sync)
//! - Does not implement `to_xxx()` methods (because Box types cannot Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$mutator_type` - Mutator wrapper type name
//! * `$mutator_trait` - Mutator trait name
//!
//! # Usage Examples
//!
//! ```ignore
//! // Single-parameter Mutator
//! impl_box_conditional_mutator!(
//!     BoxConditionalMutator<T>,
//!     BoxMutator,
//!     Mutator
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Box-based Conditional Mutator implementations
///
/// This macro should be used at the top level (outside of any impl block)
/// to generate complete impl blocks for Box-based conditional mutators.
/// It generates `and_then` and `or_else` methods, as well as complete Mutator
/// trait implementations.
///
/// Box type characteristics:
/// - `and_then` and `or_else` consume self (because Box cannot Clone)
/// - Does not implement `into_arc()` (because Box types are not Send + Sync)
/// - Does not implement `to_xxx()` methods (because Box types cannot Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$mutator_type` - Mutator wrapper type name
/// * `$mutator_trait` - Mutator trait name
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
/// impl_box_conditional_mutator!(
///     BoxConditionalMutator<T>,
///     BoxMutator,
///     Mutator
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_conditional_mutator {
    // Single generic parameter - Mutator
    (
        $struct_name:ident<$t:ident>,
        $mutator_type:ident,
        $mutator_trait:ident
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
            /// let mutator1 = BoxMutator::new(|x: &mut i32| {
            ///     *x += 1;
            /// });
            ///
            /// let mutator2 = BoxMutator::new(|x: &mut i32| {
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
            pub fn and_then<M>(self, mut next: M) -> $mutator_type<$t>
            where
                M: $mutator_trait<$t> + 'static,
            {
                let first_predicate = self.predicate;
                let mut first_mutator = self.mutator;
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
            pub fn or_else<M>(self, mut else_mutator: M) -> $mutator_type<$t>
            where
                M: $mutator_trait<$t> + 'static,
            {
                let predicate = self.predicate;
                let mut then_mutator = self.mutator;
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

pub(crate) use impl_box_conditional_mutator;
