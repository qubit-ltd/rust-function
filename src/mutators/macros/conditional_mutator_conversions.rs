/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Mutator Conversions Macro
//!
//! Generates conversion methods for Conditional Mutator implementations
//!
//! This macro generates the conversion methods (`into_box`, `into_rc`, `into_fn`) for
//! conditional mutator types. It handles both immutable (Mutator) and mutable
//! (StatefulMutator) cases using the `#[allow(unused_mut)]` annotation.
//!
//! The macro works by always declaring variables as `mut`, which is necessary for
//! StatefulMutator cases, while suppressing unused_mut warnings for Mutator cases
//! where the mutability is not needed.
//!
//! # Parameters
//!
//! * `$box_type<$t:ident>` - The box-based mutator type (e.g., `BoxMutator<T>`)
//! * `$rc_type:ident` - The rc-based mutator type name (e.g., `RcMutator`)
//! * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
//!
//! # Usage Examples
//!
//! For Mutator (immutable):
//! ```ignore
//! impl<T> Mutator<T> for BoxConditionalMutator<T>
//! where
//!     T: 'static,
//! {
//!     fn apply(&self, value: &mut T) {
//!         if self.predicate.test(value) {
//!             self.mutator.apply(value);
//!         }
//!     }
//!
//!     impl_conditional_mutator_conversions!(
//!         BoxMutator<T>,
//!         RcMutator,
//!         Fn
//!     );
//! }
//! ```
//!
//! For StatefulMutator (mutable):
//! ```ignore
//! impl<T> StatefulMutator<T> for BoxConditionalStatefulMutator<T>
//! where
//!     T: 'static,
//! {
//!     fn apply(&mut self, value: &mut T) {
//!         if self.predicate.test(value) {
//!             self.mutator.apply(value);
//!         }
//!     }
//!
//!     impl_conditional_mutator_conversions!(
//!         BoxStatefulMutator<T>,
//!         RcStatefulMutator,
//!         FnMut
//!     );
//! }
//! ```
//!
//! # Implementation Details
//!
//! - Uses `#[allow(unused_mut)]` to handle Mutator cases where `mut` is not needed
//! - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
//!   or `FnMut` based on their internal operations
//! - The `into_fn` method uses the provided `$fn_trait` parameter to match the
//!   intended trait type
//!
//! # Author
//!
//! Haixing Hu

/// Generates conversion methods for Conditional Mutator implementations
///
/// This macro should be used inside an impl block to generate the conversion
/// methods (`into_box`, `into_rc`, `into_fn`) for conditional mutator types.
/// It handles both immutable (Mutator) and mutable (StatefulMutator) cases using
/// the `#[allow(unused_mut)]` annotation.
///
/// The macro works by always declaring variables as `mut`, which is necessary for
/// StatefulMutator cases, while suppressing unused_mut warnings for Mutator cases
/// where the mutability is not needed.
///
/// # Parameters
///
/// * `$box_type<$t:ident>` - The box-based mutator type (e.g., `BoxMutator<T>`)
/// * `$rc_type:ident` - The rc-based mutator type name (e.g., `RcMutator`)
/// * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
///
/// # Usage Location
///
/// This macro should be used inside an impl block for the conditional mutator
/// type, typically within a trait implementation.
///
/// # Usage Examples
///
/// For Mutator (immutable):
/// ```ignore
/// impl<T> Mutator<T> for BoxConditionalMutator<T>
/// where
///     T: 'static,
/// {
///     fn apply(&self, value: &mut T) {
///         if self.predicate.test(value) {
///             self.mutator.apply(value);
///         }
///     }
///
///     // Inside the trait impl block
///     impl_conditional_mutator_conversions!(
///         BoxMutator<T>,
///         RcMutator,
///         Fn
///     );
/// }
/// ```
///
/// For StatefulMutator (mutable):
/// ```ignore
/// impl<T> StatefulMutator<T> for BoxConditionalStatefulMutator<T>
/// where
///     T: 'static,
/// {
///     fn apply(&mut self, value: &mut T) {
///         if self.predicate.test(value) {
///             self.mutator.apply(value);
///         }
///     }
///
///     impl_conditional_mutator_conversions!(
///         BoxStatefulMutator<T>,
///         RcStatefulMutator,
///         FnMut
///     );
/// }
/// ```
///
/// # Implementation Details
///
/// - Uses `#[allow(unused_mut)]` to handle Mutator cases where `mut` is not needed
/// - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
///   or `FnMut` based on their internal operations
/// - The `into_fn` method uses the provided `$fn_trait` parameter to match the
///   intended trait type
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_mutator_conversions {
    // Single generic parameter - Mutator
    (
        $box_type:ident < $t:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        #[allow(unused_mut)]
        fn into_box(self) -> $box_type<$t> {
            let pred = self.predicate;
            let mut mutator = self.mutator;
            $box_type::new(move |t| {
                if pred.test(t) {
                    mutator.apply(t);
                }
            })
        }

        #[allow(unused_mut)]
        fn into_rc(self) -> $rc_type<$t> {
            let pred = self.predicate.into_rc();
            let mut mutator = self.mutator.into_rc();
            let mut mutator_fn = mutator;
            $rc_type::new(move |t| {
                if pred.test(t) {
                    mutator_fn.apply(t);
                }
            })
        }

        #[allow(unused_mut)]
        fn into_fn(self) -> impl $fn_trait(&mut $t) {
            let pred = self.predicate;
            let mut mutator = self.mutator;
            move |t: &mut $t| {
                if pred.test(t) {
                    mutator.apply(t);
                }
            }
        }
    };
}

pub(crate) use impl_conditional_mutator_conversions;
