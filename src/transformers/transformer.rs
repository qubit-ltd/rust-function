/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Transformer Types
//!
//! Provides Rust implementations of transformer traits for type conversion
//! and value transformation. Transformers consume input values (taking
//! ownership) and produce output values. This is analogous to
//！ `Fn(T) -> R` in Rust's standard library.
//!
//! This module provides the `Transformer<T, R>` trait and three
//! implementations:
//!
//! - [`BoxTransformer`]: Single ownership, not cloneable
//! - [`ArcTransformer`]: Thread-safe shared ownership, cloneable
//! - [`RcTransformer`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu

use std::rc::Rc;
use std::sync::Arc;

use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};
use crate::macros::impl_box_conversions;
use crate::transformers::macros::{
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
};
use crate::BoxTransformerOnce;

// ============================================================================
// Core Trait
// ============================================================================

/// Transformer trait - transforms values from type T to type R
///
/// Defines the behavior of a transformation: converting a value of type `T`
/// to a value of type `R` by consuming the input. This is analogous to
/// `Fn(T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait Transformer<T, R> {
    /// Applies the transformation to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `input` - The input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&self, input: T) -> R;

    /// Converts to BoxTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxTransformer<T, R>`
    fn into_box(self) -> BoxTransformer<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(move |x| self.apply(x))
    }

    /// Converts to RcTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcTransformer<T, R>`
    fn into_rc(self) -> RcTransformer<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        RcTransformer::new(move |x| self.apply(x))
    }

    /// Converts to ArcTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates
    /// an `ArcTransformer`. Types can override this method to provide
    /// more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcTransformer<T, R>`
    fn into_arc(self) -> ArcTransformer<T, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        ArcTransformer::new(move |x| self.apply(x))
    }

    /// Converts transformer to a closure
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes
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
    /// Returns a closure that implements `Fn(T) -> R`
    fn into_fn(self) -> impl Fn(T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |t: T| self.apply(t)
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
    /// use prism3_function::Transformer;
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
        BoxTransformerOnce::new(move |t| self.apply(t))
    }

    /// Converts to BoxTransformer without consuming self
    ///
    /// **📌 Borrows `&self`**: The original transformer remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `BoxTransformer` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxTransformer<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{ArcTransformer, Transformer};
    ///
    /// let double = ArcTransformer::new(|x: i32| x * 2);
    /// let boxed = double.to_box();
    ///
    /// // Original transformer still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(boxed.apply(21), 42);
    /// ```
    fn to_box(&self) -> BoxTransformer<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Converts to RcTransformer without consuming self
    ///
    /// **📌 Borrows `&self`**: The original transformer remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `RcTransformer` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcTransformer<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{ArcTransformer, Transformer};
    ///
    /// let double = ArcTransformer::new(|x: i32| x * 2);
    /// let rc = double.to_rc();
    ///
    /// // Original transformer still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(rc.apply(21), 42);
    /// ```
    fn to_rc(&self) -> RcTransformer<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Converts to ArcTransformer without consuming self
    ///
    /// **📌 Borrows `&self`**: The original transformer remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `ArcTransformer` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcTransformer<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{ArcTransformer, Transformer};
    ///
    /// let double = ArcTransformer::new(|x: i32| x * 2);
    /// let arc = double.to_arc();
    ///
    /// // Original transformer still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(arc.apply(21), 42);
    /// ```
    fn to_arc(&self) -> ArcTransformer<T, R>
    where
        Self: Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts transformer to a closure without consuming self
    ///
    /// **📌 Borrows `&self`**: The original transformer remains usable
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
    /// Returns a closure that implements `Fn(T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{ArcTransformer, Transformer};
    ///
    /// let double = ArcTransformer::new(|x: i32| x * 2);
    /// let closure = double.to_fn();
    ///
    /// // Original transformer still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(closure(21), 42);
    /// ```
    fn to_fn(&self) -> impl Fn(T) -> R
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }
}

// ============================================================================
// BoxTransformer - Box<dyn Fn(T) -> R>
// ============================================================================

/// BoxTransformer - transformer wrapper based on `Box<dyn Fn>`
///
/// A transformer wrapper that provides single ownership with reusable
/// transformation. The transformer consumes the input and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxTransformer<T, R> {
    function: Box<dyn Fn(T) -> R>,
    name: Option<String>,
}

// Implement BoxTransformer
impl<T, R> BoxTransformer<T, R>
where
    T: 'static,
    R: 'static,
{
    impl_transformer_common_methods!(
        BoxTransformer<T, R>,
        (Fn(T) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxTransformer<T, R>,
        BoxConditionalTransformer,
        Transformer
    );
}

// Implement constant method for BoxTransformer
impl_transformer_constant_method!(BoxTransformer<T, R>);

// Implement Debug and Display for BoxTransformer
impl_transformer_debug_display!(BoxTransformer<T, R>);

// Implement Transformer for BoxTransformer
impl<T, R> Transformer<T, R> for BoxTransformer<T, R> {
    fn apply(&self, input: T) -> R {
        (self.function)(input)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxTransformer<T, R>,
        RcTransformer,
        Fn(T) -> R,
        BoxTransformerOnce
    );
}

// ============================================================================
// RcTransformer - Rc<dyn Fn(T) -> R>
// ============================================================================

/// RcTransformer - single-threaded transformer wrapper
///
/// A single-threaded, clonable transformer wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(T) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct RcTransformer<T, R> {
    function: Rc<dyn Fn(T) -> R>,
    name: Option<String>,
}

// Implement RcTransformer
impl<T, R> RcTransformer<T, R>
where
    T: 'static,
    R: 'static,
{
    impl_transformer_common_methods!(
        RcTransformer<T, R>,
        (Fn(T) -> R + 'static),
        |f| Rc::new(f)
    );

    impl_shared_transformer_methods!(
        RcTransformer<T, R>,
        RcConditionalTransformer,
        into_rc,
        Transformer,
        'static
    );
}

impl_transformer_constant_method!(RcTransformer<T, R>);

// Implement Debug and Display for RcTransformer
impl_transformer_debug_display!(RcTransformer<T, R>);

// Implement Clone for RcTransformer
impl_transformer_clone!(RcTransformer<T, R>);

// Implement Transformer for RcTransformer
impl<T, R> Transformer<T, R> for RcTransformer<T, R> {
    fn apply(&self, input: T) -> R {
        (self.function)(input)
    }

    // RcTransformer::into_box() is implemented by the default implementation
    // of Transformer::into_box()

    fn into_box(self) -> BoxTransformer<T, R>
    where
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(move |t| (self.function)(t))
    }

    // Override with zero-cost implementation: directly return itself
    fn into_rc(self) -> RcTransformer<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self
    }

    // do NOT override RcTransformer::into_arc() because RcTransformer is not Send + Sync
    // and calling RcTransformer::into_arc() will cause a compile error

    // Override with optimized implementation: wrap the Rc in a
    // closure to avoid double indirection
    fn into_fn(self) -> impl Fn(T) -> R
    where
        T: 'static,
        R: 'static,
    {
        move |t| (self.function)(t)
    }

    // Override with optimized implementation: clone the Rc (cheap)
    fn to_box(&self) -> BoxTransformer<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        BoxTransformer::new(move |t| self_fn(t))
    }

    // Override with zero-cost implementation: clone itself
    fn to_rc(&self) -> RcTransformer<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self.clone()
    }

    // do NOT override RcTransformer::to_arc() because RcTransformer is not Send + Sync
    // and calling RcTransformer::to_arc() will cause a compile error

    // Override with optimized implementation: clone the Rc (cheap)
    fn to_fn(&self) -> impl Fn(T) -> R
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        move |t| self_fn(t)
    }
}

// ============================================================================
// ArcTransformer - Arc<dyn Fn(T) -> R + Send + Sync>
// ============================================================================

/// ArcTransformer - thread-safe transformer wrapper
///
/// A thread-safe, clonable transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(T) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct ArcTransformer<T, R> {
    function: Arc<dyn Fn(T) -> R + Send + Sync>,
    name: Option<String>,
}

// Implement ArcTransformer
impl<T, R> ArcTransformer<T, R>
where
    T: 'static,
    R: 'static,
{
    impl_transformer_common_methods!(
        ArcTransformer<T, R>,
        (Fn(T) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    impl_shared_transformer_methods!(
        ArcTransformer<T, R>,
        ArcConditionalTransformer,
        into_arc,
        Transformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcTransformer
impl_transformer_constant_method!(thread_safe ArcTransformer<T, R>);

// Implement Debug and Display for ArcTransformer
impl_transformer_debug_display!(ArcTransformer<T, R>);

// Implement Clone for ArcTransformer
impl_transformer_clone!(ArcTransformer<T, R>);

// Implement Transformer for ArcTransformer
impl<T, R> Transformer<T, R> for ArcTransformer<T, R> {
    fn apply(&self, input: T) -> R {
        (self.function)(input)
    }

    fn into_box(self) -> BoxTransformer<T, R>
    where
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(move |t| (self.function)(t))
    }

    fn into_rc(self) -> RcTransformer<T, R>
    where
        T: 'static,
        R: 'static,
    {
        RcTransformer::new(move |t| (self.function)(t))
    }

    fn into_arc(self) -> ArcTransformer<T, R>
    where
        T: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self
    }

    fn into_fn(self) -> impl Fn(T) -> R
    where
        T: 'static,
        R: 'static,
    {
        move |t| (self.function)(t)
    }

    fn to_box(&self) -> BoxTransformer<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        BoxTransformer::new(move |t| self_fn(t))
    }

    fn to_rc(&self) -> RcTransformer<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        RcTransformer::new(move |t| self_fn(t))
    }

    fn to_arc(&self) -> ArcTransformer<T, R>
    where
        T: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.clone()
    }

    fn to_fn(&self) -> impl Fn(T) -> R
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

/// Implement Transformer<T, R> for any type that implements Fn(T) -> R
///
/// This allows closures and function pointers to be used directly with our
/// Transformer trait without wrapping.
///
/// # Examples
///
/// ```rust
/// use prism3_function::Transformer;
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
impl<F, T, R> Transformer<T, R> for F
where
    F: Fn(T) -> R,
    T: 'static,
    R: 'static,
{
    fn apply(&self, input: T) -> R {
        self(input)
    }

    fn into_box(self) -> BoxTransformer<T, R>
    where
        Self: Sized + 'static,
    {
        BoxTransformer::new(self)
    }

    fn into_rc(self) -> RcTransformer<T, R>
    where
        Self: Sized + 'static,
    {
        RcTransformer::new(self)
    }

    fn into_arc(self) -> ArcTransformer<T, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        ArcTransformer::new(self)
    }

    fn into_fn(self) -> impl Fn(T) -> R
    where
        Self: Sized + 'static,
    {
        self
    }

    // use the default implementation of to_box(), to_rc(), to_arc() from
    // Transformer trait

    fn to_fn(&self) -> impl Fn(T) -> R
    where
        Self: Clone + Sized + 'static,
    {
        self.clone()
    }
}

// ============================================================================
// FnTransformerOps - Extension trait for closure transformers
// ============================================================================

/// Extension trait for closures implementing `Fn(T) -> R`
///
/// Provides composition methods (`and_then`, `compose`, `when`) for closures
/// and function pointers without requiring explicit wrapping in
/// `BoxTransformer`, `RcTransformer`, or `ArcTransformer`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `Fn(T) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `Transformer<T, R>` through blanket
/// implementation, they don't have access to instance methods like `and_then`,
/// `compose`, and `when`. This extension trait provides those methods,
/// returning `BoxTransformer` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use prism3_function::{Transformer, FnTransformerOps};
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
/// use prism3_function::{Transformer, FnTransformerOps};
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
/// use prism3_function::{Transformer, FnTransformerOps};
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
pub trait FnTransformerOps<T, R>: Fn(T) -> R + Sized + 'static {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new transformer that applies this transformer first, then
    /// applies the after transformer to the result. Consumes self and returns
    /// a `BoxTransformer`.
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
    /// A new `BoxTransformer<T, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use prism3_function::{Transformer, FnTransformerOps, BoxTransformer};
    ///
    /// let double = |x: i32| x * 2;
    /// let to_string = BoxTransformer::new(|x: i32| x.to_string());
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
    /// use prism3_function::{Transformer, FnTransformerOps, BoxTransformer};
    ///
    /// let double = |x: i32| x * 2;
    /// let to_string = BoxTransformer::new(|x: i32| x.to_string());
    ///
    /// // Clone to preserve original
    /// let composed = double.and_then(to_string.clone());
    /// assert_eq!(composed.apply(21), "42");
    ///
    /// // Original still usable
    /// assert_eq!(to_string.apply(5), "5");
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxTransformer<T, S>
    where
        S: 'static,
        F: Transformer<R, S> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(move |x: T| after.apply(self(x)))
    }

    /// Reverse composition - applies before first, then self
    ///
    /// Creates a new transformer that applies the before transformer first,
    /// then applies this transformer to the result. Consumes self and returns
    /// a `BoxTransformer`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The input type of the before transformer
    /// * `F` - The type of the before transformer (must implement Transformer<S, T>)
    ///
    /// # Parameters
    ///
    /// * `before` - The transformer to apply before self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original transformer, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: S| -> T`
    ///   - A function pointer: `fn(S) -> T`
    ///   - A `BoxTransformer<S, T>`
    ///   - An `RcTransformer<S, T>`
    ///   - An `ArcTransformer<S, T>`
    ///   - Any type implementing `Transformer<S, T>`
    ///
    /// # Returns
    ///
    /// A new `BoxTransformer<S, R>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use prism3_function::{Transformer, FnTransformerOps, BoxTransformer};
    ///
    /// let double = |x: i32| x * 2;
    /// let add_one = BoxTransformer::new(|x: i32| x + 1);
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
    /// use prism3_function::{Transformer, FnTransformerOps, BoxTransformer};
    ///
    /// let double = |x: i32| x * 2;
    /// let add_one = BoxTransformer::new(|x: i32| x + 1);
    ///
    /// // Clone to preserve original
    /// let composed = double.compose(add_one.clone());
    /// assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    ///
    /// // Original still usable
    /// assert_eq!(add_one.apply(3), 4);
    /// ```
    fn compose<S, F>(self, before: F) -> BoxTransformer<S, R>
    where
        S: 'static,
        F: Transformer<S, T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(move |x: S| self(before.apply(x)))
    }

    /// Creates a conditional transformer
    ///
    /// Returns a transformer that only executes when a predicate is satisfied.
    /// You must call `or_else()` to provide an alternative transformer for when
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
    /// Returns `BoxConditionalTransformer<T, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use prism3_function::{Transformer, FnTransformerOps};
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
    /// use prism3_function::{Transformer, FnTransformerOps, BoxPredicate};
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
    fn when<P>(self, predicate: P) -> BoxConditionalTransformer<T, R>
    where
        P: Predicate<T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(self).when(predicate)
    }
}

/// Blanket implementation of FnTransformerOps for all closures
///
/// Automatically implements `FnTransformerOps<T, R>` for any type that
/// implements `Fn(T) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, R, F> FnTransformerOps<T, R> for F where F: Fn(T) -> R + 'static {}

// ============================================================================
// UnaryOperator Trait - Marker trait for Transformer<T, T>
// ============================================================================

/// UnaryOperator trait - marker trait for unary operators
///
/// A unary operator transforms a value of type `T` to another value of the
/// same type `T`. This trait extends `Transformer<T, T>` to provide semantic
/// clarity for same-type transformations. Equivalent to Java's `UnaryOperator<T>`
/// which extends `Function<T, T>`.
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// `Transformer<T, T>`, so you don't need to implement it manually.
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
/// use prism3_function::{UnaryOperator, Transformer};
///
/// fn apply_twice<T, O>(value: T, op: O) -> T
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
/// use prism3_function::{BoxUnaryOperator, UnaryOperator, Transformer};
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
pub trait UnaryOperator<T>: Transformer<T, T> {}

/// Blanket implementation of UnaryOperator for all Transformer<T, T>
///
/// This automatically implements `UnaryOperator<T>` for any type that
/// implements `Transformer<T, T>`.
///
/// # Author
///
/// Haixing Hu
impl<F, T> UnaryOperator<T> for F
where
    F: Transformer<T, T>,
    T: 'static,
{
    // empty
}

// ============================================================================
// Type Aliases for UnaryOperator (Transformer<T, T>)
// ============================================================================

/// Type alias for `BoxTransformer<T, T>`
///
/// Represents a unary operator that transforms a value of type `T` to another
/// value of the same type `T`, with single ownership semantics. Equivalent to
/// Java's `UnaryOperator<T>`.
///
/// # Examples
///
/// ```rust
/// use prism3_function::{BoxUnaryOperator, Transformer};
///
/// let increment: BoxUnaryOperator<i32> = BoxUnaryOperator::new(|x| x + 1);
/// assert_eq!(increment.apply(41), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxUnaryOperator<T> = BoxTransformer<T, T>;

/// Type alias for `ArcTransformer<T, T>`
///
/// Represents a thread-safe unary operator that transforms a value of type `T`
/// to another value of the same type `T`. Equivalent to Java's `UnaryOperator<T>`
/// with shared, thread-safe ownership.
///
/// # Examples
///
/// ```rust
/// use prism3_function::{ArcUnaryOperator, Transformer};
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
pub type ArcUnaryOperator<T> = ArcTransformer<T, T>;

/// Type alias for `RcTransformer<T, T>`
///
/// Represents a single-threaded unary operator that transforms a value of type
/// `T` to another value of the same type `T`. Equivalent to Java's
/// `UnaryOperator<T>` with shared, single-threaded ownership.
///
/// # Examples
///
/// ```rust
/// use prism3_function::{RcUnaryOperator, Transformer};
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
pub type RcUnaryOperator<T> = RcTransformer<T, T>;

// ============================================================================
// BoxConditionalTransformer - Box-based Conditional Transformer
// ============================================================================

/// BoxConditionalTransformer struct
///
/// A conditional transformer that only executes when a predicate is satisfied.
/// Uses `BoxTransformer` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements Transformer**: Can be used anywhere a `Transformer` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use prism3_function::{Transformer, BoxTransformer};
///
/// let double = BoxTransformer::new(|x: i32| x * 2);
/// let negate = BoxTransformer::new(|x: i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// assert_eq!(conditional.apply(5), 10); // when branch executed
/// assert_eq!(conditional.apply(-5), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalTransformer<T, R> {
    transformer: BoxTransformer<T, R>,
    predicate: BoxPredicate<T>,
}

// Implement BoxConditionalTransformer
impl_box_conditional_transformer!(
    BoxConditionalTransformer<T, R>,
    BoxTransformer,
    Transformer
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalTransformer<T, R>);

// ============================================================================
// RcConditionalTransformer - Rc-based Conditional Transformer
// ============================================================================

/// RcConditionalTransformer struct
///
/// A single-threaded conditional transformer that only executes when a
/// predicate is satisfied. Uses `RcTransformer` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalTransformer`
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Transformer, RcTransformer};
///
/// let double = RcTransformer::new(|x: i32| x * 2);
/// let identity = RcTransformer::<i32, i32>::identity();
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
pub struct RcConditionalTransformer<T, R> {
    transformer: RcTransformer<T, R>,
    predicate: RcPredicate<T>,
}

// Implement RcConditionalTransformer
impl_shared_conditional_transformer!(
    RcConditionalTransformer<T, R>,
    RcTransformer,
    Transformer,
    into_rc,
    'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalTransformer<T, R>);

// Implement Clone for RcConditionalTransformer
impl_conditional_transformer_clone!(RcConditionalTransformer<T, R>);

// ============================================================================
// ArcConditionalTransformer - Arc-based Conditional Transformer
// ============================================================================

/// ArcConditionalTransformer struct
///
/// A thread-safe conditional transformer that only executes when a predicate is
/// satisfied. Uses `ArcTransformer` and `ArcPredicate` for shared ownership
/// across threads.
///
/// This type is typically created by calling `ArcTransformer::when()` and is
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
/// use prism3_function::{Transformer, ArcTransformer};
///
/// let double = ArcTransformer::new(|x: i32| x * 2);
/// let identity = ArcTransformer::<i32, i32>::identity();
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
pub struct ArcConditionalTransformer<T, R> {
    transformer: ArcTransformer<T, R>,
    predicate: ArcPredicate<T>,
}

// Implement ArcConditionalTransformer
impl_shared_conditional_transformer!(
    ArcConditionalTransformer<T, R>,
    ArcTransformer,
    Transformer,
    into_arc,
    Send + Sync + 'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(ArcConditionalTransformer<T, R>);

// Implement Clone for ArcConditionalTransformer
impl_conditional_transformer_clone!(ArcConditionalTransformer<T, R>);
