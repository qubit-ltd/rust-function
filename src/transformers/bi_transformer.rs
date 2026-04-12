/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # BiTransformer Types
//!
//! Provides Rust implementations of bi-transformer traits for type conversion
//! and value transformation with two inputs. BiTransformers consume two input
//! values (taking ownership) and produce an output value.
//!
//! This module provides the `BiTransformer<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiTransformer`]: Single ownership, not cloneable
//! - [`ArcBiTransformer`]: Thread-safe shared ownership, cloneable
//! - [`RcBiTransformer`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_rc_conversions,
};
use crate::predicates::bi_predicate::{
    ArcBiPredicate,
    BiPredicate,
    BoxBiPredicate,
    RcBiPredicate,
};
use crate::transformers::{
    bi_transformer_once::BoxBiTransformerOnce,
    macros::{
        impl_box_conditional_transformer,
        impl_box_transformer_methods,
        impl_conditional_transformer_clone,
        impl_conditional_transformer_debug_display,
        impl_shared_conditional_transformer,
        impl_shared_transformer_methods,
        impl_transformer_clone,
        impl_transformer_common_methods,
        impl_transformer_constant_method,
        impl_transformer_debug_display,
    },
    transformer::Transformer,
};

// ============================================================================
// Core Trait
// ============================================================================

/// BiTransformer trait - transforms two values to produce a result
///
/// Defines the behavior of a bi-transformation: converting two values of types
/// `T` and `U` to a value of type `R` by consuming the inputs. This is
/// analogous to `Fn(T, U) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (consumed)
/// * `U` - The type of the second input value (consumed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait BiTransformer<T, U, R> {
    /// Transforms two input values to produce an output value
    ///
    /// # Parameters
    ///
    /// * `first` - The first input value to transform (consumed)
    /// * `second` - The second input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&self, first: T, second: U) -> R;

    /// Converts to BoxBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiTransformer<T, U, R>`
    fn into_box(self) -> BoxBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiTransformer::new(move |x, y| self.apply(x, y))
    }

    /// Converts to RcBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcBiTransformer<T, U, R>`
    fn into_rc(self) -> RcBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        RcBiTransformer::new(move |x, y| self.apply(x, y))
    }

    /// Converts to ArcBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates an
    /// `ArcBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcBiTransformer<T, U, R>`
    fn into_arc(self) -> ArcBiTransformer<T, U, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        ArcBiTransformer::new(move |x, y| self.apply(x, y))
    }

    /// Converts bi-transformer to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self`
    /// and calls its `apply` method. Types can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `Fn(T, U) -> R`
    fn into_fn(self) -> impl Fn(T, U) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        move |t, u| self.apply(t, u)
    }

    /// Convert to BiTransformerOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer will be unavailable after calling this method.
    ///
    /// Converts a reusable bi-transformer to a one-time bi-transformer that consumes itself on use.
    /// This enables passing `BiTransformer` to functions that require `BiTransformerOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiTransformerOnce<T, U, R>`
    fn into_once(self) -> BoxBiTransformerOnce<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiTransformerOnce::new(move |t, u| self.apply(t, u))
    }

    /// Non-consuming conversion to `BoxBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_box`.
    fn to_box(&self) -> BoxBiTransformer<T, U, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_rc`.
    fn to_rc(&self) -> RcBiTransformer<T, U, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_arc`.
    fn to_arc(&self) -> ArcBiTransformer<T, U, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a boxed function using `&self`.
    ///
    /// Returns a `Box<dyn Fn(T, U) -> R>` that clones `self` and calls
    /// `apply` inside the boxed closure.
    fn to_fn(&self) -> impl Fn(T, U) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to BiTransformerOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current bi-transformer and converts the clone to a one-time bi-transformer.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiTransformerOnce<T, U, R>`
    fn to_once(&self) -> BoxBiTransformerOnce<T, U, R>
    where
        Self: Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_once()
    }
}

// ============================================================================
// BoxBiTransformer - Box<dyn Fn(T, U) -> R>
// ============================================================================

/// BoxBiTransformer - bi-transformer wrapper based on `Box<dyn Fn>`
///
/// A bi-transformer wrapper that provides single ownership with reusable
/// transformation. The bi-transformer consumes both inputs and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(T, U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxBiTransformer<T, U, R> {
    function: Box<dyn Fn(T, U) -> R>,
    name: Option<String>,
}

// Implement BoxBiTransformer
impl<T, U, R> BoxBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        BoxBiTransformer<T, U, R>,
        (Fn(T, U) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxBiTransformer<T, U, R>,
        BoxConditionalBiTransformer,
        Transformer
    );
}

// Implement constant method for BoxBiTransformer
impl_transformer_constant_method!(BoxBiTransformer<T, U, R>);

// Implement Debug and Display for BoxBiTransformer
impl_transformer_debug_display!(BoxBiTransformer<T, U, R>);

// Implement BiTransformer trait for BoxBiTransformer
impl<T, U, R> BiTransformer<T, U, R> for BoxBiTransformer<T, U, R> {
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxBiTransformer<T, U, R>,
        RcBiTransformer,
        Fn(T, U) -> R,
        BoxBiTransformerOnce
    );
}

// ============================================================================
// RcBiTransformer - Rc<dyn Fn(T, U) -> R>
// ============================================================================

/// RcBiTransformer - single-threaded bi-transformer wrapper
///
/// A single-threaded, clonable bi-transformer wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(T, U) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct RcBiTransformer<T, U, R> {
    function: Rc<dyn Fn(T, U) -> R>,
    name: Option<String>,
}

impl<T, U, R> RcBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        RcBiTransformer<T, U, R>,
        (Fn(T, U) -> R + 'static),
        |f| Rc::new(f)
    );

    impl_shared_transformer_methods!(
        RcBiTransformer<T, U, R>,
        RcConditionalBiTransformer,
        into_rc,
        Transformer,
        'static
    );
}

// Implement constant method for RcBiTransformer
impl_transformer_constant_method!(RcBiTransformer<T, U, R>);

// Implement Debug and Display for RcBiTransformer
impl_transformer_debug_display!(RcBiTransformer<T, U, R>);

// Implement Clone for RcBiTransformer
impl_transformer_clone!(RcBiTransformer<T, U, R>);

// Implement BiTransformer trait for RcBiTransformer
impl<T, U, R> BiTransformer<T, U, R> for RcBiTransformer<T, U, R> {
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcBiTransformer<T, U, R>,
        BoxBiTransformer,
        BoxBiTransformerOnce,
        Fn(first: T, second: U) -> R
    );

    // do NOT override RcBiTransformer::into_arc() because RcBiTransformer is not Send + Sync
    // and calling RcBiTransformer::into_arc() will cause a compile error

    // do NOT override RcBiTransformer::to_arc() because RcBiTransformer is not Send + Sync
    // and calling RcBiTransformer::to_arc() will cause a compile error
}

// ============================================================================
// ArcBiTransformer - Arc<dyn Fn(T, U) -> R + Send + Sync>
// ============================================================================

/// ArcBiTransformer - thread-safe bi-transformer wrapper
///
/// A thread-safe, clonable bi-transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(T, U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct ArcBiTransformer<T, U, R> {
    function: Arc<dyn Fn(T, U) -> R + Send + Sync>,
    name: Option<String>,
}

impl<T, U, R> ArcBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        ArcBiTransformer<T, U, R>,
        (Fn(T, U) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    impl_shared_transformer_methods!(
        ArcBiTransformer<T, U, R>,
        ArcConditionalBiTransformer,
        into_arc,
        Transformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcBiTransformer
impl_transformer_constant_method!(thread_safe ArcBiTransformer<T, U, R>);

// Implement Debug and Display for ArcBiTransformer
impl_transformer_debug_display!(ArcBiTransformer<T, U, R>);

// Implement Clone for ArcBiTransformer
impl_transformer_clone!(ArcBiTransformer<T, U, R>);

// Implement BiTransformer trait for ArcBiTransformer
impl<T, U, R> BiTransformer<T, U, R> for ArcBiTransformer<T, U, R> {
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcBiTransformer<T, U, R>,
        BoxBiTransformer,
        RcBiTransformer,
        BoxBiTransformerOnce,
        Fn(t: T, u: U) -> R
    );
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

/// Implement BiTransformer<T, U, R> for any type that implements Fn(T, U) -> R
///
/// This allows closures and function pointers to be used directly with our
/// BiTransformer trait without wrapping.
///
/// # Examples
///
/// ```rust
/// use qubit_function::BiTransformer;
///
/// fn add(x: i32, y: i32) -> i32 { x + y }
///
/// assert_eq!(add.apply(20, 22), 42);
///
/// let multiply = |x: i32, y: i32| x * y;
/// assert_eq!(multiply.apply(6, 7), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
impl<F, T, U, R> BiTransformer<T, U, R> for F
where
    F: Fn(T, U) -> R,
{
    fn apply(&self, first: T, second: U) -> R {
        self(first, second)
    }

    fn into_box(self) -> BoxBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiTransformer::new(self)
    }

    fn into_rc(self) -> RcBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
    {
        RcBiTransformer::new(self)
    }

    fn into_arc(self) -> ArcBiTransformer<T, U, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        ArcBiTransformer::new(self)
    }

    fn into_fn(self) -> impl Fn(T, U) -> R
    where
        Self: Sized + 'static,
    {
        move |t: T, u: U| self(t, u)
    }

    // use the default implementation of to_box(), to_rc(), to_arc() from
    // BiTransformer trait

    fn to_fn(&self) -> impl Fn(T, U) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone()
    }
}

// ============================================================================
// FnBiTransformerOps - Extension trait for Fn(T, U) -> R bi-transformers
// ============================================================================

/// Extension trait for closures implementing `Fn(T, U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for bi-transformer
/// closures and function pointers without requiring explicit wrapping in
/// `BoxBiTransformer`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `Fn(T, U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiTransformer<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiTransformer` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{BiTransformer, FnBiTransformerOps};
///
/// let add = |x: i32, y: i32| x + y;
/// let double = |x: i32| x * 2;
///
/// let composed = add.and_then(double);
/// assert_eq!(composed.apply(3, 5), 16); // (3 + 5) * 2
/// ```
///
/// ## Conditional execution with when
///
/// ```rust
/// use qubit_function::{BiTransformer, FnBiTransformerOps};
///
/// let add = |x: i32, y: i32| x + y;
/// let multiply = |x: i32, y: i32| x * y;
///
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(5, 3), 8);   // add
/// assert_eq!(conditional.apply(-5, 3), -15); // multiply
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnBiTransformerOps<T, U, R>: Fn(T, U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-transformer that applies this bi-transformer first,
    /// then applies the after transformer to the result. Consumes self and
    /// returns a `BoxBiTransformer`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after transformer
    /// * `F` - The type of the after transformer (must implement Transformer<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The transformer to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original transformer, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxTransformer<R, S>`
    ///   - An `RcTransformer<R, S>`
    ///   - An `ArcTransformer<R, S>`
    ///   - Any type implementing `Transformer<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxBiTransformer<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use qubit_function::{BiTransformer, FnBiTransformerOps,
    ///     BoxTransformer};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let to_string = BoxTransformer::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved here
    /// let composed = add.and_then(to_string);
    /// assert_eq!(composed.apply(20, 22), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with clone
    ///
    /// ```rust
    /// use qubit_function::{BiTransformer, FnBiTransformerOps,
    ///     BoxTransformer};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let to_string = BoxTransformer::new(|x: i32| x.to_string());
    ///
    /// // Clone to preserve original
    /// let composed = add.and_then(to_string.clone());
    /// assert_eq!(composed.apply(20, 22), "42");
    ///
    /// // Original still usable
    /// assert_eq!(to_string.apply(10), "10");
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiTransformer<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: crate::transformers::transformer::Transformer<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiTransformer::new(move |t: T, u: U| after.apply(self(t, u)))
    }

    /// Creates a conditional bi-transformer
    ///
    /// Returns a bi-transformer that only executes when a bi-predicate is
    /// satisfied. You must call `or_else()` to provide an alternative
    /// bi-transformer for when the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. **Note: This parameter is passed
    ///   by value and will transfer ownership.** If you need to preserve the
    ///   original bi-predicate, clone it first (if it implements `Clone`).
    ///   Can be:
    ///   - A closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalBiTransformer<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{BiTransformer, FnBiTransformerOps};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let conditional = add.when(|x: &i32, y: &i32| *x > 0)
    ///     .or_else(|x: i32, y: i32| x * y);
    ///
    /// assert_eq!(conditional.apply(5, 3), 8);
    /// assert_eq!(conditional.apply(-5, 3), -15);
    /// ```
    ///
    /// ## Preserving bi-predicate with clone
    ///
    /// ```rust
    /// use qubit_function::{BiTransformer, FnBiTransformerOps,
    ///     RcBiPredicate};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let both_positive = RcBiPredicate::new(|x: &i32, y: &i32|
    ///     *x > 0 && *y > 0);
    ///
    /// // Clone to preserve original bi-predicate
    /// let conditional = add.when(both_positive.clone())
    ///     .or_else(|x: i32, y: i32| x * y);
    ///
    /// assert_eq!(conditional.apply(5, 3), 8);
    ///
    /// // Original bi-predicate still usable
    /// assert!(both_positive.test(&5, &3));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalBiTransformer<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiTransformer::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiTransformerOps for all closures
///
/// Automatically implements `FnBiTransformerOps<T, U, R>` for any type that
/// implements `Fn(T, U) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, U, R, F> FnBiTransformerOps<T, U, R> for F where F: Fn(T, U) -> R {}

// ============================================================================
// BinaryOperator Trait - Marker trait for BiTransformer<T, T, T>
// ============================================================================

/// BinaryOperator trait - marker trait for binary operators
///
/// A binary operator takes two values of type `T` and produces a value of the
/// same type `T`. This trait extends `BiTransformer<T, T, T>` to provide
/// semantic clarity for same-type binary operations. Equivalent to Java's
/// `BinaryOperator<T>` which extends `BiFunction<T, T, T>`.
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// `BiTransformer<T, T, T>`, so you don't need to implement it manually.
///
/// # Type Parameters
///
/// * `T` - The type of both input values and the output value
///
/// # Examples
///
/// ## Using in generic constraints
///
/// ```rust
/// use qubit_function::{BinaryOperator, BiTransformer};
///
/// fn reduce<T, O>(values: Vec<T>, initial: T, op: O) -> T
/// where
///     O: BinaryOperator<T>,
///     T: Clone,
/// {
///     values.into_iter().fold(initial, |acc, val| op.apply(acc, val))
/// }
///
/// let sum = |a: i32, b: i32| a + b;
/// assert_eq!(reduce(vec![1, 2, 3, 4], 0, sum), 10);
/// ```
///
/// ## With concrete types
///
/// ```rust
/// use qubit_function::{BoxBinaryOperator, BinaryOperator, BiTransformer};
///
/// fn create_adder() -> BoxBinaryOperator<i32> {
///     BoxBinaryOperator::new(|x, y| x + y)
/// }
///
/// let op = create_adder();
/// assert_eq!(op.apply(20, 22), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait BinaryOperator<T>: BiTransformer<T, T, T> {}

/// Blanket implementation of BinaryOperator for all BiTransformer<T, T, T>
///
/// This automatically implements `BinaryOperator<T>` for any type that
/// implements `BiTransformer<T, T, T>`.
///
/// # Author
///
/// Haixing Hu
impl<F, T> BinaryOperator<T> for F
where
    F: BiTransformer<T, T, T>,
{
    // empty
}

// ============================================================================
// Type Aliases for BinaryOperator (BiTransformer<T, T, T>)
// ============================================================================

/// Type alias for `BoxBiTransformer<T, T, T>`
///
/// Represents a binary operator that takes two values of type `T` and produces
/// a value of the same type `T`, with single ownership semantics. Equivalent to
/// Java's `BinaryOperator<T>`.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxBinaryOperator, BiTransformer};
///
/// let add: BoxBinaryOperator<i32> = BoxBinaryOperator::new(|x, y| x + y);
/// assert_eq!(add.apply(20, 22), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxBinaryOperator<T> = BoxBiTransformer<T, T, T>;

/// Type alias for `ArcBiTransformer<T, T, T>`
///
/// Represents a thread-safe binary operator that takes two values of type `T`
/// and produces a value of the same type `T`. Equivalent to Java's
/// `BinaryOperator<T>` with shared, thread-safe ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcBinaryOperator, BiTransformer};
///
/// let multiply: ArcBinaryOperator<i32> = ArcBinaryOperator::new(|x, y| x * y);
/// let multiply_clone = multiply.clone();
/// assert_eq!(multiply.apply(6, 7), 42);
/// assert_eq!(multiply_clone.apply(6, 7), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type ArcBinaryOperator<T> = ArcBiTransformer<T, T, T>;

/// Type alias for `RcBiTransformer<T, T, T>`
///
/// Represents a single-threaded binary operator that takes two values of type
/// `T` and produces a value of the same type `T`. Equivalent to Java's
/// `BinaryOperator<T>` with shared, single-threaded ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcBinaryOperator, BiTransformer};
///
/// let max: RcBinaryOperator<i32> = RcBinaryOperator::new(|x, y| if x > y { x } else { y });
/// let max_clone = max.clone();
/// assert_eq!(max.apply(30, 42), 42);
/// assert_eq!(max_clone.apply(30, 42), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type RcBinaryOperator<T> = RcBiTransformer<T, T, T>;

// ============================================================================
// BoxConditionalBiTransformer - Box-based Conditional BiTransformer
// ============================================================================

/// BoxConditionalBiTransformer struct
///
/// A conditional bi-transformer that only executes when a bi-predicate is
/// satisfied. Uses `BoxBiTransformer` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiTransformer**: Can be used anywhere a `BiTransformer` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiTransformer, BoxBiTransformer};
///
/// let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = BoxBiTransformer::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(5, 3), 8);  // when branch executed
/// assert_eq!(conditional.apply(-5, 3), -15); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalBiTransformer<T, U, R> {
    transformer: BoxBiTransformer<T, U, R>,
    predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalTransformer
impl_box_conditional_transformer!(
    BoxConditionalBiTransformer<T, U, R>,
    BoxBiTransformer,
    BiTransformer
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalBiTransformer<T, U, R>);

// ============================================================================
// RcConditionalBiTransformer - Rc-based Conditional BiTransformer
// ============================================================================

/// RcConditionalBiTransformer struct
///
/// A single-threaded conditional bi-transformer that only executes when a
/// bi-predicate is satisfied. Uses `RcBiTransformer` and `RcBiPredicate` for
/// shared ownership within a single thread.
///
/// This type is typically created by calling `RcBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalBiTransformer`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiTransformer, RcBiTransformer};
///
/// let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = RcBiTransformer::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5, 3), 8);
/// assert_eq!(conditional_clone.apply(-5, 3), -15);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalBiTransformer<T, U, R> {
    transformer: RcBiTransformer<T, U, R>,
    predicate: RcBiPredicate<T, U>,
}

// Implement RcConditionalBiTransformer
impl_shared_conditional_transformer!(
    RcConditionalBiTransformer<T, U, R>,
    RcBiTransformer,
    BiTransformer,
    into_rc,
    'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalBiTransformer<T, U, R>);

// Implement Clone for RcConditionalBiTransformer
impl_conditional_transformer_clone!(RcConditionalBiTransformer<T, U, R>);

// ============================================================================
// ArcConditionalBiTransformer - Arc-based Conditional BiTransformer
// ============================================================================

/// ArcConditionalBiTransformer struct
///
/// A thread-safe conditional bi-transformer that only executes when a
/// bi-predicate is satisfied. Uses `ArcBiTransformer` and `ArcBiPredicate` for
/// shared ownership across threads.
///
/// This type is typically created by calling `ArcBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiTransformer, ArcBiTransformer};
///
/// let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = ArcBiTransformer::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5, 3), 8);
/// assert_eq!(conditional_clone.apply(-5, 3), -15);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalBiTransformer<T, U, R> {
    transformer: ArcBiTransformer<T, U, R>,
    predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiTransformer
impl_shared_conditional_transformer!(
    ArcConditionalBiTransformer<T, U, R>,
    ArcBiTransformer,
    BiTransformer,
    into_arc,
    Send + Sync + 'static
);

// Implement Debug and Display for ArcConditionalBiTransformer
impl_conditional_transformer_debug_display!(ArcConditionalBiTransformer<T, U, R>);

// Implement Clone for ArcConditionalBiTransformer
impl_conditional_transformer_clone!(ArcConditionalBiTransformer<T, U, R>);
