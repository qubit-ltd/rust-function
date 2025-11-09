/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Shared Conditional Function Macro
//!
//! Generates Arc/Rc-based Conditional Function implementations
//!
//! For Arc/Rc-based conditional functions, generates `or_else` methods,
//! as well as complete Function/BiFunction trait implementations.
//!
//! Arc/Rc type characteristics:
//! - `or_else` borrow &self (because Arc/Rc can Clone)
//! - Uses trait default implementations for `into_arc()` and `to_arc()`
//! - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
//! - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
//! - Implement complete `to_xxx()` methods (because they can Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$shared_function_type` - Function wrapper type name
//! * `$else_function_trait` - The name of the else function trait (e.g., Function, BiFunction)
//! * `$extra_bounds` - Extra trait bounds
//!
//! # Usage Examples
//!
//! ```ignore
//! // Arc two-parameter Function
//! impl_shared_conditional_function!(
//!     ArcConditionalFunction<T, R>,
//!     ArcFunction,
//!     Function,
//!     Send + Sync + 'static
//! );
//!
//! // Rc two-parameter Function
//! impl_shared_conditional_function!(
//!     RcConditionalFunction<T, R>,
//!     RcFunction,
//!     Function,
//!     'static
//! );
//!
//! // Arc three-parameter BiFunction
//! impl_shared_conditional_function!(
//!     ArcConditionalBiFunction<T, U, R>,
//!     ArcBiFunction,
//!     BiFunction,
//!     Send + Sync + 'static
//! );
//!
//! // Rc three-parameter BiFunction
//! impl_shared_conditional_function!(
//!     RcConditionalBiFunction<T, U, R>,
//!     RcBiFunction,
//!     BiFunction,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Arc/Rc-based Conditional Function implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Arc/Rc-based conditional functions, generates `or_else` methods,
/// as well as complete Function/BiFunction trait implementations.
///
/// Arc/Rc type characteristics:
/// - `or_else` borrow &self (because Arc/Rc can Clone)
/// - Uses trait default implementations for `into_arc()` and `to_arc()`
/// - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
/// - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
/// - Implement complete `to_xxx()` methods (because they can Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$shared_function_type` - Function wrapper type name
/// * `$else_function_trait` - The name of the else function trait (e.g., Function, BiFunction)
/// * `$extra_bounds` - Extra trait bounds
///
/// # Usage Examples
///
/// ```ignore
/// // Arc two-parameter Function
/// impl_shared_conditional_function!(
///     ArcConditionalFunction<T, R>,
///     ArcFunction,
///     Function,
///     Send + Sync + 'static
/// );
///
/// // Rc two-parameter Function
/// impl_shared_conditional_function!(
///     RcConditionalFunction<T, R>,
///     RcFunction,
///     Function,
///     'static
/// );
///
/// // Arc three-parameter BiFunction
/// impl_shared_conditional_function!(
///     ArcConditionalBiFunction<T, U, R>,
///     ArcBiFunction,
///     BiFunction,
///     Send + Sync + 'static
/// );
///
/// // Rc three-parameter BiFunction
/// impl_shared_conditional_function!(
///     RcConditionalBiFunction<T, U, R>,
///     RcBiFunction,
///     BiFunction,
///     'static
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_shared_conditional_function {
    // Two generic parameters - Function types
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $shared_function_type:ident,
        $else_function_trait:ident,
        $($extra_bounds:tt)+
    ) => {
        impl<$t, $r> $struct_name<$t, $r>
        where
            $t: 'static,
            $r: 'static,
        {
            /// Provides an alternative function for when the predicate is not satisfied
            ///
            /// Combines the current conditional function with an alternative function
            /// into a new function that implements the following semantics:
            ///
            /// When the returned function is called with an argument:
            /// - If the predicate is satisfied, it executes the internal function
            /// - If the predicate is NOT satisfied, it executes the alternative function
            ///
            /// # Parameters
            ///
            /// * `else_function` - The alternative function to execute when predicate fails
            ///
            /// # Returns
            ///
            /// Returns a new function that handles both conditional branches
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let func = ArcFunction::new(|x: i32| x * 2);
            /// let alternative = ArcFunction::new(|x: i32| x + 10);
            ///
            /// let conditional = func.when(|x| *x > 0).or_else(alternative);
            ///
            /// assert_eq!(conditional.apply(5), 10);  // 5 * 2 = 10
            /// assert_eq!(conditional.apply(-3), 7);  // -3 + 10 = 7
            /// ```
            #[allow(unused_mut)]
            pub fn or_else<F>(&self, mut else_function: F) -> $shared_function_type<$t, $r>
            where
                F: $else_function_trait<$t, $r> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                let mut then_function = self.function.clone();
                $shared_function_type::new(move |t| {
                    if predicate.test(t) {
                        then_function.apply(t)
                    } else {
                        else_function.apply(t)
                    }
                })
            }
        }
    };

    // Three generic parameters - BiFunction types (Rc version)
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $shared_function_type:ident,
        $else_function_trait:ident,
        into_rc,
        'static
    ) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
        {
            /// Provides an alternative bi-function for when the predicate is not satisfied
            ///
            /// Combines the current conditional bi-function with an alternative bi-function
            /// into a new bi-function that implements the following semantics:
            ///
            /// When the returned bi-function is called with arguments:
            /// - If the predicate is satisfied, it executes the internal bi-function
            /// - If the predicate is NOT satisfied, it executes the alternative bi-function
            ///
            /// # Parameters
            ///
            /// * `else_function` - The alternative bi-function to execute when predicate fails
            ///
            /// # Returns
            ///
            /// Returns a new bi-function that handles both conditional branches
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let func = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
            /// let alternative = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
            ///
            /// let conditional = func.when(|x, y| *x > 0 && *y > 0).or_else(alternative);
            ///
            /// assert_eq!(conditional.apply(3, 4), 7);   // 3 + 4 = 7
            /// assert_eq!(conditional.apply(-2, 4), -8); // -2 * 4 = -8
            /// ```
            #[allow(unused_mut)]
            pub fn or_else<F>(&self, mut else_function: F) -> $shared_function_type<$t, $u, $r>
            where
                F: $else_function_trait<$t, $u, $r> + 'static,
            {
                let predicate = self.predicate.clone();
                let mut then_function = self.function.clone();
                $shared_function_type::new(move |t, u| {
                    if predicate.test(t, u) {
                        then_function.apply(t, u)
                    } else {
                        else_function.apply(t, u)
                    }
                })
            }
        }

        impl<$t, $u, $r> $else_function_trait<$t, $u, $r> for $struct_name<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
        {
            fn apply(&self, first: &$t, second: &$u) -> $r {
                if self.predicate.test(first, second) {
                    self.function.apply(first, second)
                } else {
                    // This should not happen - conditional functions should always have an else
                    // via or_else(), but we need to return something
                    panic!("Conditional bi-function called without or_else() alternative")
                }
            }

            // Use trait default implementations for conversion methods
        }
    };

    // Three generic parameters - BiFunction types (Arc version)
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $shared_function_type:ident,
        $else_function_trait:ident,
        into_arc,
        Send + Sync + 'static
    ) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
        {
            /// Provides an alternative bi-function for when the predicate is not satisfied
            ///
            /// Combines the current conditional bi-function with an alternative bi-function
            /// into a new bi-function that implements the following semantics:
            ///
            /// When the returned bi-function is called with arguments:
            /// - If the predicate is satisfied, it executes the internal bi-function
            /// - If the predicate is NOT satisfied, it executes the alternative bi-function
            ///
            /// # Parameters
            ///
            /// * `else_function` - The alternative bi-function to execute when predicate fails
            ///
            /// # Returns
            ///
            /// Returns a new bi-function that handles both conditional branches
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let func = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
            /// let alternative = ArcBiFunction::new(|x: &i32, y: &i32| *x * *y);
            ///
            /// let conditional = func.when(|x, y| *x > 0 && *y > 0).or_else(alternative);
            ///
            /// assert_eq!(conditional.apply(3, 4), 7);   // 3 + 4 = 7
            /// assert_eq!(conditional.apply(-2, 4), -8); // -2 * 4 = -8
            /// ```
            #[allow(unused_mut)]
            pub fn or_else<F>(&self, mut else_function: F) -> $shared_function_type<$t, $u, $r>
            where
                F: $else_function_trait<$t, $u, $r> + Send + Sync + 'static,
            {
                let predicate = self.predicate.clone();
                let mut then_function = self.function.clone();
                $shared_function_type::new(move |t, u| {
                    if predicate.test(t, u) {
                        then_function.apply(t, u)
                    } else {
                        else_function.apply(t, u)
                    }
                })
            }
        }

        impl<$t, $u, $r> $else_function_trait<$t, $u, $r> for $struct_name<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
        {
            fn apply(&self, first: &$t, second: &$u) -> $r {
                if self.predicate.test(first, second) {
                    self.function.apply(first, second)
                } else {
                    // This should not happen - conditional functions should always have an else
                    // via or_else(), but we need to return something
                    panic!("Conditional bi-function called without or_else() alternative")
                }
            }

            // Use trait default implementations for conversion methods
        }
    };

}

pub(crate) use impl_shared_conditional_function;
