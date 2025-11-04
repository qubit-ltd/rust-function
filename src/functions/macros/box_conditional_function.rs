/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Box Conditional Function Macro
//!
//! Generates Box-based Conditional Function implementations
//!
//! For Box-based conditional functions, generates `or_else` methods,
//! as well as complete Function/BiFunction trait implementations.
//!
//! Box type characteristics:
//! - `or_else` function self (because Box cannot Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$box_function_type` - Function wrapper type name
//! * `$else_function_trait` - The name of the else function trait (e.g., Function, BiFunction)
//!
//! # Usage Examples
//!
//! ```ignore
//! // Two-parameter Function
//! impl_box_conditional_function!(
//!     BoxConditionalFunction<T, R>,
//!     BoxFunction,
//!     Function
//! );
//!
//! // Three-parameter BiFunction
//! impl_box_conditional_function!(
//!     BoxConditionalBiFunction<T, U, R>,
//!     BoxBiFunction,
//!     BiFunction
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Box-based Conditional Function implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Box-based conditional functions, generates `or_else` methods,
/// as well as complete Function/BiFunction trait implementations.
///
/// Box type characteristics:
/// - `or_else` consume self (because Box cannot Clone)
/// - Does not implement `into_arc()` (because Box types are not Send + Sync)
/// - Does not implement `to_xxx()` methods (because Box types cannot Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$box_function_type` - Function wrapper type name
/// * `$else_function_trait` - The name of the else function trait (e.g., Function, BiFunction)
///
/// # Usage Examples
///
/// ```ignore
/// // Two-parameter Function
/// impl_box_conditional_function!(
///     BoxConditionalFunction<T, R>,
///     BoxFunction,
///     Function
/// );
///
/// // Three-parameter BiFunction
/// impl_box_conditional_function!(
///     BoxConditionalBiFunction<T, U, R>,
///     BoxBiFunction,
///     BiFunction
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_conditional_function {
    // Two generic parameters - Function
    (
        $struct_name:ident<$t:ident, $r:ident>,
        $box_function_type:ident,
        $else_function_trait:ident
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
            /// let func = BoxFunction::new(|x: i32| x * 2);
            /// let alternative = BoxFunction::new(|x: i32| x + 10);
            ///
            /// let conditional = func.when(|x| *x > 0).or_else(alternative);
            ///
            /// assert_eq!(conditional.apply(5), 10);  // 5 * 2 = 10
            /// assert_eq!(conditional.apply(-3), 7);  // -3 + 10 = 7
            /// ```
            #[allow(unused_mut)]
            pub fn or_else<F>(self, mut else_function: F) -> $box_function_type<$t, $r>
            where
                F: $else_function_trait<$t, $r> + 'static,
            {
                let predicate = self.predicate;
                let mut then_function = self.function;
                $box_function_type::new(move |t| {
                    if predicate.test(t) {
                        then_function.apply(t)
                    } else {
                        else_function.apply(t)
                    }
                })
            }
        }
    };

    // Three generic parameters - BiFunction
    (
        $struct_name:ident<$t:ident, $u:ident, $r:ident>,
        $box_function_type:ident,
        $else_function_trait:ident
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
            /// let func = BoxBiFunction::new(|x: i32, y: i32| x + y);
            /// let alternative = BoxBiFunction::new(|x: i32, y: i32| x * y);
            ///
            /// let conditional = func.when(|x, y| *x > 0 && *y > 0).or_else(alternative);
            ///
            /// assert_eq!(conditional.apply(3, 4), 7);   // 3 + 4 = 7 (predicate satisfied)
            /// assert_eq!(conditional.apply(-2, 4), -8); // -2 * 4 = -8 (predicate failed)
            /// ```
            #[allow(unused_mut)]
            pub fn or_else<F>(self, mut else_function: F) -> $box_function_type<$t, $u, $r>
            where
                F: $else_function_trait<$t, $u, $r> + 'static,
            {
                let predicate = self.predicate;
                let mut then_function = self.function;
                $box_function_type::new(move |t, u| {
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

pub(crate) use impl_box_conditional_function;
