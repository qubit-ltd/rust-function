/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Transformer Conversions Macro
//!
//! Generates conversion methods for Conditional Transformer implementations
//!
//! This macro generates the conversion methods (`into_box`, `into_rc`, `into_fn`) for
//! conditional transformer types. It handles both immutable (Transformer) and mutable
//! (StatefulTransformer) cases using the `#[allow(unused_mut)]` annotation.
//!
//! The macro works by always declaring variables as `mut`, which is necessary for
//! StatefulTransformer cases, while suppressing unused_mut warnings for Transformer cases
//! where the mutability is not needed.
//!
//! # Parameters
//!
//! * `$box_type<$t:ident, $u:ident>` - The box-based transformer type (e.g., `BoxTransformer<T, U>`)
//! * `$rc_type:ident` - The rc-based transformer type name (e.g., `RcTransformer`)
//! * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
//!
//! # Usage Examples
//!
//! For Transformer (immutable):
//! ```ignore
//! impl<T, U> Transformer<T, U> for BoxConditionalTransformer<T, U>
//! where
//!     T: 'static,
//!     U: 'static,
//! {
//!     fn transform(&self, value: &T) -> U {
//!         if self.predicate.test(value) {
//!             self.transformer.transform(value)
//!         } else {
//!             // return identity/default value
//!             todo!()
//!         }
//!     }
//!
//!     impl_conditional_transformer_conversions!(
//!         BoxTransformer<T, U>,
//!         RcTransformer,
//!         Fn
//!     );
//! }
//! ```
//!
//! For StatefulTransformer (mutable):
//! ```ignore
//! impl<T, U> StatefulTransformer<T, U> for BoxConditionalStatefulTransformer<T, U>
//! where
//!     T: 'static,
//!     U: 'static,
//! {
//!     fn transform(&mut self, value: &T) -> U {
//!         if self.predicate.test(value) {
//!             self.transformer.transform(value)
//!         } else {
//!             // return identity/default value
//!             todo!()
//!         }
//!     }
//!
//!     impl_conditional_transformer_conversions!(
//!         BoxStatefulTransformer<T, U>,
//!         RcStatefulTransformer,
//!         FnMut
//!     );
//! }
//! ```
//!
//! # Implementation Details
//!
//! - Uses `#[allow(unused_mut)]` to handle Transformer cases where `mut` is not needed
//! - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
//!   or `FnMut` based on their internal operations
//! - The `into_fn` method uses the provided `$fn_trait` parameter to match the
//!   intended trait type
//!
//! # Author
//!
//! Haixing Hu

/// Generates conversion methods for Conditional Transformer implementations
///
/// This macro should be used inside an existing impl block (typically within
/// a trait implementation block). It generates individual conversion methods
/// but does not create a complete impl block itself. This macro generates the
/// conversion methods (`into_box`, `into_rc`, `into_fn`) for conditional transformer
/// types. It handles both immutable (Transformer) and mutable (StatefulTransformer)
/// cases using the `#[allow(unused_mut)]` annotation.
///
/// The macro works by always declaring variables as `mut`, which is necessary for
/// StatefulTransformer cases, while suppressing unused_mut warnings for Transformer cases
/// where the mutability is not needed.
///
/// # Parameters
///
/// * `$box_type<$t:ident, $u:ident>` - The box-based transformer type (e.g., `BoxTransformer<T, U>`)
/// * `$rc_type:ident` - The rc-based transformer type name (e.g., `RcTransformer`)
/// * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
///
/// # Usage Examples
///
/// For Transformer (immutable):
/// ```ignore
/// impl<T, U> Transformer<T, U> for BoxConditionalTransformer<T, U>
/// where
///     T: 'static,
///     U: 'static,
/// {
///     fn transform(&self, value: &T) -> U {
///         if self.predicate.test(value) {
///             self.transformer.transform(value)
///         } else {
///             // return identity/default value
///             todo!()
///         }
///     }
///
///     impl_conditional_transformer_conversions!(
///         BoxTransformer<T, U>,
///         RcTransformer,
///         Fn
///     );
/// }
/// ```
///
/// For StatefulTransformer (mutable):
/// ```ignore
/// impl<T, U> StatefulTransformer<T, U> for BoxConditionalStatefulTransformer<T, U>
/// where
///     T: 'static,
///     U: 'static,
/// {
///     fn transform(&mut self, value: &T) -> U {
///         if self.predicate.test(value) {
///             self.transformer.transform(value)
///         } else {
///             // return identity/default value
///             todo!()
///         }
///     }
///
///     impl_conditional_transformer_conversions!(
///         BoxStatefulTransformer<T, U>,
///         RcStatefulTransformer,
///         FnMut
///     );
/// }
/// ```
///
/// # Implementation Details
///
/// - Uses `#[allow(unused_mut)]` to handle Transformer cases where `mut` is not needed
/// - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
///   or `FnMut` based on their internal operations
/// - The `into_fn` method uses the provided `$fn_trait` parameter to match the
///   intended trait type
macro_rules! impl_conditional_transformer_conversions {
    // Two generic parameters - Transformer
    (
        $box_type:ident < $t:ident, $u:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        #[allow(unused_mut)]
        fn into_box(self) -> $box_type<$t, $u> {
            let pred = self.predicate;
            let mut transformer = self.transformer;
            $box_type::new(move |t| {
                if pred.test(t) {
                    transformer.transform(t)
                } else {
                    // Return identity transformation - this would need to be handled
                    // by the specific implementation, but for the macro we use a placeholder
                    panic!("Identity transformation not implemented for conditional transformer")
                }
            })
        }

        #[allow(unused_mut)]
        fn into_rc(self) -> $rc_type<$t, $u> {
            let pred = self.predicate.into_rc();
            let mut transformer = self.transformer.into_rc();
            let mut transformer_fn = transformer;
            $rc_type::new(move |t| {
                if pred.test(t) {
                    transformer_fn.transform(t)
                } else {
                    // Return identity transformation - this would need to be handled
                    // by the specific implementation, but for the macro we use a placeholder
                    panic!("Identity transformation not implemented for conditional transformer")
                }
            })
        }

        #[allow(unused_mut)]
        fn into_fn(self) -> impl $fn_trait(&$t) -> $u {
            let pred = self.predicate;
            let mut transformer = self.transformer;
            move |t: &$t| {
                if pred.test(t) {
                    transformer.transform(t)
                } else {
                    // Return identity transformation - this would need to be handled
                    // by the specific implementation, but for the macro we use a placeholder
                    panic!("Identity transformation not implemented for conditional transformer")
                }
            }
        }
    };

    // Three generic parameters - BiTransformer
    (
        $box_type:ident < $t:ident, $u:ident, $v:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        #[allow(unused_mut)]
        fn into_box(self) -> $box_type<$t, $u, $v> {
            let pred = self.predicate;
            let mut transformer = self.transformer;
            $box_type::new(move |t, u| {
                if pred.test(t, u) {
                    transformer.transform(t, u)
                } else {
                    // Return identity transformation - this would need to be handled
                    // by the specific implementation, but for the macro we use a placeholder
                    panic!("Identity transformation not implemented for conditional transformer")
                }
            })
        }

        #[allow(unused_mut)]
        fn into_rc(self) -> $rc_type<$t, $u, $v> {
            let pred = self.predicate.into_rc();
            let mut transformer = self.transformer.into_rc();
            let mut transformer_fn = transformer;
            $rc_type::new(move |t, u| {
                if pred.test(t, u) {
                    transformer_fn.transform(t, u)
                } else {
                    // Return identity transformation - this would need to be handled
                    // by the specific implementation, but for the macro we use a placeholder
                    panic!("Identity transformation not implemented for conditional transformer")
                }
            })
        }

        #[allow(unused_mut)]
        fn into_fn(self) -> impl $fn_trait(&$t, &$u) -> $v {
            let pred = self.predicate;
            let mut transformer = self.transformer;
            move |t: &$t, u: &$u| {
                if pred.test(t, u) {
                    transformer.transform(t, u)
                } else {
                    // Return identity transformation - this would need to be handled
                    // by the specific implementation, but for the macro we use a placeholder
                    panic!("Identity transformation not implemented for conditional transformer")
                }
            }
        }
    };
}

pub(crate) use impl_conditional_transformer_conversions;
