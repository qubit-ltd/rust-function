/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # SupplierOnce Types
//!
//! Provides one-time supplier implementations that generate and
//! return values without taking any input parameters, consuming
//! themselves in the process.
//!
//! # Overview
//!
//! A **SupplierOnce** is a functional abstraction similar to
//! `Supplier`, but can only be called once. The `get()` method
//! consumes `self`, ensuring the supplier cannot be reused.
//!
//! # Key Characteristics
//!
//! - **Single use**: Can only call `get()` once
//! - **Consumes self**: The method takes ownership of `self`
//! - **Holds `FnOnce`**: Can capture and move non-cloneable values
//! - **Type-system guaranteed**: Prevents multiple calls at compile
//!   time
//!
//! # Use Cases
//!
//! 1. **Lazy initialization**: Delay expensive computation until
//!    needed
//! 2. **One-time resource consumption**: Generate value by consuming
//!    a resource
//! 3. **Move-only closures**: Hold closures that capture moved
//!    values
//!
//! # Examples
//!
//! ## Lazy Initialization
//!
//! ```rust
//! use prism3_function::{BoxSupplierOnce, SupplierOnce};
//!
//! let once = BoxSupplierOnce::new(|| {
//!     println!("Expensive initialization");
//!     42
//! });
//!
//! let value = once.get(); // Only initializes once
//! assert_eq!(value, 42);
//! ```
//!
//! ## Moving Captured Values
//!
//! ```rust
//! use prism3_function::{BoxSupplierOnce, SupplierOnce};
//!
//! let resource = String::from("data");
//! let once = BoxSupplierOnce::new(move || resource);
//!
//! let value = once.get();
//! assert_eq!(value, "data");
//! ```
//!
//! # Author
//!
//! Haixing Hu

use crate::{
    macros::box_conversions::impl_box_once_conversions,
    suppliers::macros::{
        impl_supplier_common_methods,
        impl_supplier_debug_display,
    },
};

// ==========================================================================
// SupplierOnce Trait
// ==========================================================================

/// One-time supplier trait: generates a value consuming self.
///
/// Similar to `Supplier`, but can only be called once. The `get()`
/// method consumes `self`, ensuring the supplier cannot be reused.
///
/// # Key Characteristics
///
/// - **Single use**: Can only call `get()` once
/// - **Consumes self**: The method takes ownership of `self`
/// - **Holds `FnOnce`**: Can capture and move non-cloneable values
/// - **Type-system guaranteed**: Prevents multiple calls at compile
///   time
///
/// # Use Cases
///
/// 1. **Lazy initialization**: Delay expensive computation until
///    needed
/// 2. **One-time resource consumption**: Generate value by consuming
///    a resource
/// 3. **Move-only closures**: Hold closures that capture moved
///    values
///
/// # Examples
///
/// ## Lazy Initialization
///
/// ```rust
/// use prism3_function::{BoxSupplierOnce, SupplierOnce};
///
/// let once = BoxSupplierOnce::new(|| {
///     println!("Expensive computation");
///     42
/// });
///
/// let value = once.get(); // Prints: Expensive computation
/// assert_eq!(value, 42);
/// // once is now consumed and cannot be used again
/// ```
///
/// ## Resource Consumption
///
/// ```rust
/// use prism3_function::{BoxSupplierOnce, SupplierOnce};
///
/// let resource = String::from("data");
/// let once = BoxSupplierOnce::new(move || {
///     resource // Move the resource
/// });
///
/// let value = once.get();
/// assert_eq!(value, "data");
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait SupplierOnce<T> {
    /// Generates and returns the value, consuming self.
    ///
    /// This method can only be called once because it consumes
    /// `self`. This ensures type-system level guarantee that the
    /// supplier won't be called multiple times.
    ///
    /// # Returns
    ///
    /// The generated value of type `T`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{BoxSupplierOnce, SupplierOnce};
    ///
    /// let once = BoxSupplierOnce::new(|| 42);
    /// assert_eq!(once.get(), 42);
    /// // once is consumed here
    /// ```
    fn get(self) -> T;

    /// Converts to `BoxSupplierOnce`.
    ///
    /// # Returns
    ///
    /// A new `BoxSupplierOnce<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::SupplierOnce;
    ///
    /// let closure = || 42;
    /// let boxed = closure.into_box();
    /// assert_eq!(boxed.get(), 42);
    /// ```
    fn into_box(self) -> BoxSupplierOnce<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        BoxSupplierOnce::new(move || self.get())
    }

    /// Converts the supplier to a `Box<dyn FnOnce() -> T>`.
    ///
    /// This method consumes the current supplier and wraps it in a `Box` as a
    /// trait object, allowing it to be used where a dynamically dispatched
    /// `FnOnce` is needed.
    ///
    /// # Returns
    ///
    /// A `Box<dyn FnOnce() -> T>` that executes the supplier when called.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::SupplierOnce;
    ///
    /// let closure = || 42;
    /// let fn_once = closure.into_fn();
    /// assert_eq!(fn_once(), 42);
    /// ```
    fn into_fn(self) -> impl FnOnce() -> T
    where
        Self: Sized + 'static,
        T: 'static,
    {
        move || self.get()
    }

    /// Converts the supplier to a `BoxSupplierOnce`.
    ///
    /// This is a convenience method that clones the current supplier and
    /// wraps it in a `BoxSupplierOnce`. This is useful for type erasure and
    /// creating homogenous collections of suppliers.
    ///
    /// # Returns
    ///
    /// A new `BoxSupplierOnce<T>` instance.
    ///
    /// # Note
    ///
    /// This requires the `SupplierOnce` to be `Clone` because it only
    /// borrows `&self` but must create a new owned `BoxSupplierOnce`. The
    /// clone provides the owned value needed for the new instance.
    fn to_box(&self) -> BoxSupplierOnce<T>
    where
        Self: Clone + Sized + 'static,
        T: 'static,
    {
        self.clone().into_box()
    }

    /// Converts the supplier to a `Box<dyn FnOnce() -> T>`.
    ///
    /// This method clones the current supplier and wraps it in a `Box` as a
    /// trait object, allowing it to be used where a dynamically dispatched
    /// `FnOnce` is needed.
    ///
    /// # Returns
    ///
    /// A `Box<dyn FnOnce() -> T>` that executes the supplier when called.
    ///
    /// # Note
    ///
    /// This requires the `SupplierOnce` to be `Clone` since `to_fn` only
    /// borrows `&self` but needs to produce a `FnOnce` which will be
    /// consumed. The underlying supplier is cloned to provide an owned value
    /// that the returned closure can consume.
    fn to_fn(&self) -> impl FnOnce() -> T
    where
        Self: Clone + Sized + 'static,
        T: 'static,
    {
        self.clone().into_fn()
    }
}

// ==========================================================================
// BoxSupplierOnce - One-time Supplier Implementation
// ==========================================================================

/// Box-based one-time supplier.
///
/// Uses `Box<dyn FnOnce() -> T>` for one-time value generation.
/// Can only call `get()` once, consuming the supplier.
///
/// # Examples
///
/// ## Lazy Initialization
///
/// ```rust
/// use prism3_function::{BoxSupplierOnce, SupplierOnce};
///
/// let once = BoxSupplierOnce::new(|| {
///     println!("Expensive initialization");
///     42
/// });
///
/// let value = once.get(); // Prints: Expensive initialization
/// assert_eq!(value, 42);
/// ```
///
/// ## Moving Captured Values
///
/// ```rust
/// use prism3_function::{BoxSupplierOnce, SupplierOnce};
///
/// let resource = String::from("data");
/// let once = BoxSupplierOnce::new(move || resource);
///
/// let value = once.get();
/// assert_eq!(value, "data");
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxSupplierOnce<T> {
    function: Box<dyn FnOnce() -> T>,
    name: Option<String>,
}

impl<T> BoxSupplierOnce<T> {
    // Generates: new(), new_with_name(), name(), set_name(), constant()
    impl_supplier_common_methods!(BoxSupplierOnce<T>, (FnOnce() -> T + 'static), |f| Box::new(
        f
    ));
}

// Generates: Debug and Display implementations for BoxSupplierOnce<T>
impl_supplier_debug_display!(BoxSupplierOnce<T>);

// Generates: implement SupplierOnce for BoxSupplierOnce<T>
impl<T> SupplierOnce<T> for BoxSupplierOnce<T> {
    fn get(self) -> T {
        (self.function)()
    }

    impl_box_once_conversions!(
        BoxSupplierOnce<T>,
        SupplierOnce,
        FnOnce() -> T
    );
}

// ==========================================================================
// Implement SupplierOnce for Closures
// ==========================================================================

impl<T, F> SupplierOnce<T> for F
where
    F: FnOnce() -> T,
{
    fn get(self) -> T {
        self()
    }

    fn into_box(self) -> BoxSupplierOnce<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        BoxSupplierOnce::new(self)
    }

    fn into_fn(self) -> impl FnOnce() -> T
    where
        Self: Sized + 'static,
        T: 'static,
    {
        self
    }

    fn to_box(&self) -> BoxSupplierOnce<T>
    where
        Self: Clone + Sized + 'static,
        T: 'static,
    {
        BoxSupplierOnce::new(self.clone())
    }

    fn to_fn(&self) -> impl FnOnce() -> T
    where
        Self: Clone + Sized + 'static,
        T: 'static,
    {
        self.clone()
    }
}
