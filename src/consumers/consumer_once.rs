/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # ConsumerOnce Types
//!
//! Provides implementations of one-time consumer interfaces for executing one-time operations
//! that accept a single input parameter but return no result.
//!
//! It is similar to the `FnOnce(&T)` trait in the standard library.
//!
//! This module provides a unified `ConsumerOnce` trait and one concrete implementation:
//!
//! - **`BoxConsumerOnce<T>`**: Box-based single ownership implementation
//!
//! # Why No Arc/Rc Variants?
//!
//! Unlike `Consumer` and `ReadonlyConsumer`, this module does **not** provide `ArcConsumerOnce`
//! or `RcConsumerOnce` implementations. This is a design decision based on the fact that
//! `FnOnce` semantics are fundamentally incompatible with shared ownership. See design docs for details.
//!
//! # Design Philosophy
//!
//! ConsumerOnce uses `FnOnce(&T)` semantics for truly one-time consumption operations.
//!
//! Unlike Consumer, ConsumerOnce consumes itself on first call. Suitable for initialization
//! callbacks, cleanup callbacks, and similar scenarios.
//!
//! # Author
//!
//! Haixing Hu

use crate::consumers::macros::{
    impl_box_conditional_consumer,
    impl_box_consumer_methods,
    impl_conditional_consumer_debug_display,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};

// ============================================================================
// 1. ConsumerOnce Trait - Unified ConsumerOnce Interface
// ============================================================================

/// ConsumerOnce trait - Unified one-time consumer interface
///
/// It is similar to the `FnOnce(&T)` trait in the standard library.
///
/// Defines the core behavior of all one-time consumer types. Similar to consumers
/// implementing `FnOnce(&T)`, executes operations that accept a value reference but
/// return no result (only side effects), consuming itself in the process.
///
/// # Automatic Implementation
///
/// - All closures implementing `FnOnce(&T)`
/// - `BoxConsumerOnce<T>`
///
/// # Features
///
/// - **Unified Interface**: All consumer types share the same `accept` method signature
/// - **Automatic Implementation**: Closures automatically implement this trait with zero overhead
/// - **Type Conversion**: Can be converted to BoxConsumerOnce
/// - **Generic Programming**: Write functions that work with any one-time consumer type
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// fn apply_consumer<C: ConsumerOnce<i32>>(consumer: C, value: &i32) {
///     consumer.accept(value);
/// }
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let box_con = BoxConsumerOnce::new(move |x: &i32| {
///     l.lock().unwrap().push(*x);
/// });
/// apply_consumer(box_con, &5);
/// assert_eq!(*log.lock().unwrap(), vec![5]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait ConsumerOnce<T> {
    /// Execute one-time consumption operation
    ///
    /// Executes an operation on the given reference. The operation typically reads
    /// the input value or produces side effects, but does not modify the input
    /// value itself. Consumes self.
    ///
    /// # Parameters
    ///
    /// * `value` - Reference to the value to be consumed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
    ///
    /// let consumer = BoxConsumerOnce::new(|x: &i32| println!("{}", x));
    /// consumer.accept(&5);
    /// ```
    fn accept(self, value: &T);

    /// Convert to BoxConsumerOnce
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `BoxConsumerOnce` by calling
    /// `accept` on the consumer. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxConsumerOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// };
    /// let box_consumer = closure.into_box();
    /// box_consumer.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// ```
    fn into_box(self) -> BoxConsumerOnce<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        BoxConsumerOnce::new(move |t| self.accept(t))
    }

    /// Convert to closure
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after calling this method.
    ///
    /// Converts a one-time consumer to a closure that can be used directly in places
    /// where the standard library requires `FnOnce`.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self` and calls
    /// its `accept` method. Types can override this method to provide more efficient
    /// conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `FnOnce(&T)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x * 2);
    /// };
    /// let func = closure.into_fn();
    /// func(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![10]);
    /// ```
    fn into_fn(self) -> impl FnOnce(&T)
    where
        Self: Sized + 'static,
    {
        move |t| self.accept(t)
    }

    /// Convert to BoxConsumerOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement
    /// `Clone`. Clones the current consumer and wraps it in a
    /// `BoxConsumerOnce`.
    ///
    /// # Default Implementation
    ///
    /// The default implementation clones `self` and then calls
    /// `into_box()` on the clone. Types can override this method to
    /// provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxConsumerOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// };
    /// let box_consumer = closure.to_box();
    /// box_consumer.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// ```
    fn to_box(&self) -> BoxConsumerOnce<T>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
    {
        self.clone().into_box()
    }

    /// Convert to closure without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement
    /// `Clone`. Clones the current consumer and then converts the clone
    /// to a closure.
    ///
    /// # Default Implementation
    ///
    /// The default implementation clones `self` and then calls
    /// `into_fn()` on the clone. Types can override this method to
    /// provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `FnOnce(&T)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x * 2);
    /// };
    /// let func = closure.to_fn();
    /// func(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![10]);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&T)
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }
}

// ============================================================================
// 2. BoxConsumerOnce - Single Ownership Implementation
// ============================================================================

/// BoxConsumerOnce struct
///
/// One-time consumer implementation based on `Box<dyn FnOnce(&T)>` for single ownership scenarios.
/// This is the simplest consumer type for truly one-time use.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, transfers ownership on use
/// - **Zero Overhead**: No reference counting or lock overhead
/// - **One-time Use**: Consumes self on first call
/// - **Builder Pattern**: Method chaining naturally consumes `self`
///
/// # Use Cases
///
/// Choose `BoxConsumerOnce` when:
/// - Consumer is truly used only once
/// - Building pipelines where ownership flows naturally
/// - Consumer captures values that should be consumed
/// - Performance critical and cannot accept shared overhead
///
/// # Performance
///
/// `BoxConsumerOnce` has the best performance:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
///
/// let consumer = BoxConsumerOnce::new(|x: &i32| {
///     println!("Value: {}", x);
/// });
/// consumer.accept(&5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConsumerOnce<T> {
    function: Box<dyn FnOnce(&T)>,
    name: Option<String>,
}

// All methods require T: 'static because Box<dyn FnOnce(&T)> requires it
impl<T> BoxConsumerOnce<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(BoxConsumerOnce<T>, (FnOnce(&T) + 'static), |f| Box::new(f));

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(BoxConsumerOnce<T>, BoxConditionalConsumerOnce, ConsumerOnce);
}

impl<T> ConsumerOnce<T> for BoxConsumerOnce<T> {
    fn accept(self, value: &T) {
        (self.function)(value)
    }

    impl_box_once_conversions!(BoxConsumerOnce<T>, ConsumerOnce, FnOnce(&T));
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxConsumerOnce<T>);

// ============================================================================
// 3. Implement ConsumerOnce trait for closures
// ============================================================================

// Implement ConsumerOnce for all FnOnce(&T) using macro
impl_closure_once_trait!(
    ConsumerOnce<T>,
    accept,
    BoxConsumerOnce,
    FnOnce(value: &T)
);

// ============================================================================
// 4. Extension methods for closures
// ============================================================================

/// Extension trait providing one-time consumer composition methods for closures
///
/// Provides `and_then` and other composition methods for all closures implementing `FnOnce(&T)`,
/// allowing closures to chain methods directly without explicit wrapper types.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxConsumerOnce**: Composed results can continue chaining
/// - **Zero Cost**: No overhead when composing closures
/// - **Automatic Implementation**: All `FnOnce(&T)` closures automatically get these methods
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ConsumerOnce, FnConsumerOnceOps};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let chained = (move |x: &i32| {
///     l1.lock().unwrap().push(*x * 2);
/// }).and_then(move |x: &i32| {
///     l2.lock().unwrap().push(*x + 10);
/// });
/// chained.accept(&5);
/// assert_eq!(*log.lock().unwrap(), vec![10, 15]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnConsumerOnceOps<T>: FnOnce(&T) + Sized {
    /// Sequentially chain another one-time consumer
    ///
    /// Returns a new consumer that executes the current operation first, then the next operation.
    /// Consumes the current closure and returns `BoxConsumerOnce<T>`.
    ///
    /// # Type Parameters
    ///
    /// * `C` - Type of the next consumer
    ///
    /// # Parameters
    ///
    /// * `next` - Consumer to execute after the current operation. **Note: This
    ///   parameter is passed by value and will transfer ownership.** Since
    ///   `BoxConsumerOnce` cannot be cloned, the parameter will be consumed.
    ///   Can be:
    ///   - A closure: `|x: &T|`
    ///   - A `BoxConsumerOnce<T>`
    ///   - Any type implementing `ConsumerOnce<T>`
    ///
    /// # Returns
    ///
    /// Returns a combined `BoxConsumerOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ConsumerOnce, FnConsumerOnceOps};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l1 = log.clone();
    /// let l2 = log.clone();
    /// let chained = (move |x: &i32| {
    ///     l1.lock().unwrap().push(*x * 2);
    /// }).and_then(move |x: &i32| {
    ///     l2.lock().unwrap().push(*x + 10);
    /// }).and_then(|x: &i32| println!("Result: {}", x));
    ///
    /// chained.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![10, 15]);
    /// ```
    fn and_then<C>(self, next: C) -> BoxConsumerOnce<T>
    where
        Self: 'static,
        C: ConsumerOnce<T> + 'static,
        T: 'static,
    {
        let first = self;
        let second = next;
        BoxConsumerOnce::new(move |t| {
            first(t);
            second.accept(t);
        })
    }
}

/// Implement FnConsumerOnceOps for all closure types
impl<T, F> FnConsumerOnceOps<T> for F where F: FnOnce(&T) {}

// ============================================================================
// 5. BoxConditionalConsumerOnce - Box-based Conditional Consumer
// ============================================================================

/// BoxConditionalConsumerOnce struct
///
/// A conditional one-time consumer that only executes when a predicate is satisfied.
/// Uses `BoxConsumerOnce` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxConsumerOnce::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements ConsumerOnce**: Can be used anywhere a `ConsumerOnce` is expected
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let consumer = BoxConsumerOnce::new(move |x: &i32| {
///     l.lock().unwrap().push(*x);
/// });
/// let conditional = consumer.when(|x: &i32| *x > 0);
///
/// conditional.accept(&5);
/// assert_eq!(*log.lock().unwrap(), vec![5]); // Executed
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let consumer = BoxConsumerOnce::new(move |x: &i32| {
///     l1.lock().unwrap().push(*x);
/// })
/// .when(|x: &i32| *x > 0)
/// .or_else(move |x: &i32| {
///     l2.lock().unwrap().push(-*x);
/// });
///
/// consumer.accept(&5);
/// assert_eq!(*log.lock().unwrap(), vec![5]); // when branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalConsumerOnce<T> {
    consumer: BoxConsumerOnce<T>,
    predicate: BoxPredicate<T>,
}

// Generate and_then and or_else methods using macro
impl_box_conditional_consumer!(BoxConditionalConsumerOnce<T>, BoxConsumerOnce, ConsumerOnce);

impl<T> ConsumerOnce<T> for BoxConditionalConsumerOnce<T> {
    fn accept(self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }

    fn into_box(self) -> BoxConsumerOnce<T>
    where
        T: 'static,
    {
        let pred = self.predicate;
        let consumer = self.consumer;
        BoxConsumerOnce::new(move |t| {
            if pred.test(t) {
                consumer.accept(t);
            }
        })
    }

    fn into_fn(self) -> impl FnOnce(&T) {
        let pred = self.predicate;
        let consumer = self.consumer;
        move |t: &T| {
            if pred.test(t) {
                consumer.accept(t);
            }
        }
    }

    // do NOT override ConsumerOnce::to_xxxx() because BoxConditionalConsumerOnce is not Clone
    // and calling BoxConditionalConsumerOnce::to_xxxx() will cause a compile error
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalConsumerOnce<T>);
