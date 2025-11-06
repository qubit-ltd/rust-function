/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Supplier Methods Macro
//!
//! Generates map, filter, zip method implementations for Arc/Rc-based Supplier
//!
//! Generates transformation methods for Arc/Rc-based suppliers that borrow &self
//! (because Arc/Rc can be cloned).
//!
//! This macro supports single-parameter suppliers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `ArcSupplier<T>`
//! * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
//! * `($extra_bounds)` - Extra trait bounds in parentheses ('static for Rc, Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Supplier Type | Struct Signature | `$supplier_trait` | `($extra_bounds)` |
//! |---------------|------------------|-------------------|------------------|
//! | **ArcSupplier** | `ArcSupplier<T>` | Supplier | (Send + Sync + 'static) |
//! | **RcSupplier** | `RcSupplier<T>` | Supplier | ('static) |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter with Arc
//! impl_shared_supplier_methods!(
//!     ArcSupplier<T>,
//!     Supplier,
//!     (Send + Sync + 'static)
//! );
//!
//! // Single-parameter with Rc
//! impl_shared_supplier_methods!(
//!     RcSupplier<T>,
//!     Supplier,
//!     ('static)
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates map, filter, zip method implementations for Arc/Rc-based Supplier
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates transformation methods for Arc/Rc-based suppliers
/// that borrow &self (because Arc/Rc can be cloned).
///
/// This macro supports single-parameter suppliers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `ArcSupplier<T>`
/// * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
/// * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Supplier Type | Struct Signature | `$supplier_trait` | `$extra_bounds` |
/// |---------------|------------------|-------------------|----------------|
/// | **ArcSupplier** | `ArcSupplier<T>` | Supplier | Send + Sync + 'static |
/// | **RcSupplier** | `RcSupplier<T>` | Supplier | 'static |
///
/// # Examples
///
/// ```ignore
/// // Single-parameter with Arc
/// impl_shared_supplier_methods!(
///     ArcSupplier<T>,
///     Supplier,
///     Send + Sync + 'static
/// );
///
/// // Single-parameter with Rc
/// impl_shared_supplier_methods!(
///     RcSupplier<T>,
///     Supplier,
///     'static
/// );
/// ```
/// # Author
///
/// Haixing Hu
macro_rules! impl_shared_supplier_methods {
    // Single generic parameter
    (
        $struct_name:ident < $t:ident >,
        $supplier_trait:ident,
        ($($extra_bounds:tt)*)
    ) => {
        /// Maps the output using a transformation function.
        ///
        /// # Parameters
        ///
        /// * `mapper` - The transformation function to apply
        ///
        /// # Returns
        ///
        /// A new `$struct_name<U>` with the mapped output
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::{$struct_name, $supplier_trait};
        ///
        /// let source = $struct_name::new(|| 10);
        /// let mapped = source.map(|x| x * 2);
        /// // source is still usable
        /// assert_eq!(mapped.get(), 20);
        /// ```
        #[allow(unused_mut)]
        pub fn map<U, M>(&self, mapper: M) -> $struct_name<U>
        where
            M: Transformer<$t, U> + $($extra_bounds)+,
            U: $($extra_bounds)+,
        {
            let mut self_cloned = self.clone();
            $struct_name::new(move || {
                let value = self_cloned.get();
                mapper.apply(value)
            })
        }

        /// Filters output based on a predicate.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to test the supplied value
        ///
        /// # Returns
        ///
        /// A new filtered `$struct_name<Option<$t>>`
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::{$struct_name, $supplier_trait};
        ///
        /// let source = $struct_name::new(|| 42);
        /// let filtered = source.filter(|x| x % 2 == 0);
        ///
        /// assert_eq!(filtered.get(), Some(42));
        /// ```
        #[allow(unused_mut)]
        pub fn filter<P>(&self, predicate: P) -> $struct_name<Option<$t>>
        where
            P: Predicate<$t> + $($extra_bounds)+,
        {
            let mut self_cloned = self.clone();
            $struct_name::new(move || {
                let value = self_cloned.get();
                if predicate.test(&value) {
                    Some(value)
                } else {
                    None
                }
            })
        }

        /// Combines this supplier with another, producing a tuple.
        ///
        /// # Parameters
        ///
        /// * `other` - The other supplier to combine with
        ///
        /// # Returns
        ///
        /// A new `$struct_name<($t, U)>`
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_function::{$struct_name, $supplier_trait};
        ///
        /// let first = $struct_name::new(|| 42);
        /// let second = $struct_name::new(|| "hello");
        ///
        /// let zipped = first.zip(second);
        ///
        /// assert_eq!(zipped.get(), (42, "hello"));
        /// ```
        #[allow(unused_mut)]
        pub fn zip<U, S>(&self, mut other: S) -> $struct_name<($t, U)>
        where
            S: $supplier_trait<U> + $($extra_bounds)+,
            U: $($extra_bounds)+,
        {
            let mut self_cloned = self.clone();
            $struct_name::new(move || {
                let first = self_cloned.get();
                let second = other.get();
                (first, second)
            })
        }
    };
}

pub(crate) use impl_shared_supplier_methods;
