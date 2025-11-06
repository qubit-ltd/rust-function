/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Box Supplier Methods Macro
//!
//! Generates map, filter, and zip method implementations for Box-based Supplier
//!
//! Generates transformation methods for Box-based suppliers that consume self
//! (because Box cannot be cloned).
//!
//! This macro supports single-parameter suppliers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxSupplier<T>`
//! * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
//!
//! # Parameter Usage Comparison
//!
//! | Supplier Type | Struct Signature | `$supplier_trait` |
//! |---------------|-----------------|------------------|
//! | **Supplier** | `BoxSupplier<T>` | Supplier |
//! | **SupplierOnce** | `BoxSupplierOnce<T>` | SupplierOnce |
//! | **StatefulSupplier** | `BoxStatefulSupplier<T>` | StatefulSupplier |
//!
//! # Examples
//!
//! ```ignore
//! impl_box_supplier_methods!(
//!     BoxSupplier<T>,
//!     Supplier
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates map, filter, and zip method implementations for Box-based Supplier
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// Generates transformation methods for Box-based suppliers that consume self
/// (because Box cannot be cloned).
///
/// This macro supports single-parameter suppliers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxSupplier<T>`
/// * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
///
/// # Parameter Usage Comparison
///
/// | Supplier Type | Struct Signature | `$supplier_trait` |
/// |---------------|-----------------|------------------|
/// | **Supplier** | `BoxSupplier<T>` | Supplier |
/// | **SupplierOnce** | `BoxSupplierOnce<T>` | SupplierOnce |
/// | **StatefulSupplier** | `BoxStatefulSupplier<T>` | StatefulSupplier |
///
/// # Examples
///
/// ```ignore
/// // Single-parameter supplier
/// impl_box_supplier_methods!(
///     BoxSupplier<T>,
///     Supplier
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_supplier_methods {
    // Single generic parameter - Supplier
    (
        $struct_name:ident < $t:ident >,
        $supplier_trait:ident
    ) => {
        /// Maps the output using a transformation function.
        ///
        /// Consumes self and returns a new supplier that applies the
        /// mapper to each output.
        ///
        /// # Parameters
        ///
        /// * `mapper` - The transformer to apply to the output. Can be a
        ///   closure, function pointer, or any type implementing
        ///   `Transformer<T, U>`.
        ///
        /// # Returns
        ///
        /// A new mapped supplier
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_rust_function::suppliers::*;
        ///
        /// let supplier = BoxSupplier::new(|| 10);
        /// let mapped = supplier
        ///     .map(|x| x * 2)
        ///     .map(|x| x + 5);
        /// assert_eq!(mapped.get(), 25);
        /// ```
        pub fn map<U, M>(self, mapper: M) -> $struct_name<U>
        where
            M: Transformer<$t, U> + 'static,
            U: 'static,
        {
            $struct_name::new(move || mapper.apply(self.get()))
        }

        /// Filters output based on a predicate.
        ///
        /// Returns a new supplier that returns `Some(value)` if the
        /// predicate is satisfied, `None` otherwise.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to test the supplied value
        ///
        /// # Returns
        ///
        /// A new filtered supplier
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_rust_function::suppliers::*;
        ///
        /// let supplier = BoxSupplier::new(|| 42);
        /// let filtered = supplier.filter(|x| x % 2 == 0);
        ///
        /// assert_eq!(filtered.get(), Some(42));
        /// ```
        pub fn filter<P>(self, predicate: P) -> $struct_name<Option<$t>>
        where
            P: Predicate<$t> + 'static,
        {
            $struct_name::new(move || {
                let value = self.get();
                if predicate.test(&value) {
                    Some(value)
                } else {
                    None
                }
            })
        }

        /// Combines this supplier with another, producing a tuple.
        ///
        /// Consumes both suppliers and returns a new supplier that
        /// produces tuples.
        ///
        /// # Parameters
        ///
        /// * `other` - The other supplier to combine with
        ///
        /// # Returns
        ///
        /// A new supplier that produces tuples
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_rust_function::suppliers::*;
        ///
        /// let first = BoxSupplier::new(|| 42);
        /// let second = BoxSupplier::new(|| "hello");
        /// let zipped = first.zip(second);
        ///
        /// assert_eq!(zipped.get(), (42, "hello"));
        /// ```
        pub fn zip<U>(self, other: $struct_name<U>) -> $struct_name<($t, U)>
        where
            U: 'static,
        {
            $struct_name::new(move || (self.get(), other.get()))
        }
    };
}

pub (crate) use impl_box_supplier_methods;