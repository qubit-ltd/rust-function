/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Shared Function Methods Macro
//!
//! Generates when and and_then method implementations for Arc/Rc-based Function
//!
//! Generates conditional execution when method and chaining and_then method
//! for Arc/Rc-based functions that borrow &self (because Arc/Rc can be cloned).
//!
//! This macro supports both single-parameter and two-parameter functions through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `ArcFunction<T, R>`
//!   - Two parameters: `ArcBiFunction<T, U, R>`
//! * `$return_type` - The return type for when (e.g., ArcConditionalFunction)
//! * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
//! * `$function_trait` - Function trait name (e.g., Function, BiFunction)
//! * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Function Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$function_trait` | `$extra_bounds` |
//! |---------------|-----------------|----------------|------------------------|------------------|----------------|
//! | **ArcFunction** | `ArcFunction<T, R>` | ArcConditionalFunction | into_arc | Function | Send + Sync + 'static |
//! | **RcFunction** | `RcFunction<T, R>` | RcConditionalFunction | into_rc | Function | 'static |
//! | **ArcStatefulFunction** | `ArcStatefulFunction<T, R>` | ArcConditionalStatefulFunction | into_arc | StatefulFunction | Send + Sync + 'static |
//! | **RcStatefulFunction** | `RcStatefulFunction<T, R>` | RcConditionalStatefulFunction | into_rc | StatefulFunction | 'static |
//! | **ArcBiFunction** | `ArcBiFunction<T, U, R>` | ArcConditionalBiFunction | into_arc | BiFunction | Send + Sync + 'static |
//! | **RcBiFunction** | `RcBiFunction<T, U, R>` | RcConditionalBiFunction | into_rc | BiFunction | 'static |
//! | **ArcStatefulBiFunction** | `ArcStatefulBiFunction<T, U, R>` | ArcConditionalStatefulBiFunction | into_arc | StatefulBiFunction | Send + Sync + 'static |
//! | **RcStatefulBiFunction** | `RcStatefulBiFunction<T, U, R>` | RcConditionalStatefulBiFunction | into_rc | StatefulBiFunction | 'static |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter with Arc
//! impl_shared_function_methods!(
//!     ArcFunction<T, R>,
//!     ArcConditionalFunction,
//!     into_arc,
//!     Function,
//!     Send + Sync + 'static
//! );
//!
//! // Two-parameter with Rc
//! impl_shared_function_methods!(
//!     RcBiFunction<T, U, R>,
//!     RcConditionalBiFunction,
//!     into_rc,
//!     BiFunction,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Arc/Rc-based Function
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates conditional execution when method and chaining
/// and_then method for Arc/Rc-based functions that borrow &self (because Arc/Rc
/// can be cloned).
///
/// This macro supports both single-parameter and two-parameter functions through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `ArcFunction<T, R>`
///   - Two parameters: `ArcBiFunction<T, U, R>`
/// * `$return_type` - The return type for when (e.g., ArcConditionalFunction)
/// * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
/// * `$function_trait` - Function trait name (e.g., Function, StatefulFunction)
/// * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Function Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$function_trait` | `$extra_bounds` |
/// |---------------|-----------------|----------------|------------------------|------------------|----------------|
/// | **ArcFunction** | `ArcFunction<T, R>` | ArcConditionalFunction | into_arc | Function | Send + Sync + 'static |
/// | **RcFunction** | `RcFunction<T, R>` | RcConditionalFunction | into_rc | Function | 'static |
/// | **ArcStatefulFunction** | `ArcStatefulFunction<T, R>` | ArcConditionalStatefulFunction | into_arc | StatefulFunction | Send + Sync + 'static |
/// | **RcStatefulFunction** | `RcStatefulFunction<T, R>` | RcConditionalStatefulFunction | into_rc | StatefulFunction | 'static |
/// | **ArcBiFunction** | `ArcBiFunction<T, U, R>` | ArcConditionalBiFunction | into_arc | BiFunction | Send + Sync + 'static |
/// | **RcBiFunction** | `RcBiFunction<T, U, R>` | RcConditionalBiFunction | into_rc | BiFunction | 'static |
/// | **ArcStatefulBiFunction** | `ArcStatefulBiFunction<T, U, R>` | ArcConditionalStatefulBiFunction | into_arc | StatefulBiFunction | Send + Sync + 'static |
/// | **RcStatefulBiFunction** | `RcStatefulBiFunction<T, U, R>` | RcConditionalStatefulBiFunction | into_rc | StatefulBiFunction | 'static |
///
/// # Examples
///
/// ```ignore
/// // Single-parameter with Arc
/// impl_shared_function_methods!(
///     ArcFunction<T, R>,
///     ArcConditionalFunction,
///     into_arc,
//!     Function,
//!     Send + Sync + 'static
//! );
//!
//! // Two-parameter with Rc
//! impl_shared_function_methods!(
//!     RcBiFunction<T, U, R>,
//!     RcConditionalBiFunction,
//!     into_rc,
//!     BiFunction,
//!     'static
//! );
//! ```
macro_rules! impl_shared_function_methods {
    // Two generic parameters - Function types
    ($struct_name:ident < $t:ident, $r:ident >, $return_type:ident, $predicate_conversion:ident, $function_trait:ident, $($extra_bounds:tt)+) => {
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
        /// use prism3_function::{ArcFunction, Function};
        /// use std::sync::Arc;
        ///
        /// let double = ArcFunction::new(|x: i32| x * 2);
        /// let conditional = double.when(|value: &i32| *value > 0);
        /// assert_eq!(conditional.or_else(|_| 0).apply(5), 10);  // executed
        /// assert_eq!(conditional.or_else(|_| 0).apply(-3), 0);  // not executed
        /// ```
        pub fn when<P>(&self, predicate: P) -> $return_type<$t, $r>
        where
            P: Predicate<$t> + $($extra_bounds)+,
        {
            $return_type {
                function: self.clone(),
                predicate: predicate.$predicate_conversion(),
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
        /// use prism3_function::{ArcFunction, Function};
        /// use std::sync::Arc;
        ///
        /// let double = ArcFunction::new(|x: i32| x * 2);
        /// let to_string = ArcFunction::new(|x: i32| x.to_string());
        ///
        /// let chained = double.and_then(to_string);
        /// assert_eq!(chained.apply(5), "10".to_string());
        /// ```
        #[allow(unused_mut)]
        pub fn and_then<S, F>(&self, mut after: F) -> $struct_name<$t, S>
        where
            $t: 'static,
            $r: 'static,
            S: 'static,
            F: $function_trait<$r, S> + $($extra_bounds)+,
        {
            let mut first = self.clone();
            $struct_name::new(move |t: &$t| {
                let intermediate = first.apply(t);
                after.apply(&intermediate)
            })
        }
    };

    // Three generic parameters - BiFunction types
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >, $return_type:ident, $predicate_conversion:ident, $function_trait:ident, $($extra_bounds:tt)+) => {
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
        /// use prism3_function::{ArcBiFunction, BiFunction};
        /// use std::sync::Arc;
        ///
        /// let add = ArcBiFunction::new(|x: i32, y: i32| x + y);
        /// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        /// assert_eq!(conditional.or_else(|_, _| 0).apply(2, 3), 5);  // executed
        /// assert_eq!(conditional.or_else(|_, _| 0).apply(-1, 3), 0); // not executed
        /// ```
        pub fn when<P>(&self, predicate: P) -> $return_type<$t, $u, $r>
        where
            P: BiPredicate<$t, $u> + $($extra_bounds)+,
        {
            $return_type {
                function: self.clone(),
                predicate: predicate.$predicate_conversion(),
            }
        }

        /// Chains execution with another two-parameter function, executing
        /// the current function first, then the subsequent function.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent two-parameter function to execute after
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
        /// use prism3_function::{ArcBiFunction, BiFunction};
        /// use std::sync::Arc;
        ///
        /// let add = ArcBiFunction::new(|x: i32, y: i32| x + y);
        /// let multiply_by_two = ArcBiFunction::new(|x: i32, y: i32| x * y * 2);
        ///
        /// let chained = add.and_then(multiply_by_two);
        /// assert_eq!(chained.apply(2, 3), 10); // (2+3) * 2 = 10
        /// ```
        #[allow(unused_mut)]
        pub fn and_then<S, F>(&self, mut after: F) -> $struct_name<$t, $u, S>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            S: 'static,
            F: $function_trait<$r, S> + $($extra_bounds)+,
        {
            let mut first = self.clone();
            $struct_name::new(move |t: &$t, u: &$u| {
                let intermediate = first.apply(t, u);
                after.apply(&intermediate)
            })
        }
    };
}

pub(crate) use impl_shared_function_methods;
