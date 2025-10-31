/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Function Conversions Macro
//!
//! Generates conversion methods for Conditional Function implementations
//!
//! This macro generates the conversion methods (`into_box`, `into_rc`, `into_fn`) for
//! conditional function types. It handles both immutable (Function) and mutable
//! (StatefulFunction) cases using the `#[allow(unused_mut)]` annotation.
//!
//! The macro works by always declaring variables as `mut`, which is necessary for
//! StatefulFunction cases, while suppressing unused_mut warnings for Function cases
//! where the mutability is not needed.
//!
//! # Parameters
//!
//! * `$box_type<$t:ident, $r:ident>` - The box-based function type (e.g., `BoxFunction<T, R>`)
//! * `$rc_type:ident` - The rc-based function type name (e.g., `RcFunction`)
//! * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
//!
//! # Usage Examples
//!
//! For Function (immutable):
//! ```ignore
//! impl<T, R> Function<T, R> for BoxConditionalFunction<T, R>
//! where
//!     T: 'static,
//!     R: 'static,
//! {
//!     fn apply(&self, input: &T) -> R {
//!         if self.predicate.test(input) {
//!             self.function.apply(input)
//!         } else {
//!             // default implementation
//!         }
//!     }
//!
//!     impl_conditional_function_conversions!(
//!         BoxFunction<T, R>,
//!         RcFunction,
//!         Fn
//!     );
//! }
//! ```
//!
//! For StatefulFunction (mutable):
//! ```ignore
//! impl<T, R> StatefulFunction<T, R> for BoxConditionalStatefulFunction<T, R>
//! where
//!     T: 'static,
//!     R: 'static,
//! {
//!     fn apply(&mut self, input: &T) -> R {
//!         if self.predicate.test(input) {
//!             self.function.apply(input)
//!         } else {
//!             // default implementation
//!         }
//!     }
//!
//!     impl_conditional_function_conversions!(
//!         BoxStatefulFunction<T, R>,
//!         RcStatefulFunction,
//!         FnMut
//!     );
//! }
//! ```
//!
//! # Implementation Details
//!
//! - Uses `#[allow(unused_mut)]` to handle Function cases where `mut` is not needed
//! - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
//!   or `FnMut` based on their internal operations
//! - The `into_fn` method uses the provided `$fn_trait` parameter to match the
//!   intended trait type
//!
//! # Author
//!
//! Haixing Hu

/// Generates conversion methods for Conditional Function implementations
///
/// This macro should be used inside an existing impl block (typically within
/// a trait implementation block). It generates individual conversion methods
/// but does not create a complete impl block itself. This macro generates the
/// conversion methods (`into_box`, `into_rc`, `into_fn`) for conditional function
/// types. It handles both immutable (Function) and mutable (StatefulFunction)
/// cases using the `#[allow(unused_mut)]` annotation.
///
/// The macro works by always declaring variables as `mut`, which is necessary for
/// StatefulFunction cases, while suppressing unused_mut warnings for Function cases
/// where the mutability is not needed.
///
/// # Parameters
///
/// * `$box_type<$t:ident, $r:ident>` - The box-based function type (e.g., `BoxFunction<T, R>`)
/// * `$rc_type:ident` - The rc-based function type name (e.g., `RcFunction`)
/// * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
///
/// # Usage Examples
///
/// For Function (immutable):
/// ```ignore
/// impl<T, R> Function<T, R> for BoxConditionalFunction<T, R>
/// where
///     T: 'static,
///     R: 'static,
/// {
///     fn apply(&self, input: &T) -> R {
///         if self.predicate.test(input) {
///             self.function.apply(input)
///         } else {
///             // default implementation
///         }
///     }
///
///     impl_conditional_function_conversions!(
///         BoxFunction<T, R>,
///         RcFunction,
///         Fn
///     );
/// }
/// ```
///
/// For StatefulFunction (mutable):
/// ```ignore
/// impl<T, R> StatefulFunction<T, R> for BoxConditionalStatefulFunction<T, R>
/// where
///     T: 'static,
///     R: 'static,
/// {
///     fn apply(&mut self, input: &T) -> R {
///         if self.predicate.test(input) {
///             self.function.apply(input)
///         } else {
///             // default implementation
///         }
///     }
///
///     impl_conditional_function_conversions!(
///         BoxStatefulFunction<T, R>,
///         RcStatefulFunction,
///         FnMut
///     );
/// }
/// ```
///
/// # Implementation Details
///
/// - Uses `#[allow(unused_mut)]` to handle Function cases where `mut` is not needed
/// - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
///   or `FnMut` based on their internal operations
/// - The `into_fn` method uses the provided `$fn_trait` parameter to match the
///   intended trait type
macro_rules! impl_conditional_function_conversions {
    // Two generic parameters - Function
    (
        $box_type:ident < $t:ident, $r:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        #[allow(unused_mut)]
        fn into_box(self) -> $box_type<$t, $r> {
            let pred = self.predicate;
            let mut function = self.function;
            $box_type::new(move |x: &$t| {
                if pred.test(x) {
                    function.apply(x)
                } else {
                    // For conditional functions, we need to provide a default
                    // This would typically be handled by the or_else method
                    // For now, we'll panic as this should be handled by the user
                    panic!("Conditional function without or_else case - use or_else() to provide alternative")
                }
            })
        }

        #[allow(unused_mut)]
        fn into_rc(self) -> $rc_type<$t, $r> {
            let pred = self.predicate.into_rc();
            let mut function = self.function.into_rc();
            $rc_type::new(move |x: &$t| {
                if pred.test(x) {
                    function.apply(x)
                } else {
                    // For conditional functions, we need to provide a default
                    // This would typically be handled by the or_else method
                    panic!("Conditional function without or_else case - use or_else() to provide alternative")
                }
            })
        }

        #[allow(unused_mut)]
        fn into_fn(self) -> impl $fn_trait(&$t) -> $r
        {
            let pred = self.predicate;
            let mut function = self.function;
            move |x: &$t| {
                if pred.test(x) {
                    function.apply(x)
                } else {
                    // For conditional functions, we need to provide a default
                    panic!("Conditional function without or_else case - use or_else() to provide alternative")
                }
            }
        }
    };

    // Three generic parameters - BiFunction
    (
        $box_type:ident < $t:ident, $u:ident, $r:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        #[allow(unused_mut)]
        fn into_box(self) -> $box_type<$t, $u, $r> {
            let pred = self.predicate;
            let mut function = self.function;
            $box_type::new(move |x: &$t, y: &$u| {
                if pred.test(x, y) {
                    function.apply(x, y)
                } else {
                    // For conditional functions, we need to provide a default
                    panic!("Conditional function without or_else case - use or_else() to provide alternative")
                }
            })
        }

        #[allow(unused_mut)]
        fn into_rc(self) -> $rc_type<$t, $u, $r> {
            let pred = self.predicate.into_rc();
            let mut function = self.function.into_rc();
            $rc_type::new(move |x: &$t, y: &$u| {
                if pred.test(x, y) {
                    function.apply(x, y)
                } else {
                    // For conditional functions, we need to provide a default
                    panic!("Conditional function without or_else case - use or_else() to provide alternative")
                }
            })
        }

        #[allow(unused_mut)]
        fn into_fn(self) -> impl $fn_trait(&$t, &$u) -> $r {
            let pred = self.predicate;
            let mut function = self.function;
            move |x: &$t, y: &$u| {
                if pred.test(x, y) {
                    function.apply(x, y)
                } else {
                    // For conditional functions, we need to provide a default
                    panic!("Conditional function without or_else case - use or_else() to provide alternative")
                }
            }
        }
    };
}

pub(crate) use impl_conditional_function_conversions;
