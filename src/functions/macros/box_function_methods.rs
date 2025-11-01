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
//! * `$conditional_type` - The conditional function type for when (e.g., BoxConditionalFunction)
//! * `$function_trait` - Function trait name (e.g., Function, BiFunction)
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
/// * `$conditional_type` - The conditional function type for when (e.g., BoxConditionalFunction)
/// * `$function_trait` - Function trait name (e.g., Function, BiFunction)
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
macro_rules! impl_box_function_methods {
    // Two generic parameters - Function
    ($struct_name:ident < $t:ident, $r:ident >, $conditional_type:ident, $function_trait:ident) => {
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
            F: $function_trait<$r, S> + 'static,
        {
            let mut before = self.function;
            $struct_name::new(move |t| {
                let mut r = before(t);
                after.apply(&mut r)
            })
        }

        /// Creates a composed function that executes the provided function first,
        /// then applies this function to its result.
        ///
        /// This is the reverse of `and_then`: `before` is executed first, then `self`.
        ///
        /// # Type Parameters
        ///
        /// * `S` - The input type of the before function
        /// * `F` - The type of the before function
        ///
        /// # Parameters
        ///
        /// * `before` - The function to execute before this function
        ///
        /// # Returns
        ///
        /// Returns a new function that executes `before` first, then applies this
        /// function to the result.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::{BoxFunction, Function};
        ///
        /// let to_string = BoxFunction::new(|x: i32| x.to_string());
        /// let add_prefix = BoxFunction::new(|s: &str| format!("Value: {}", s));
        ///
        /// let composed = add_prefix.compose(to_string);
        /// assert_eq!(composed.apply(42), "Value: 42");
        /// ```
        #[allow(unused_mut)]
        pub fn compose<S, F>(self, mut before: F) -> $struct_name<S, $r>
        where
            S: 'static,
            F: $function_trait<S, $t> + 'static,
        {
            let mut after = self.function;
            $struct_name::new(move |s| {
                let mut t = before.apply(s);
                after(&mut t)
            })
        }
    };

    // Three generic parameters - BiFunction
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >, $conditional_type:ident, $function_trait:ident) => {
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
            F: $function_trait<$r, S> + 'static,
        {
            let mut before = self.function;
            $struct_name::new(move |t, u| {
                let mut r = before(t, u);
                after.apply(&mut r)
            })
        }

        /// Creates a composed function that executes the provided function first,
        /// then applies this function to its result along with the second parameter.
        ///
        /// This is the reverse of `and_then`: `before` is executed first, then `self`
        /// is applied with the result of `before` as the first parameter and the
        /// original second parameter.
        ///
        /// # Type Parameters
        ///
        /// * `S` - The input type of the before function
        /// * `F` - The type of the before function
        ///
        /// # Parameters
        ///
        /// * `before1` - The first two parameters function to execute before this function
        /// * `before2` - The second two parameter function to execute before this function
        ///
        /// # Returns
        ///
        /// Returns a new function that executes `before1` and `before2` first,
        /// then applies this function with the result of `before1` and `before2`
        /// as the first parameter and the second parameter.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::BoxBiFunction;
        ///
        /// let add = BoxBiFunction::new(|x: i32, y: i32| x + y);
        /// let sub = BoxBiFunction::new(|x: i32, y: i32| x - y);
        /// let multiply = BoxBiFunction::new(|x: i32, y: i32| x * y);
        ///
        /// let composed = multiply.compose(add, sub);
        /// assert_eq!(composed.apply(4, 2), 12); // (4 + 2) * (4 - 2) = 12
        /// ```
        #[allow(unused_mut)]
        pub fn compose<S1, S2, F1, F2>(
            self,
            mut before1: F1,
            mut before2: F2,
        ) -> $struct_name<S1, S2, $r>
        where
            S1: 'static,
            S2: 'static,
            F1: $function_trait<S1, $t> + 'static,
            F2: $function_trait<S2, $u> + 'static,
        {
            let mut after = self.function;
            $struct_name::new(move |s1, s2| {
                let mut t = before1.apply(s1);
                let mut u = before2.apply(s2);
                after(&mut t, &mut u)
            })
        }
    };
}

pub(crate) use impl_box_function_methods;
