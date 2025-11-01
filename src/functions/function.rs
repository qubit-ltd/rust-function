/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Function Types
//!
//! Provides Rust implementations of function traits for computing output values
//! from input references. Functions borrow input values (not consuming them)
//! and produce output values.
//!
//! This module provides the `Function<T, R>` trait and three
//! implementations:
//!
//! - [`BoxFunction`]: Single ownership, not cloneable
//! - [`ArcFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcFunction`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

use crate::{
    functions::macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
        impl_function_identity_method,
        impl_shared_conditional_function,
        impl_shared_function_methods,
    },
    predicates::predicate::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    },
};

// ============================================================================
// Core Trait
// ============================================================================

/// Function trait - computes output from input reference
///
/// Defines the behavior of a function: computing a value of type `R`
/// from a reference to type `T` without consuming the input. This is analogous to
/// `Fn(&T) -> R` in Rust's standard library, similar to Java's `Function<T, R>`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (borrowed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait Function<T, R> {
    /// Applies the function to the input reference to produce an output value
    ///
    /// # Parameters
    ///
    /// * `t` - Reference to the input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, t: &T) -> R;

    /// Converts to BoxFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunction<T, R>`
    fn into_box(self) -> BoxFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxFunction::new(move |t| self.apply(t))
    }

    /// Converts to RcFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcFunction<T, R>`
    fn into_rc(self) -> RcFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        RcFunction::new(move |t| self.apply(t))
    }

    /// Converts to ArcFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates
    /// an `ArcFunction`. Types can override this method to provide
    /// more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcFunction<T, R>`
    fn into_arc(self) -> ArcFunction<T, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: 'static,
    {
        ArcFunction::new(move |t| self.apply(t))
    }

    /// Converts function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self`
    /// and calls its `transform` method. Types can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `Fn(&T) -> R`
    fn into_fn(self) -> impl Fn(&T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |t| self.apply(t)
    }

    /// Converts to BoxFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `BoxFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: i32| x * 2);
    /// let boxed = double.to_box();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(boxed.apply(21), 42);
    /// ```
    fn to_box(&self) -> BoxFunction<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Converts to RcFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `RcFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: i32| x * 2);
    /// let rc = double.to_rc();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(rc.apply(21), 42);
    /// ```
    fn to_rc(&self) -> RcFunction<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Converts to ArcFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `ArcFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: i32| x * 2);
    /// let arc = double.to_arc();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(arc.apply(21), 42);
    /// ```
    fn to_arc(&self) -> ArcFunction<T, R>
    where
        Self: Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts function to a closure without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures a
    /// clone of `self` and calls its `transform` method. Types can
    /// override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `Fn(&T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: i32| x * 2);
    /// let closure = double.to_fn();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(closure(21), 42);
    /// ```
    fn to_fn(&self) -> impl Fn(&T) -> R
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }
}

// ============================================================================
// BoxFunction - Box<dyn Fn(&T) -> R>
// ============================================================================

/// BoxFunction - function wrapper based on `Box<dyn Fn>`
///
/// A function wrapper that provides single ownership with reusable
/// transformation. The function consumes the input and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxFunction<T, R> {
    function: Box<dyn Fn(&T) -> R>,
    name: Option<String>,
}

impl<T, R> BoxFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxFunction<T, R>,
        BoxConditionalFunction,
        Function
    );
}

// Generates: constant() method for BoxFunction<T, R>
impl_function_constant_method!(BoxFunction<T, R>, 'static);

// Generates: identity() method for BoxFunction<T, T>
impl_function_identity_method!(BoxFunction<T, T>);

// Generates: Debug and Display implementations for BoxFunction<T, R>
impl_function_debug_display!(BoxFunction<T, R>);

// Implement Function trait for BoxFunction<T, R>
impl<T, R> Function<T, R> for BoxFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    // Override with zero-cost implementation: directly return itself
    fn into_box(self) -> BoxFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self
    }

    // Override with optimized implementation: convert Box to Rc
    fn into_rc(self) -> RcFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        RcFunction::new_with_optional_name(self.function, self.name)
    }

    // do NOT override BoxFunction::into_arc() because BoxFunction is not Send + Sync
    // and calling BoxFunction::to_arc() will cause a compile error

    // Override with optimized implementation: directly return the
    // underlying function by unwrapping the Box
    fn into_fn(self) -> impl Fn(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        self.function
    }

    // Note: BoxFunction doesn't implement Clone, so the default to_xxx()
    // implementations that require Clone cannot be used. We need to provide
    // special implementations that create new functions by wrapping the
    // function reference.

    // Override: BoxFunction doesn't implement Clone, can't use default
    // We create a new BoxFunction that references self through a closure
    // This requires T and R to be Clone-independent
    // Users should prefer using RcFunction if they need sharing

    // Note: We intentionally don't override to_box(), to_rc(), to_arc(), to_fn()
    // for BoxFunction because:
    // 1. BoxFunction doesn't implement Clone
    // 2. We can't share ownership of Box<dyn Fn> without cloning
    // 3. Users should convert to RcFunction or ArcFunction first if they
    //    need to create multiple references
    // 4. The default implementations will fail to compile (as expected), which
    //    guides users to the correct usage pattern
}

// ============================================================================
// RcFunction - Rc<dyn Fn(&T) -> R>
// ============================================================================

/// RcFunction - single-threaded function wrapper
///
/// A single-threaded, clonable function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&T) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct RcFunction<T, R> {
    function: Rc<dyn Fn(&T) -> R>,
    name: Option<String>,
}

impl<T, R> RcFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcFunction<T, R>,
        RcConditionalFunction,
        into_rc,
        Function,
        'static
    );
}

// Generates: constant() method for RcFunction<T, R>
impl_function_constant_method!(RcFunction<T, R>, 'static);

// Generates: identity() method for RcFunction<T, T>
impl_function_identity_method!(RcFunction<T, T>);

// Generates: Clone implementation for RcFunction<T, R>
impl_function_clone!(RcFunction<T, R>);

// Generates: Debug and Display implementations for RcFunction<T, R>
impl_function_debug_display!(RcFunction<T, R>);

// Implement Function trait for RcFunction<T, R>
impl<T, R> Function<T, R> for RcFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    fn into_box(self) -> BoxFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        BoxFunction::new_with_optional_name(move |t| (self.function)(t), self.name)
    }

    // Override with zero-cost implementation: directly return itself
    fn into_rc(self) -> RcFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self
    }

    // do NOT override RcFunction::into_arc() because RcFunction is not Send + Sync
    // and calling RcFunction::into_arc() will cause a compile error

    // Override with optimized implementation: wrap the Rc in a
    // closure to avoid double indirection
    fn into_fn(self) -> impl Fn(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        move |t| (self.function)(t)
    }

    // Override with optimized implementation: clone the Rc (cheap)
    fn to_box(&self) -> BoxFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        let self_name = self.name.clone();
        BoxFunction::new_with_optional_name(move |t| self_fn(t), self_name)
    }

    // Override with zero-cost implementation: clone itself
    fn to_rc(&self) -> RcFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self.clone()
    }

    // do NOT override RcFunction::to_arc() because RcFunction is not Send + Sync
    // and calling RcFunction::to_arc() will cause a compile error

    // Override with optimized implementation: clone the Rc (cheap)
    fn to_fn(&self) -> impl Fn(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        move |t| self_fn(t)
    }
}

// ============================================================================
// ArcFunction - Arc<dyn Fn(&T) -> R + Send + Sync>
// ============================================================================

/// ArcFunction - thread-safe function wrapper
///
/// A thread-safe, clonable function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&T) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct ArcFunction<T, R> {
    function: Arc<dyn Fn(&T) -> R>,
    name: Option<String>,
}

impl<T, R> ArcFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Arc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcFunction<T, R>,
        ArcConditionalFunction,
        into_arc,
        Function,
        Send + Sync + 'static
    );
}

// Generates: constant() method for ArcFunction<T, R>
impl_function_constant_method!(ArcFunction<T, R>, Send + Sync + 'static);

// Generates: identity() method for ArcFunction<T, T>
impl_function_identity_method!(ArcFunction<T, T>);

// Generates: Clone implementation for ArcFunction<T, R>
impl_function_clone!(ArcFunction<T, R>);

// Generates: Debug and Display implementations for ArcFunction<T, R>
impl_function_debug_display!(ArcFunction<T, R>);

// Implement Function trait for ArcFunction<T, R>
impl<T, R> Function<T, R> for ArcFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    fn into_box(self) -> BoxFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        BoxFunction::new_with_optional_name(move |t| (self.function)(t), self.name)
    }

    fn into_rc(self) -> RcFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        RcFunction::new_with_optional_name(move |t| (self.function)(t), self.name)
    }

    fn into_arc(self) -> ArcFunction<T, R>
    where
        T: Send + Sync + 'static,
        R: 'static,
    {
        self
    }

    fn into_fn(self) -> impl Fn(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        move |t| (self.function)(t)
    }

    fn to_box(&self) -> BoxFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        let self_name = self.name.clone();
        BoxFunction::new_with_optional_name(move |t| self_fn(t), self_name)
    }

    fn to_rc(&self) -> RcFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        let self_name = self.name.clone();
        RcFunction::new_with_optional_name(move |t| self_fn(t), self_name)
    }

    fn to_arc(&self) -> ArcFunction<T, R>
    where
        T: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.clone()
    }

    fn to_fn(&self) -> impl Fn(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        move |t| self_fn(t)
    }
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

/// Implement Function<T, R> for any type that implements Fn(&T) -> R
///
/// This allows closures and function pointers to be used directly with our
/// Function trait without wrapping.
///
/// # Examples
///
/// ```rust
/// use prism3_function::Function;
///
/// fn double(x: i32) -> i32 { x * 2 }
///
/// assert_eq!(double.apply(21), 42);
///
/// let triple = |x: i32| x * 3;
/// assert_eq!(triple.apply(14), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
impl<F, T, R> Function<T, R> for F
where
    F: Fn(&T) -> R,
    T: 'static,
    R: 'static,
{
    fn apply(&self, t: &T) -> R {
        self(t)
    }

    fn into_box(self) -> BoxFunction<T, R>
    where
        Self: Sized + 'static,
    {
        BoxFunction::new(self)
    }

    fn into_rc(self) -> RcFunction<T, R>
    where
        Self: Sized + 'static,
    {
        RcFunction::new(self)
    }

    fn into_arc(self) -> ArcFunction<T, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: 'static,
    {
        ArcFunction::new(self)
    }

    fn into_fn(self) -> impl Fn(&T) -> R
    where
        Self: Sized + 'static,
    {
        self
    }

    fn to_box(&self) -> BoxFunction<T, R>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    fn to_rc(&self) -> RcFunction<T, R>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    fn to_arc(&self) -> ArcFunction<T, R>
    where
        Self: Clone + Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: 'static,
    {
        self.clone().into_arc()
    }

    fn to_fn(&self) -> impl Fn(&T) -> R
    where
        Self: Clone + Sized + 'static,
    {
        self.clone()
    }
}

// ============================================================================
// FnFunctionOps - Extension trait for closure functions
// ============================================================================

/// Extension trait for closures implementing `Fn(&T) -> R`
///
/// Provides composition methods (`and_then`, `compose`, `when`) for closures
/// and function pointers without requiring explicit wrapping in
/// `BoxFunction`, `RcFunction`, or `ArcFunction`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `Fn(&T) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `Function<T, R>` through blanket
/// implementation, they don't have access to instance methods like `and_then`,
/// `compose`, and `when`. This extension trait provides those methods,
/// returning `BoxFunction` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use prism3_function::{Function, FnFunctionOps};
///
/// let double = |x: i32| x * 2;
/// let to_string = |x: i32| x.to_string();
///
/// let composed = double.and_then(to_string);
/// assert_eq!(composed.apply(21), "42");
/// ```
///
/// ## Reverse composition with compose
///
/// ```rust
/// use prism3_function::{Function, FnFunctionOps};
///
/// let double = |x: i32| x * 2;
/// let add_one = |x: i32| x + 1;
///
/// let composed = double.compose(add_one);
/// assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
/// ```
///
/// ## Conditional transformation with when
///
/// ```rust
/// use prism3_function::{Function, FnFunctionOps};
///
/// let double = |x: i32| x * 2;
/// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
///
/// assert_eq!(conditional.apply(5), 10);
/// assert_eq!(conditional.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnFunctionOps<T, R>: Fn(&T) -> R + Sized + 'static {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new function that applies this function first, then
    /// applies the after function to the result. Consumes self and returns
    /// a `BoxFunction`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after function
    /// * `F` - The type of the after function (must implement Function<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The function to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original function, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxFunction<R, S>`
    ///   - An `RcFunction<R, S>`
    ///   - An `ArcFunction<R, S>`
    ///   - Any type implementing `Function<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxFunction<T, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use prism3_function::{Function, FnFunctionOps, BoxFunction};
    ///
    /// let double = |x: i32| x * 2;
    /// let to_string = BoxFunction::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved here
    /// let composed = double.and_then(to_string);
    /// assert_eq!(composed.apply(21), "42");
    /// // to_string.apply(5); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with clone
    ///
    /// ```rust
    /// use prism3_function::{Function, FnFunctionOps, BoxFunction};
    ///
    /// let double = |x: i32| x * 2;
    /// let to_string = BoxFunction::new(|x: i32| x.to_string());
    ///
    /// // Clone to preserve original
    /// let composed = double.and_then(to_string.clone());
    /// assert_eq!(composed.apply(21), "42");
    ///
    /// // Original still usable
    /// assert_eq!(to_string.apply(5), "5");
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxFunction<T, S>
    where
        S: 'static,
        F: Function<R, S> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxFunction::new(move |x: &T| after.apply(&self(x)))
    }

    /// Reverse composition - applies before first, then self
    ///
    /// Creates a new function that applies the before function first,
    /// then applies this function to the result. Consumes self and returns
    /// a `BoxFunction`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The input type of the before function
    /// * `F` - The type of the before function (must implement Function<S, T>)
    ///
    /// # Parameters
    ///
    /// * `before` - The function to apply before self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original function, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: S| -> T`
    ///   - A function pointer: `fn(S) -> T`
    ///   - A `BoxFunction<S, T>`
    ///   - An `RcFunction<S, T>`
    ///   - An `ArcFunction<S, T>`
    ///   - Any type implementing `Function<S, T>`
    ///
    /// # Returns
    ///
    /// A new `BoxFunction<S, R>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use prism3_function::{Function, FnFunctionOps, BoxFunction};
    ///
    /// let double = |x: i32| x * 2;
    /// let add_one = BoxFunction::new(|x: i32| x + 1);
    ///
    /// // add_one is moved here
    /// let composed = double.compose(add_one);
    /// assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    /// // add_one.apply(3); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with clone
    ///
    /// ```rust
    /// use prism3_function::{Function, FnFunctionOps, BoxFunction};
    ///
    /// let double = |x: i32| x * 2;
    /// let add_one = BoxFunction::new(|x: i32| x + 1);
    ///
    /// // Clone to preserve original
    /// let composed = double.compose(add_one.clone());
    /// assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    ///
    /// // Original still usable
    /// assert_eq!(add_one.apply(3), 4);
    /// ```
    fn compose<S, F>(self, before: F) -> BoxFunction<S, R>
    where
        S: 'static,
        F: Function<S, T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxFunction::new(move |x: &S| self(&before.apply(x)))
    }

    /// Creates a conditional function
    ///
    /// Returns a function that only executes when a predicate is satisfied.
    /// You must call `or_else()` to provide an alternative function for when
    /// the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. **Note: This parameter is passed
    ///   by value and will transfer ownership.** If you need to preserve the
    ///   original predicate, clone it first (if it implements `Clone`). Can be:
    ///   - A closure: `|x: &T| -> bool`
    ///   - A function pointer: `fn(&T) -> bool`
    ///   - A `BoxPredicate<T>`
    ///   - An `RcPredicate<T>`
    ///   - An `ArcPredicate<T>`
    ///   - Any type implementing `Predicate<T>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use prism3_function::{Function, FnFunctionOps};
    ///
    /// let double = |x: i32| x * 2;
    /// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
    ///
    /// assert_eq!(conditional.apply(5), 10);
    /// assert_eq!(conditional.apply(-5), 5);
    /// ```
    ///
    /// ## Preserving predicate with clone
    ///
    /// ```rust
    /// use prism3_function::{Function, FnFunctionOps, BoxPredicate};
    ///
    /// let double = |x: i32| x * 2;
    /// let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
    ///
    /// // Clone to preserve original predicate
    /// let conditional = double.when(is_positive.clone())
    ///     .or_else(|x: i32| -x);
    ///
    /// assert_eq!(conditional.apply(5), 10);
    ///
    /// // Original predicate still usable
    /// assert!(is_positive.test(&3));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalFunction<T, R>
    where
        P: Predicate<T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxFunction::new(self).when(predicate)
    }
}

/// Blanket implementation of FnFunctionOps for all closures
///
/// Automatically implements `FnFunctionOps<T, R>` for any type that
/// implements `Fn(&T) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, R, F> FnFunctionOps<T, R> for F where F: Fn(&T) -> R + 'static {}

// ============================================================================
// UnaryOperator Trait - Marker trait for Function<T, T>
// ============================================================================

/// UnaryOperator trait - marker trait for unary operators
///
/// A unary operator transforms a value of type `T` to another value of the
/// same type `T`. This trait extends `Function<T, T>` to provide semantic
/// clarity for same-type transformations. Equivalent to Java's `UnaryOperator<T>`
/// which extends `Function<T, T>`.
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// `Function<T, T>`, so you don't need to implement it manually.
///
/// # Type Parameters
///
/// * `T` - The type of both input and output values
///
/// # Examples
///
/// ## Using in generic constraints
///
/// ```rust
/// use prism3_function::{UnaryOperator, Function};
///
/// fn apply_twice<T, O>(value: &T, op: O) -> T
/// where
///     O: UnaryOperator<T>,
///     T: Clone,
/// {
///     let result = op.apply(value.clone());
///     op.apply(result)
/// }
///
/// let increment = |x: i32| x + 1;
/// assert_eq!(apply_twice(5, increment), 7); // (5 + 1) + 1
/// ```
///
/// ## With concrete types
///
/// ```rust
/// use prism3_function::{BoxUnaryOperator, UnaryOperator, Function};
///
/// fn create_incrementer() -> BoxUnaryOperator<i32> {
///     BoxUnaryOperator::new(|x| x + 1)
/// }
///
/// let op = create_incrementer();
/// assert_eq!(op.apply(41), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait UnaryOperator<T>: Function<T, T> {}

/// Blanket implementation of UnaryOperator for all Function<T, T>
///
/// This automatically implements `UnaryOperator<T>` for any type that
/// implements `Function<T, T>`.
///
/// # Author
///
/// Haixing Hu
impl<F, T> UnaryOperator<T> for F
where
    F: Function<T, T>,
    T: 'static,
{
    // empty
}

// ============================================================================
// Type Aliases for UnaryOperator (Function<T, T>)
// ============================================================================

/// Type alias for `BoxFunction<T, T>`
///
/// Represents a unary operator that transforms a value of type `T` to another
/// value of the same type `T`, with single ownership semantics. Equivalent to
/// Java's `UnaryOperator<T>`.
///
/// # Examples
///
/// ```rust
/// use prism3_function::{BoxUnaryOperator, Function};
///
/// let increment: BoxUnaryOperator<i32> = BoxUnaryOperator::new(|x| x + 1);
/// assert_eq!(increment.apply(41), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxUnaryOperator<T> = BoxFunction<T, T>;

/// Type alias for `ArcFunction<T, T>`
///
/// Represents a thread-safe unary operator that transforms a value of type `T`
/// to another value of the same type `T`. Equivalent to Java's `UnaryOperator<T>`
/// with shared, thread-safe ownership.
///
/// # Examples
///
/// ```rust
/// use prism3_function::{ArcUnaryOperator, Function};
///
/// let double: ArcUnaryOperator<i32> = ArcUnaryOperator::new(|x| x * 2);
/// let double_clone = double.clone();
/// assert_eq!(double.apply(21), 42);
/// assert_eq!(double_clone.apply(21), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type ArcUnaryOperator<T> = ArcFunction<T, T>;

/// Type alias for `RcFunction<T, T>`
///
/// Represents a single-threaded unary operator that transforms a value of type
/// `T` to another value of the same type `T`. Equivalent to Java's
/// `UnaryOperator<T>` with shared, single-threaded ownership.
///
/// # Examples
///
/// ```rust
/// use prism3_function::{RcUnaryOperator, Function};
///
/// let negate: RcUnaryOperator<i32> = RcUnaryOperator::new(|x: i32| -x);
/// let negate_clone = negate.clone();
/// assert_eq!(negate.apply(42), -42);
/// assert_eq!(negate_clone.apply(42), -42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type RcUnaryOperator<T> = RcFunction<T, T>;

// ============================================================================
// BoxConditionalFunction - Box-based Conditional Function
// ============================================================================

/// BoxConditionalFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements Function**: Can be used anywhere a `Function` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use prism3_function::{Function, BoxFunction};
///
/// let double = BoxFunction::new(|x: i32| x * 2);
/// let negate = BoxFunction::new(|x: i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// assert_eq!(conditional.apply(5), 10); // when branch executed
/// assert_eq!(conditional.apply(-5), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalFunction<T, R> {
    function: BoxFunction<T, R>,
    predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalFunction<T, R>,
    BoxFunction,
    Function
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalFunction<T, R>);

// ============================================================================
// RcConditionalFunction - Rc-based Conditional Function
// ============================================================================

/// RcConditionalFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcFunction` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalFunction`
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Function, RcFunction};
///
/// let double = RcFunction::new(|x: i32| x * 2);
/// let identity = RcFunction::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5), 10);
/// assert_eq!(conditional_clone.apply(-5), -5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalFunction<T, R> {
    function: RcFunction<T, R>,
    predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalFunction<T, R>,
    RcFunction,
    Function,
    'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalFunction<T, R>);

// ============================================================================
// ArcConditionalFunction - Arc-based Conditional Function
// ============================================================================

/// ArcConditionalFunction struct
///
/// A thread-safe conditional function that only executes when a predicate is
/// satisfied. Uses `ArcFunction` and `ArcPredicate` for shared ownership
/// across threads.
///
/// This type is typically created by calling `ArcFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Function, ArcFunction};
///
/// let double = ArcFunction::new(|x: i32| x * 2);
/// let identity = ArcFunction::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5), 10);
/// assert_eq!(conditional_clone.apply(-5), -5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalFunction<T, R> {
    function: ArcFunction<T, R>,
    predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalFunction<T, R>,
    ArcFunction,
    Function,
    Send + Sync + 'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalFunction<T, R>);
