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
//! * `$function_type` - Function wrapper type name
//! * `$function_trait` - Function trait name
//! * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
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
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc two-parameter Function
//! impl_shared_conditional_function!(
//!     RcConditionalFunction<T, R>,
//!     RcFunction,
//!     Function,
//!     into_rc,
//!     'static
//! );
//!
//! // Arc three-parameter BiFunction
//! impl_shared_conditional_function!(
//!     ArcConditionalBiFunction<T, U, R>,
//!     ArcBiFunction,
//!     BiFunction,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc three-parameter BiFunction
//! impl_shared_conditional_function!(
//!     RcConditionalBiFunction<T, U, R>,
//!     RcBiFunction,
//!     BiFunction,
//!     into_rc,
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
/// * `$function_type` - Function wrapper type name
/// * `$function_trait` - Function trait name
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
macro_rules! impl_shared_conditional_function {
    // Two generic parameters - Function types
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $function_type:ident,
        $function_trait:ident,
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
            pub fn or_else<F>(&self, mut else_function: F) -> $function_type<$t, $r>
            where
                F: $function_trait<$t, $r> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                let mut then_function = self.function.clone();
                $function_type::new(move |t| {
                    if predicate.test(t) {
                        then_function.apply(t)
                    } else {
                        else_function.apply(t)
                    }
                })
            }
        }
    };

    // Three generic parameters - BiFunction types
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $function_type:ident,
        $function_trait:ident,
        $($extra_bounds:tt)+
    ) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
        {
            /// Provides an alternative function for when the predicate is not satisfied
            ///
            /// Combines the current conditional bifunction with an alternative bifunction
            /// into a new bifunction that implements the following semantics:
            ///
            /// When the returned bifunction is called with two arguments:
            /// - If the predicate is satisfied, it executes the internal bifunction
            /// - If the predicate is NOT satisfied, it executes the alternative bifunction
            ///
            /// # Parameters
            ///
            /// * `else_function` - The alternative bifunction to execute when predicate fails
            ///
            /// # Returns
            ///
            /// Returns a new bifunction that handles both conditional branches
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let func = ArcBiFunction::new(|x: i32, y: i32| x + y);
            /// let alternative = ArcBiFunction::new(|x: i32, y: i32| x * y);
            ///
            /// let conditional = func.when(|x, y| *x > 0 && *y > 0).or_else(alternative);
            ///
            /// assert_eq!(conditional.apply(3, 4), 7);   // 3 + 4 = 7 (predicate satisfied)
            /// assert_eq!(conditional.apply(-2, 4), -8); // -2 * 4 = -8 (predicate failed)
            /// ```
            #[allow(unused_mut)]
            pub fn or_else<F>(&self, mut else_function: F) -> $function_type<$t, $u, $r>
            where
                F: $function_trait<$t, $u, $r> + 'static,
            {
                let predicate = self.predicate.clone();
                let mut then_function = self.function.clone();
                $function_type::new(move |t, u| {
                    if predicate.test(t, u) {
                        then_function.apply(t, u)
                    } else {
                        else_function.apply(t, u)
                    }
                })
            }
        }
    };
}

pub(crate) use impl_shared_conditional_function;
