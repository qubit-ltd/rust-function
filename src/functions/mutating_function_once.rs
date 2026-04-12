/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # MutatingFunctionOnce Types
//!
//! Provides Java-like one-time `MutatingFunction` interface implementations
//! for performing operations that consume self, accept a mutable reference,
//! and return a result.
//!
//! It is similar to the `FnOnce(&mut T) -> R` trait in the standard library.
//!
//! This module provides a unified `MutatingFunctionOnce` trait and a
//! Box-based single ownership implementation:
//!
//! - **`BoxMutatingFunctionOnce<T, R>`**: Box-based single ownership
//!   implementation for one-time use scenarios
//!
//! # Design Philosophy
//!
//! The key difference between `MutatingFunctionOnce` and
//! `MutatingFunction`:
//!
//! - **MutatingFunction**: `&self`, can be called multiple times, uses
//!   `Fn(&mut T) -> R`
//! - **MutatingFunctionOnce**: `self`, can only be called once, uses
//!   `FnOnce(&mut T) -> R`
//!
//! ## MutatingFunctionOnce vs MutatingFunction
//!
//! | Feature | MutatingFunction | MutatingFunctionOnce |
//! |---------|------------------|----------------------|
//! | **Self Parameter** | `&self` | `self` |
//! | **Call Count** | Multiple | Once |
//! | **Closure Type** | `Fn(&mut T) -> R` | `FnOnce(&mut T) -> R` |
//! | **Use Cases** | Repeatable operations | One-time resource
//! transfers |
//!
//! # Why MutatingFunctionOnce?
//!
//! Core value of MutatingFunctionOnce:
//!
//! 1. **Store FnOnce closures**: Allows moving captured variables
//! 2. **Delayed execution**: Store in data structures, execute later
//! 3. **Resource transfer**: Suitable for scenarios requiring ownership
//!    transfer
//! 4. **Return results**: Unlike MutatorOnce, returns information about the
//!    operation
//!
//! # Why Only Box Variant?
//!
//! - **Arc/Rc conflicts with FnOnce semantics**: FnOnce can only be called
//!   once, while shared ownership implies multiple references
//! - **Box is perfect match**: Single ownership aligns perfectly with
//!   one-time call semantics
//!
//! # Use Cases
//!
//! ## BoxMutatingFunctionOnce
//!
//! - Post-initialization callbacks (moving data, returning status)
//! - Resource transfer with result (moving Vec, returning old value)
//! - One-time complex operations (requiring moved capture variables)
//! - Validation with fixes (fix data once, return validation result)
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! let data = vec![1, 2, 3];
//! let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
//!     let old_len = x.len();
//!     x.extend(data); // Move data
//!     old_len
//! });
//!
//! let mut target = vec![0];
//! let old_len = func.apply(&mut target);
//! assert_eq!(old_len, 1);
//! assert_eq!(target, vec![0, 1, 2, 3]);
//! ```
//!
//! ## Method Chaining
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! let data1 = vec![1, 2];
//! let data2 = vec![3, 4];
//!
//! let chained = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
//!     x.extend(data1);
//!     x.len()
//! })
//! .and_then(move |x: &mut Vec<i32>| {
//!     x.extend(data2);
//!     x.len()
//! });
//!
//! let mut target = vec![0];
//! let final_len = chained.apply(&mut target);
//! assert_eq!(final_len, 5);
//! assert_eq!(target, vec![0, 1, 2, 3, 4]);
//! ```
//!
//! ## Validation Pattern
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! struct Data {
//!     value: i32,
//! }
//!
//! let validator = BoxMutatingFunctionOnce::new(|data: &mut Data| {
//!     if data.value < 0 {
//!         data.value = 0;
//!         Err("Fixed negative value")
//!     } else {
//!         Ok("Valid")
//!     }
//! });
//!
//! let mut data = Data { value: -5 };
//! let result = validator.apply(&mut data);
//! assert_eq!(data.value, 0);
//! assert!(result.is_err());
//! ```
//!
//! # Author
//!
//! Haixing Hu
use crate::functions::{
    function_once::FunctionOnce,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_debug_display,
        impl_fn_ops_trait,
        impl_function_common_methods,
        impl_function_debug_display,
        impl_function_identity_method,
    },
};
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};

// =======================================================================
// 1. MutatingFunctionOnce Trait - One-time Function Interface
// =======================================================================

/// MutatingFunctionOnce trait - One-time mutating function interface
///
/// It is similar to the `FnOnce(&mut T) -> R` trait in the standard library.
///
/// Defines the core behavior of all one-time mutating function types.
/// Performs operations that consume self, accept a mutable reference,
/// potentially modify it, and return a result.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnOnce(&mut T) -> R`
/// - `BoxMutatingFunctionOnce<T, R>`
///
/// # Design Rationale
///
/// This trait provides a unified abstraction for one-time mutating function
/// operations. The key difference from `MutatingFunction`:
/// - `MutatingFunction` uses `&self`, can be called multiple times
/// - `MutatingFunctionOnce` uses `self`, can only be called once
///
/// # Features
///
/// - **Unified Interface**: All one-time mutating functions share the same
///   `apply` method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait with zero overhead
/// - **Type Conversions**: Provides `into_box` method for type conversion
/// - **Generic Programming**: Write functions that work with any one-time
///   mutating function type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// fn apply<F: MutatingFunctionOnce<Vec<i32>, usize>>(
///     func: F,
///     initial: Vec<i32>
/// ) -> (Vec<i32>, usize) {
///     let mut val = initial;
///     let result = func.apply(&mut val);
///     (val, result)
/// }
///
/// let data = vec![1, 2, 3];
/// let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
///     let old_len = x.len();
///     x.extend(data);
///     old_len
/// });
/// let (vec, old_len) = apply(func, vec![0]);
/// assert_eq!(vec, vec![0, 1, 2, 3]);
/// assert_eq!(old_len, 1);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::MutatingFunctionOnce;
///
/// let data = vec![1, 2, 3];
/// let closure = move |x: &mut Vec<i32>| {
///     let old_len = x.len();
///     x.extend(data);
///     old_len
/// };
/// let box_func = closure.into_box();
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait MutatingFunctionOnce<T, R> {
    /// Performs the one-time mutating function operation
    ///
    /// Consumes self and executes an operation on the given mutable
    /// reference, potentially modifying it, and returns a result. The
    /// operation can only be called once.
    ///
    /// # Parameters
    ///
    /// * `t - A mutable reference to the input value
    ///
    /// # Returns
    ///
    /// The computed result value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatingFunctionOnce,
    ///                       BoxMutatingFunctionOnce};
    ///
    /// let data = vec![1, 2, 3];
    /// let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
    ///     let old_len = x.len();
    ///     x.extend(data);
    ///     old_len
    /// });
    ///
    /// let mut target = vec![0];
    /// let old_len = func.apply(&mut target);
    /// assert_eq!(old_len, 1);
    /// assert_eq!(target, vec![0, 1, 2, 3]);
    /// ```
    fn apply(self, t: &mut T) -> R;

    /// Converts to `BoxMutatingFunctionOnce` (consuming)
    ///
    /// Consumes `self` and returns an owned `BoxMutatingFunctionOnce<T, R>`.
    /// The default implementation simply wraps the consuming
    /// `apply(self, &mut T)` call in a `Box<dyn FnOnce(&mut T) -> R>`.
    /// Types that can provide a cheaper or identity conversion (for example
    /// `BoxMutatingFunctionOnce` itself) should override this method.
    ///
    /// # Note
    ///
    /// - This method consumes the source value.
    /// - Implementors may return `self` directly when `Self` is already a
    ///   `BoxMutatingFunctionOnce<T, R>` to avoid the extra wrapper
    ///   allocation.
    fn into_box(self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxMutatingFunctionOnce::new(move |t| self.apply(t))
    }

    /// Converts to a consuming closure `FnOnce(&mut T) -> R`
    ///
    /// Consumes `self` and returns a closure that, when invoked, calls
    /// `apply(self, &mut T)`. This is the default, straightforward
    /// implementation; types that can produce a more direct function pointer
    /// or avoid additional captures may override it.
    fn into_fn(self) -> impl FnOnce(&mut T) -> R
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Non-consuming adapter to `BoxMutatingFunctionOnce`
    ///
    /// Creates a `BoxMutatingFunctionOnce<T, R>` that does not consume
    /// `self`. The default implementation requires `Self: Clone` and clones
    /// the receiver for the stored closure; the clone is consumed when the
    /// boxed function is invoked. Types that can provide a zero-cost adapter
    /// (for example clonable closures) should override this method to avoid
    /// unnecessary allocations.
    fn to_box(&self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming adapter to a callable `FnOnce(&mut T) -> R`
    ///
    /// Returns a closure that does not consume `self`. The default requires
    /// `Self: Clone` and clones `self` for the captured closure; the clone is
    /// consumed when the returned closure is invoked. Implementors may
    /// provide more efficient adapters for specific types.
    fn to_fn(&self) -> impl FnOnce(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }
}

// =======================================================================
// 2. BoxMutatingFunctionOnce - Single Ownership Implementation
// =======================================================================

/// BoxMutatingFunctionOnce struct
///
/// A one-time mutating function implementation based on
/// `Box<dyn FnOnce(&mut T) -> R>` for single ownership scenarios. This is
/// the only MutatingFunctionOnce implementation type because FnOnce
/// conflicts with shared ownership semantics.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes self on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Move Semantics**: Can capture and move variables
/// - **Method Chaining**: Compose multiple operations via `and_then`
/// - **Returns Results**: Unlike MutatorOnce, returns information
///
/// # Use Cases
///
/// Choose `BoxMutatingFunctionOnce` when:
/// - Need to store FnOnce closures (with moved captured variables)
/// - One-time resource transfer operations with results
/// - Post-initialization callbacks that return status
/// - Complex operations requiring ownership transfer and results
///
/// # Performance
///
/// `BoxMutatingFunctionOnce` performance characteristics:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Why No Arc/Rc Variants?
///
/// FnOnce can only be called once, which conflicts with Arc/Rc shared
/// ownership semantics:
/// - Arc/Rc implies multiple owners might need to call
/// - FnOnce is consumed after calling, cannot be called again
/// - This semantic incompatibility makes Arc/Rc variants meaningless
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// let data = vec![1, 2, 3];
/// let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
///     let old_len = x.len();
///     x.extend(data); // Move data
///     old_len
/// });
///
/// let mut target = vec![0];
/// let old_len = func.apply(&mut target);
/// assert_eq!(old_len, 1);
/// assert_eq!(target, vec![0, 1, 2, 3]);
/// ```
///
/// ## Method Chaining
///
/// ```rust
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// let data1 = vec![1, 2];
/// let data2 = vec![3, 4];
///
/// let chained = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
///     x.extend(data1);
///     x.len()
/// })
/// .and_then(move |x: &mut Vec<i32>| {
///     x.extend(data2);
///     x.len()
/// });
///
/// let mut target = vec![0];
/// let final_len = chained.apply(&mut target);
/// assert_eq!(final_len, 5);
/// assert_eq!(target, vec![0, 1, 2, 3, 4]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxMutatingFunctionOnce<T, R> {
    function: Box<dyn FnOnce(&mut T) -> R>,
    name: Option<String>,
}

impl<T, R> BoxMutatingFunctionOnce<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxMutatingFunctionOnce<T, R>,
        (FnOnce(&mut T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxMutatingFunctionOnce<T, R>,
        BoxConditionalMutatingFunctionOnce,
        FunctionOnce    // chains a non-mutating function after this mutating function
    );
}

impl<T, R> MutatingFunctionOnce<T, R> for BoxMutatingFunctionOnce<T, R> {
    fn apply(self, input: &mut T) -> R {
        (self.function)(input)
    }

    impl_box_once_conversions!(
        BoxMutatingFunctionOnce<T, R>,
        MutatingFunctionOnce,
        FnOnce(&mut T) -> R
    );
}

// Generates: identity() method for BoxMutatingFunctionOnce<T, T>
impl_function_identity_method!(BoxMutatingFunctionOnce<T, T>, mutating);

// Generates: Debug and Display implementations for BoxMutatingFunctionOnce<T, R>
impl_function_debug_display!(BoxMutatingFunctionOnce<T, R>);

// =======================================================================
// 3. Implement MutatingFunctionOnce trait for closures
// =======================================================================

// Implement MutatingFunctionOnce for all FnOnce(&mut T) -> R using macro
impl_closure_once_trait!(
    MutatingFunctionOnce<T, R>,
    apply,
    BoxMutatingFunctionOnce,
    FnOnce(input: &mut T) -> R
);

// =======================================================================
// 4. Provide extension methods for closures
// =======================================================================

// Generates: FnMutatingFunctionOnceOps trait and blanket implementation
impl_fn_ops_trait!(
    (FnOnce(&mut T) -> R),
    FnMutatingFunctionOnceOps,
    BoxMutatingFunctionOnce,
    FunctionOnce,
    BoxConditionalMutatingFunctionOnce
);

// ============================================================================
// BoxConditionalMutatingFunctionOnce - Box-based Conditional Mutating Function
// ============================================================================

/// BoxConditionalMutatingFunctionOnce struct
///
/// A conditional consuming transformer that only executes when a predicate is
/// satisfied. Uses `BoxMutatingFunctionOnce` and `BoxPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxMutatingFunctionOnce::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// let double = BoxMutatingFunctionOnce::new(|x: &mut i32| x * 2);
/// let negate = BoxMutatingFunctionOnce::new(|x: &mut i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
/// assert_eq!(conditional.apply(5), 10); // when branch executed
///
/// let double2 = BoxMutatingFunctionOnce::new(|x: &mut i32| x * 2);
/// let negate2 = BoxMutatingFunctionOnce::new(|x: &mut i32| -x);
/// let conditional2 = double2.when(|x: &i32| *x > 0).or_else(negate2);
/// assert_eq!(conditional2.apply(-5), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalMutatingFunctionOnce<T, R> {
    function: BoxMutatingFunctionOnce<T, R>,
    predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalMutatingFunctionOnce<T, R>,
    BoxMutatingFunctionOnce,
    MutatingFunctionOnce
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalMutatingFunctionOnce<T, R>);
