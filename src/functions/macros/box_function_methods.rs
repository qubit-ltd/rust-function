/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Box Function Methods Macro
//!
//! Generates when and and_then method implementations for Box-based Function
//!
//! Generates conditional execution when method and chaining and_then method
//! for Box-based functions that consume self (because Box cannot be cloned).
//!
//! This macro supports both single-parameter and two-parameter functions through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxFunction<T, R>`
//!   - Two parameters: `BoxBiFunction<T, U, R>`
//! * `$conditional_type` - The conditional function type for when (e.g.,
//!   BoxConditionalFunction)
//! * `$chained_function_trait` - The name of the function trait that chained
//!   after the execution of this function (e.g., Function, BiFunction)
//!
//! # Parameter Usage Comparison
//!
//! | Function Type | Struct Signature | `$conditional_type` | `$function_trait` |
//! |---------------|-----------------|----------------|------------------|
//! | **Function** | `BoxFunction<T, R>` | BoxConditionalFunction | Function |
//! | **FunctionOnce** | `BoxFunctionOnce<T, R>` | BoxConditionalFunctionOnce | FunctionOnce |
//! | **StatefulFunction** | `BoxStatefulFunction<T, R>` | BoxConditionalStatefulFunction | StatefulFunction |
//! | **BiFunction** | `BoxBiFunction<T, U, R>` | BoxConditionalBiFunction | BiFunction |
//! | **BiFunctionOnce** | `BoxBiFunctionOnce<T, U, R>` | BoxConditionalBiFunctionOnce | BiFunctionOnce |
//! | **StatefulBiFunction** | `BoxStatefulBiFunction<T, U, R>` | BoxConditionalStatefulBiFunction | StatefulBiFunction |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter function
//! impl_box_function_methods!(
//!     BoxFunction<T, R>,
//!     BoxConditionalFunction,
//!     Function
//! );
//!
//! // Two-parameter function
//! impl_box_function_methods!(
//!     BoxBiFunction<T, U, R>,
//!     BoxConditionalBiFunction,
//!     BiFunction
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Box-based Function
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// Generates conditional execution when method and chaining and_then method
/// for Box-based functions that consume self (because Box cannot be cloned).
///
/// This macro supports both single-parameter and two-parameter functions through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxFunction<T, R>`
///   - Two parameters: `BoxBiFunction<T, U, R>`
/// * `$conditional_type` - The conditional function type for when (e.g.,
///   BoxConditionalFunction)
/// * `$chained_function_trait` - The name of the function trait that chained
///   after the execution of this function (e.g., Function, BiFunction)
///
/// # Parameter Usage Comparison
///
/// | Function Type | Struct Signature | `$conditional_type` | `$function_trait` |
/// |---------------|-----------------|----------------|------------------|
/// | **Function** | `BoxFunction<T, R>` | BoxConditionalFunction | Function |
/// | **FunctionOnce** | `BoxFunctionOnce<T, R>` | BoxConditionalFunctionOnce | FunctionOnce |
/// | **StatefulFunction** | `BoxStatefulFunction<T, R>` | BoxConditionalStatefulFunction | StatefulFunction |
/// | **BiFunction** | `BoxBiFunction<T, U, R>` | BoxConditionalBiFunction | Function |
/// | **BiFunctionOnce** | `BoxBiFunctionOnce<T, U, R>` | BoxConditionalBiFunctionOnce | FunctionOnce |
/// | **StatefulBiFunction** | `BoxStatefulBiFunction<T, U, R>` | BoxConditionalStatefulBiFunction | StatefulFunction |
///
/// # Examples
///
/// ```ignore
/// // Single-parameter function
/// impl_box_function_methods!(
///     BoxFunction<T, R>,
///     BoxConditionalFunction,
///     Function
/// );
///
/// // Two-parameter function
/// impl_box_function_methods!(
///     BoxBiFunction<T, U, R>,
///     BoxConditionalBiFunction,
///     BiFunction
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_function_methods {
    // Two generic parameters - Function
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $conditional_type:ident,
        $chained_function_trait:ident
    ) => {
        /// Creates a conditional function that executes based on predicate
        /// result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the function operation
        ///
        /// # Returns
        ///
        /// Returns a conditional function that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::{BoxFunction, Function};
        ///
        /// let double = BoxFunction::new(|x: i32| x * 2);
        /// let conditional = double.when(|value: &i32| *value > 0);
        /// assert_eq!(conditional.or_else(|_| 0).apply(5), 10);  // executed
        /// assert_eq!(conditional.or_else(|_| 0).apply(-3), 0);  // not executed
        /// ```
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $r>
        where
            P: Predicate<$t> + 'static,
        {
            $conditional_type {
                function: self,
                predicate: predicate.into_box(),
            }
        }

        /// Chains execution with another function, executing the current
        /// function first, then the subsequent function.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent function to execute after the current
        ///   function completes
        ///
        /// # Returns
        ///
        /// Returns a new function that executes the current function and
        /// the subsequent function in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::{BoxFunction, Function};
        ///
        /// let double = BoxFunction::new(|x: i32| x * 2);
        /// let to_string = BoxFunction::new(|x: i32| x.to_string());
        ///
        /// let chained = double.and_then(to_string);
        /// assert_eq!(chained.apply(5), "10".to_string());
        /// ```
        #[allow(unused_mut)]
        pub fn and_then<S, F>(self, mut after: F) -> $struct_name<$t, S>
        where
            S: 'static,
            F: $chained_function_trait<$r, S> + 'static,
        {
            let mut before = self.function;
            $struct_name::new(move |t| {
                let r = before(t);
                after.apply(&r)
            })
        }
    };

    // Three generic parameters - BiFunction
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $conditional_type:ident,
        $chained_function_trait:ident
    ) => {
        /// Creates a conditional two-parameter function that executes based
        /// on bi-predicate result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The bi-predicate to determine whether to execute
        ///   the function operation
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter function that only executes
        /// when the predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::{BoxBiFunction, BiFunction};
        ///
        /// let add = BoxBiFunction::new(|x: i32, y: i32| x + y);
        /// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        /// assert_eq!(conditional.or_else(|_, _| 0).apply(2, 3), 5);  // executed
        /// assert_eq!(conditional.or_else(|_, _| 0).apply(-1, 3), 0); // not executed
        /// ```
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $u, $r>
        where
            P: BiPredicate<$t, $u> + 'static,
        {
            $conditional_type {
                function: self,
                predicate: predicate.into_box(),
            }
        }

        /// Chains execution with another two-parameter function, executing
        /// the current function first, then the subsequent function.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent one-parameter function to execute after
        ///   the current function completes
        ///
        /// # Returns
        ///
        /// Returns a new two-parameter function that executes the current
        /// function and the subsequent function in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::{BoxBiFunction, BoxFunction};
        ///
        /// let add = BoxBiFunction::new(|x: i32, y: i32| x + y);
        /// let multiply_by_two = BoxFunction::new(|z: i32| z * 2);
        ///
        /// let chained = add.and_then(multiply_by_two);
        /// assert_eq!(chained.apply(2, 3), 10); // (2+3) * 2 = 10
        /// ```
        #[allow(unused_mut)]
        pub fn and_then<S, F>(self, mut after: F) -> $struct_name<$t, $u, S>
        where
            S: 'static,
            F: $chained_function_trait<$r, S> + 'static,
        {
            let mut before = self.function;
            $struct_name::new(move |t, u| {
                let mut r = before(t, u);
                after.apply(&mut r)
            })
        }
    };
}

pub(crate) use impl_box_function_methods;
