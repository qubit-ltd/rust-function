/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
/*! # Shared Mutator Methods Macro
//!
//! Generates when and and_then method implementations for Arc/Rc-based Mutator
//!
//! Generates conditional execution when method and chaining and_then method
//! for Arc/Rc-based mutators that borrow &self (because Arc/Rc can be cloned).
//!
//! This macro supports single-parameter mutators through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `ArcMutator<T>`
//! * `$return_type` - The return type for when (e.g., ArcConditionalMutator)
//! * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
//! * `$mutator_trait` - Mutator trait name (e.g., Mutator, MutatorOnce)
//! * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Mutator Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$mutator_trait` | `$extra_bounds` |
//! |--------------|-----------------|----------------|------------------------|------------------|----------------|
//! | **ArcMutator** | `ArcMutator<T>` | ArcConditionalMutator | into_arc | Mutator | Send + Sync + 'static |
//! | **RcMutator** | `RcMutator<T>` | RcConditionalMutator | into_rc | Mutator | 'static |
//! | **ArcStatefulMutator** | `ArcStatefulMutator<T>` | ArcConditionalStatefulMutator | into_arc | StatefulMutator | Send + Sync + 'static |
//! | **RcStatefulMutator** | `RcStatefulMutator<T>` | RcConditionalStatefulMutator | into_rc | StatefulMutator | 'static |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter with Arc
//! impl_shared_mutator_methods!(
//!     ArcMutator<T>,
//!     ArcConditionalMutator,
//!     into_arc,
//!     Mutator,
//!     Send + Sync + 'static
//! );
//!
//! // Single-parameter with Rc
//! impl_shared_mutator_methods!(
//!     RcMutator<T>,
//!     RcConditionalMutator,
//!     into_rc,
//!     Mutator,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu
*/

/// Generates when and and_then method implementations for Arc/Rc-based Mutator
///
/// This macro should be used inside an impl block to generate conditional
/// execution when method and chaining and_then method for Arc/Rc-based mutators
/// that borrow &self (because Arc/Rc can be cloned).
///
/// This macro supports single-parameter mutators through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `ArcMutator<T>`
/// * `$return_type` - The return type for when (e.g., ArcConditionalMutator)
/// * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
/// * `$mutator_trait` - Mutator trait name (e.g., Mutator, MutatorOnce)
/// * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Mutator Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$mutator_trait` | `$extra_bounds` |
/// |--------------|-----------------|----------------|------------------------|------------------|----------------|
/// | **ArcMutator** | `ArcMutator<T>` | ArcConditionalMutator | into_arc | Mutator | Send + Sync + 'static |
/// | **RcMutator** | `RcMutator<T>` | RcConditionalMutator | into_arc | Mutator | 'static |
/// | **ArcStatefulMutator** | `ArcStatefulMutator<T>` | ArcConditionalStatefulMutator | into_arc | StatefulMutator | Send + Sync + 'static |
/// | **RcStatefulMutator** | `RcStatefulMutator<T>` | RcConditionalStatefulMutator | into_rc | StatefulMutator | 'static |
///
/// # Usage Location
///
/// This macro should be used inside an impl block for the struct type.
///
/// # Examples
///
/// ```ignore
/// impl<T> ArcMutator<T> {
///     // Inside an impl block
///     impl_shared_mutator_methods!(
///         ArcMutator<T>,
///         ArcConditionalMutator,
///         into_arc,
///         Mutator,
///         Send + Sync + 'static
///     );
/// }
///
/// impl<T> RcMutator<T> {
///     // Inside an impl block
///     impl_shared_mutator_methods!(
///         RcMutator<T>,
///         RcConditionalMutator,
///         into_rc,
///         Mutator,
///         'static
///     );
/// }
/// ```
macro_rules! impl_shared_mutator_methods {
    // Single generic parameter
    ($struct_name:ident < $t:ident >, $return_type:ident, $predicate_conversion:ident, $mutator_trait:ident, $($extra_bounds:tt)+) => {
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
        /// let mutator = ArcMutator::new({
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
        pub fn when<P>(&self, predicate: P) -> $return_type<$t>
        where
            P: Predicate<$t> + $($extra_bounds)+,
        {
            $return_type {
                mutator: self.clone(),
                predicate: predicate.$predicate_conversion(),
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
        /// let mutator1 = ArcMutator::new({
        ///     let counter = Arc::clone(&counter1);
        ///     move |value: &mut i32| {
        ///         *value += counter.fetch_add(1, Ordering::SeqCst);
        ///     }
        /// });
        ///
        /// let mutator2 = ArcMutator::new({
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
        pub fn and_then<M>(&self, mut after: M) -> $struct_name<$t>
        where
            $t: 'static,
            M: $mutator_trait<$t> + $($extra_bounds)+,
        {
            let mut first = self.clone();
            $struct_name::new(move |t: &mut $t| {
                first.apply(t);
                after.apply(t);
            })
        }
    };
}

pub(crate) use impl_shared_mutator_methods;
