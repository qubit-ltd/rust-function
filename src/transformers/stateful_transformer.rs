/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # StatefulTransformer Types
//!
//! Provides Rust implementations of stateful transformer traits for stateful value
//! transformation. StatefulTransformers consume input values (taking ownership) and
//! produce output values while allowing internal state modification. This is
//! analogous to `FnMut(T) -> R` in Rust's standard library.
//!
//! This module provides the `StatefulTransformer<T, R>` trait and three implementations:
//!
//! - [`BoxStatefulTransformer`]: Single ownership, not cloneable
//! - [`ArcStatefulTransformer`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulTransformer`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};
use crate::transformers::{
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
    transformer_once::BoxTransformerOnce,
};

// ============================================================================
// Core Trait
// ============================================================================

/// StatefulTransformer trait - transforms values from type T to type R with state
///
/// Defines the behavior of a stateful transformation: converting a value
/// of type `T` to a value of type `R` by consuming the input while
/// allowing modification of internal state. This is analogous to
/// `FnMut(T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait StatefulTransformer<T, R> {
    /// Applies the transformation to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `input` - The input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&mut self, input: T) -> R;

    /// Converts to BoxStatefulTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxStatefulTransformer<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `BoxStatefulTransformer` by creating
    /// a new closure that calls `self.apply()`. This provides a zero-cost
    /// abstraction for most use cases.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulTransformer, BoxStatefulTransformer};
    ///
    /// struct CustomTransformer {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulTransformer<i32, i32> for CustomTransformer {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let transformer = CustomTransformer { multiplier: 0 };
    /// let mut boxed = transformer.into_box();
    /// assert_eq!(boxed.apply(10), 10);  // 10 * 1
    /// assert_eq!(boxed.apply(10), 20);  // 10 * 2
    /// ```
    fn into_box(self) -> BoxStatefulTransformer<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let mut transformer = self;
        BoxStatefulTransformer::new(move |t| transformer.apply(t))
    }

    /// Converts to RcStatefulTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `RcStatefulTransformer<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation first converts to `BoxStatefulTransformer` using
    /// `into_box()`, then wraps it in `RcStatefulTransformer`. Specific implementations
    /// may override this for better efficiency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulTransformer, RcStatefulTransformer};
    ///
    /// struct CustomTransformer {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulTransformer<i32, i32> for CustomTransformer {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let transformer = CustomTransformer { multiplier: 0 };
    /// let mut rc_transformer = transformer.into_rc();
    /// assert_eq!(rc_transformer.apply(10), 10);  // 10 * 1
    /// assert_eq!(rc_transformer.apply(10), 20);  // 10 * 2
    /// ```
    fn into_rc(self) -> RcStatefulTransformer<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let mut transformer = self;
        RcStatefulTransformer::new(move |t| transformer.apply(t))
    }

    /// Converts to ArcStatefulTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `ArcStatefulTransformer<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `ArcStatefulTransformer` by creating
    /// a new closure that calls `self.apply()`. Note that this requires `self`
    /// to implement `Send` due to Arc's thread-safety requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulTransformer, ArcStatefulTransformer};
    ///
    /// struct CustomTransformer {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulTransformer<i32, i32> for CustomTransformer {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let transformer = CustomTransformer { multiplier: 0 };
    /// let mut arc_transformer = transformer.into_arc();
    /// assert_eq!(arc_transformer.apply(10), 10);  // 10 * 1
    /// assert_eq!(arc_transformer.apply(10), 20);  // 10 * 2
    /// ```
    fn into_arc(self) -> ArcStatefulTransformer<T, R>
    where
        Self: Sized + Send + 'static,
        T: Send + Sync + 'static,
        R: Send + 'static,
    {
        let mut transformer = self;
        ArcStatefulTransformer::new(move |t| transformer.apply(t))
    }

    /// Converts to a closure implementing `FnMut(T) -> R`
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns an implementation of `FnMut(T) -> R`
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new closure that calls `self.apply()`.
    /// Specific implementations may override this for better efficiency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulTransformer, BoxStatefulTransformer};
    ///
    /// let transformer = BoxStatefulTransformer::new(|x: i32| x * 2);
    /// let mut closure = transformer.into_fn();
    /// assert_eq!(closure(10), 20);
    /// assert_eq!(closure(15), 30);
    /// ```
    fn into_fn(self) -> impl FnMut(T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let mut transformer = self;
        move |t| transformer.apply(t)
    }

    /// Converts to `BoxTransformerOnce`.
    ///
    /// This method has a default implementation that wraps the
    /// transformer in a `BoxTransformerOnce`. Custom implementations
    /// can override this method for optimization purposes.
    ///
    /// # Returns
    ///
    /// A new `BoxTransformerOnce<T, R>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::StatefulTransformer;
    ///
    /// let closure = |x: i32| x * 2;
    /// let once = closure.into_once();
    /// assert_eq!(once.apply(5), 10);
    /// ```
    fn into_once(self) -> BoxTransformerOnce<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let mut transformer = self;
        BoxTransformerOnce::new(move |t| transformer.apply(t))
    }

    /// Non-consuming conversion to `BoxStatefulTransformer`.
    ///
    /// Default implementation requires `Self: Clone` and wraps a cloned
    /// instance in a `RefCell` so the returned transformer can mutate state
    /// across calls.
    fn to_box(&self) -> BoxStatefulTransformer<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcStatefulTransformer`.
    ///
    /// Default implementation clones `self` into an `Rc<RefCell<_>>` so the
    /// resulting transformer can be shared within a single thread.
    fn to_rc(&self) -> RcStatefulTransformer<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcStatefulTransformer` (thread-safe).
    ///
    /// Default implementation requires `Self: Clone + Send + Sync` and wraps
    /// the cloned instance in `Arc<Mutex<_>>` so it can be used across
    /// threads.
    fn to_arc(&self) -> ArcStatefulTransformer<T, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a closure (`FnMut(T) -> R`).
    ///
    /// Default implementation clones `self` into a `RefCell` and returns a
    /// closure that calls `apply` on the interior mutable value.
    fn to_fn(&self) -> impl FnMut(T) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }

    /// Creates a `BoxTransformerOnce` from a cloned transformer
    ///
    /// Uses `Clone` to obtain an owned copy and converts it into a
    /// `BoxTransformerOnce`. Requires `Self: Clone`. Custom implementations
    /// can override this for better performance.
    fn to_once(&self) -> BoxTransformerOnce<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_once()
    }
}

// ============================================================================
// BoxStatefulTransformer - Box<dyn FnMut(T) -> R>
// ============================================================================

/// BoxStatefulTransformer - transformer wrapper based on `Box<dyn FnMut>`
///
/// A transformer wrapper that provides single ownership with reusable stateful
/// transformation. The transformer consumes the input and can be called
/// multiple times while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnMut(T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
/// - **Statefulness**: Can modify internal state between calls
///
/// # Author
///
/// Haixing Hu
pub struct BoxStatefulTransformer<T, R> {
    function: Box<dyn FnMut(T) -> R>,
    name: Option<String>,
}

impl<T, R> BoxStatefulTransformer<T, R>
where
    T: 'static,
    R: 'static,
{
    impl_transformer_common_methods!(
        BoxStatefulTransformer<T, R>,
        (FnMut(T) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxStatefulTransformer<T, R>,
        BoxConditionalStatefulTransformer,
        StatefulTransformer
    );
}

// Implement constant method for BoxStatefulTransformer
impl_transformer_constant_method!(stateful BoxStatefulTransformer<T, R>);

// Implement Debug and Display for BoxStatefulTransformer
impl_transformer_debug_display!(BoxStatefulTransformer<T, R>);

// Implement StatefulTransformer trait for BoxStatefulTransformer
impl<T, R> StatefulTransformer<T, R> for BoxStatefulTransformer<T, R> {
    fn apply(&mut self, input: T) -> R {
        (self.function)(input)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxStatefulTransformer<T, R>,
        RcStatefulTransformer,
        FnMut(T) -> R,
        BoxTransformerOnce
    );

    // do NOT override StatefulTransformer::to_xxx() because BoxStatefulTransformer is not Clone
    // and calling BoxStatefulTransformer::to_xxx() will cause a compile error
}

// ============================================================================
// RcStatefulTransformer - Rc<RefCell<dyn FnMut(T) -> R>>
// ============================================================================

/// RcStatefulTransformer - single-threaded transformer wrapper
///
/// A single-threaded, clonable transformer wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<RefCell<dyn FnMut(T) -> R>>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
/// # Author
///
/// Haixing Hu
pub struct RcStatefulTransformer<T, R> {
    function: Rc<RefCell<dyn FnMut(T) -> R>>,
    name: Option<String>,
}

// Implement RcStatefulTransformer
impl<T, R> RcStatefulTransformer<T, R>
where
    T: 'static,
    R: 'static,
{
    impl_transformer_common_methods!(
        RcStatefulTransformer<T, R>,
        (FnMut(T) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    impl_shared_transformer_methods!(
        RcStatefulTransformer<T, R>,
        RcConditionalStatefulTransformer,
        into_rc,
        StatefulTransformer,
        'static
    );
}

// Implement constant method for RcStatefulTransformer
impl_transformer_constant_method!(stateful RcStatefulTransformer<T, R>);

// Implement Debug and Display for RcStatefulTransformer
impl_transformer_debug_display!(RcStatefulTransformer<T, R>);

// Implement Clone for RcStatefulTransformer
impl_transformer_clone!(RcStatefulTransformer<T, R>);

// Implement StatefulTransformer trait for RcStatefulTransformer
impl<T, R> StatefulTransformer<T, R> for RcStatefulTransformer<T, R> {
    fn apply(&mut self, input: T) -> R {
        let mut self_fn = self.function.borrow_mut();
        self_fn(input)
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcStatefulTransformer<T, R>,
        BoxStatefulTransformer,
        BoxTransformerOnce,
        FnMut(input: T) -> R
    );
}

// ============================================================================
// ArcStatefulTransformer - Arc<Mutex<dyn FnMut(T) -> R + Send>>
// ============================================================================

/// ArcStatefulTransformer - thread-safe transformer wrapper
///
/// A thread-safe, clonable transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads
/// while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Arc<Mutex<dyn FnMut(T) -> R + Send>>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Thread-safe (`Send` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
/// # Author
///
/// Haixing Hu
pub struct ArcStatefulTransformer<T, R> {
    function: Arc<Mutex<dyn FnMut(T) -> R + Send>>,
    name: Option<String>,
}

impl<T, R> ArcStatefulTransformer<T, R>
where
    T: 'static,
    R: 'static,
{
    impl_transformer_common_methods!(
        ArcStatefulTransformer<T, R>,
        (FnMut(T) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    impl_shared_transformer_methods!(
        ArcStatefulTransformer<T, R>,
        ArcConditionalStatefulTransformer,
        into_arc,
        StatefulTransformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcStatefulTransformer
impl_transformer_constant_method!(stateful thread_safe ArcStatefulTransformer<T, R>);

// Implement Debug and Display for ArcStatefulTransformer
impl_transformer_debug_display!(ArcStatefulTransformer<T, R>);

// Implement Clone for ArcStatefulTransformer
impl_transformer_clone!(ArcStatefulTransformer<T, R>);

impl<T, R> StatefulTransformer<T, R> for ArcStatefulTransformer<T, R> {
    fn apply(&mut self, input: T) -> R {
        let mut func = self.function.lock();
        func(input)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulTransformer<T, R>,
        BoxStatefulTransformer,
        RcStatefulTransformer,
        BoxTransformerOnce,
        FnMut(t: T) -> R
    );
}

// ============================================================================
// Blanket implementation for standard FnMut trait
// ============================================================================

// Implement StatefulTransformer<T, R> for any type that implements FnMut(T) -> R
impl_closure_trait!(
    StatefulTransformer<T, R>,
    apply,
    BoxTransformerOnce,
    FnMut(input: T) -> R
);

// ============================================================================
// FnStatefulTransformerOps - Extension trait for closure transformers
// ============================================================================

/// Extension trait for closures implementing `FnMut(T) -> R`
///
/// Provides composition methods (`and_then`, `compose`, `when`) for
/// closures without requiring explicit wrapping in `BoxStatefulTransformer`,
/// `RcStatefulTransformer`, or `ArcStatefulTransformer`.
///
/// This trait is automatically implemented for all closures that
/// implement `FnMut(T) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `StatefulTransformer<T, R>` through blanket
/// implementation, they don't have access to instance methods like
/// `and_then`, `compose`, and `when`. This extension trait provides
/// those methods, returning `BoxStatefulTransformer` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use prism3_function::{StatefulTransformer, FnStatefulTransformerOps};
///
/// let mut counter1 = 0;
/// let transformer1 = move |x: i32| {
///     counter1 += 1;
///     x + counter1
/// };
///
/// let mut counter2 = 0;
/// let transformer2 = move |x: i32| {
///     counter2 += 1;
///     x * counter2
/// };
///
/// let mut composed = transformer1.and_then(transformer2);
/// assert_eq!(composed.apply(10), 11);  // (10 + 1) * 1
/// ```
///
/// ## Reverse composition with compose
///
/// ```rust
/// use prism3_function::{StatefulTransformer, FnStatefulTransformerOps};
///
/// let mut counter = 0;
/// let transformer = move |x: i32| {
///     counter += 1;
///     x * counter
/// };
///
/// let mut composed = transformer.compose(|x: i32| x + 1);
/// assert_eq!(composed.apply(10), 11); // (10 + 1) * 1
/// ```
///
/// ## Conditional mapping with when
///
/// ```rust
/// use prism3_function::{StatefulTransformer, FnStatefulTransformerOps};
///
/// let mut transformer = (|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// assert_eq!(transformer.apply(5), 10);
/// assert_eq!(transformer.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnStatefulTransformerOps<T, R>: FnMut(T) -> R + Sized + 'static {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new transformer that applies this transformer first, then applies
    /// the after transformer to the result. Consumes self and returns a
    /// `BoxStatefulTransformer`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after transformer
    /// * `F` - The type of the after transformer (must implement StatefulTransformer<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The transformer to apply after self. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A `BoxStatefulTransformer<R, S>`
    ///   - An `RcStatefulTransformer<R, S>`
    ///   - An `ArcStatefulTransformer<R, S>`
    ///   - Any type implementing `StatefulTransformer<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulTransformer<T, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulTransformer, FnStatefulTransformerOps, BoxStatefulTransformer};
    ///
    /// let mut counter1 = 0;
    /// let transformer1 = move |x: i32| {
    ///     counter1 += 1;
    ///     x + counter1
    /// };
    ///
    /// let mut counter2 = 0;
    /// let transformer2 = BoxStatefulTransformer::new(move |x: i32| {
    ///     counter2 += 1;
    ///     x * counter2
    /// });
    ///
    /// let mut composed = transformer1.and_then(transformer2);
    /// assert_eq!(composed.apply(10), 11);
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxStatefulTransformer<T, S>
    where
        S: 'static,
        F: StatefulTransformer<R, S> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulTransformer::new(self).and_then(after)
    }

    /// Creates a conditional transformer
    ///
    /// Returns a transformer that only executes when a predicate is satisfied.
    /// You must call `or_else()` to provide an alternative transformer for
    /// when the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. Can be:
    ///   - A closure: `|x: &T| -> bool`
    ///   - A function pointer: `fn(&T) -> bool`
    ///   - A `BoxPredicate<T>`
    ///   - An `RcPredicate<T>`
    ///   - An `ArcPredicate<T>`
    ///   - Any type implementing `Predicate<T>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalStatefulTransformer<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulTransformer, FnStatefulTransformerOps};
    ///
    /// let mut transformer = (|x: i32| x * 2)
    ///     .when(|x: &i32| *x > 0)
    ///     .or_else(|x: i32| -x);
    ///
    /// assert_eq!(transformer.apply(5), 10);
    /// assert_eq!(transformer.apply(-5), 5);
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalStatefulTransformer<T, R>
    where
        P: Predicate<T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulTransformer::new(self).when(predicate)
    }
}

/// Blanket implementation of FnStatefulTransformerOps for all closures
///
/// Automatically implements `FnStatefulTransformerOps<T, R>` for any type that
/// implements `FnMut(T) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, R, F> FnStatefulTransformerOps<T, R> for F where F: FnMut(T) -> R + 'static {}

// ============================================================================
// BoxConditionalStatefulTransformer - Box-based Conditional StatefulTransformer
// ============================================================================

/// BoxConditionalStatefulTransformer struct
///
/// A conditional transformer that only executes when a predicate is satisfied.
/// Uses `BoxStatefulTransformer` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxStatefulTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else
///   logic
/// - **Implements StatefulTransformer**: Can be used anywhere a `StatefulTransformer` is expected
///
/// # Examples
///
/// ```rust
/// use prism3_function::{StatefulTransformer, BoxStatefulTransformer};
///
/// let mut high_count = 0;
/// let mut low_count = 0;
///
/// let mut transformer = BoxStatefulTransformer::new(move |x: i32| {
///     high_count += 1;
///     x * 2
/// })
/// .when(|x: &i32| *x >= 10)
/// .or_else(move |x| {
///     low_count += 1;
///     x + 1
/// });
///
/// assert_eq!(transformer.apply(15), 30); // when branch executed
/// assert_eq!(transformer.apply(5), 6);   // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalStatefulTransformer<T, R> {
    transformer: BoxStatefulTransformer<T, R>,
    predicate: BoxPredicate<T>,
}

// Implement BoxConditionalTransformer
impl_box_conditional_transformer!(
    BoxConditionalStatefulTransformer<T, R>,
    BoxStatefulTransformer,
    StatefulTransformer
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalStatefulTransformer<T, R>);

// ============================================================================
// RcConditionalStatefulTransformer - Rc-based Conditional StatefulTransformer
// ============================================================================

/// RcConditionalStatefulTransformer struct
///
/// A single-threaded conditional transformer that only executes when a
/// predicate is satisfied. Uses `RcStatefulTransformer` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcStatefulTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalStatefulTransformer`
///
/// # Examples
///
/// ```rust
/// use prism3_function::{StatefulTransformer, RcStatefulTransformer};
///
/// let mut transformer = RcStatefulTransformer::new(|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// let mut transformer_clone = transformer.clone();
///
/// assert_eq!(transformer.apply(5), 10);
/// assert_eq!(transformer_clone.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalStatefulTransformer<T, R> {
    transformer: RcStatefulTransformer<T, R>,
    predicate: RcPredicate<T>,
}

// Implement RcConditionalStatefulTransformer
impl_shared_conditional_transformer!(
    RcConditionalStatefulTransformer<T, R>,
    RcStatefulTransformer,
    StatefulTransformer,
    into_rc,
    'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalStatefulTransformer<T, R>);

// Implement Clone for RcConditionalStatefulTransformer
impl_conditional_transformer_clone!(RcConditionalStatefulTransformer<T, R>);

// ============================================================================
// ArcConditionalStatefulTransformer - Arc-based Conditional StatefulTransformer
// ============================================================================

/// ArcConditionalStatefulTransformer struct
///
/// A thread-safe conditional transformer that only executes when a predicate
/// is satisfied. Uses `ArcStatefulTransformer` and `ArcPredicate` for shared
/// ownership across threads.
///
/// This type is typically created by calling `ArcStatefulTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send`, safe for concurrent use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else
///   logic
///
/// # Examples
///
/// ```rust
/// use prism3_function::{StatefulTransformer, ArcStatefulTransformer};
///
/// let mut transformer = ArcStatefulTransformer::new(|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// let mut transformer_clone = transformer.clone();
///
/// assert_eq!(transformer.apply(5), 10);
/// assert_eq!(transformer_clone.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalStatefulTransformer<T, R> {
    transformer: ArcStatefulTransformer<T, R>,
    predicate: ArcPredicate<T>,
}

// Implement ArcConditionalStatefulTransformer
impl_shared_conditional_transformer!(
    ArcConditionalStatefulTransformer<T, R>,
    ArcStatefulTransformer,
    StatefulTransformer,
    into_arc,
    Send + Sync + 'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(ArcConditionalStatefulTransformer<T, R>);

// Implement Clone for ArcConditionalStatefulTransformer
impl_conditional_transformer_clone!(ArcConditionalStatefulTransformer<T, R>);
