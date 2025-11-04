/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Conditional Supplier Conversions Macro
//!
//! Generates conversion methods for Conditional Supplier implementations
//!
//! This macro generates the conversion methods (`into_box`, `into_rc`, `into_fn`) for
//! conditional supplier types. It handles both immutable (Supplier) and mutable
//! (StatefulSupplier) cases using the `#[allow(unused_mut)]` annotation.
//!
//! The macro works by always declaring variables as `mut`, which is necessary for
//! StatefulSupplier cases, while suppressing unused_mut warnings for Supplier cases
//! where the mutability is not needed.
//!
//! # Parameters
//!
//! * `$box_type<$t:ident>` - The box-based supplier type (e.g., `BoxSupplier<T>`)
//! * `$rc_type:ident` - The rc-based supplier type name (e.g., `RcSupplier`)
//! * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
//!
//! # Usage Examples
//!
//! For Supplier (immutable):
//! ```ignore
//! impl<T> Supplier<T> for BoxConditionalSupplier<T>
//! where
//!     T: 'static,
//! {
//!     fn get(&self) -> T {
//!         if self.predicate.test(&self.supplier.get()) {
//!             self.supplier.get()
//!         } else {
//!             // default value or something
//!             unimplemented!()
//!         }
//!     }
//!
//!     impl_conditional_supplier_conversions!(
//!         BoxSupplier<T>,
//!         RcSupplier,
//!         Fn
//!     );
//! }
//! ```
//!
//! For StatefulSupplier (mutable):
//! ```ignore
//! impl<T> StatefulSupplier<T> for BoxConditionalStatefulSupplier<T>
//! where
//!     T: 'static,
//! {
//!     fn get(&mut self) -> T {
//!         if self.predicate.test(&self.supplier.get()) {
//!             self.supplier.get()
//!         } else {
//!             // default value or something
//!             unimplemented!()
//!         }
//!     }
//!
//!     impl_conditional_supplier_conversions!(
//!         BoxStatefulSupplier<T>,
//!         RcStatefulSupplier,
//!         FnMut
//!     );
//! }
//! ```
//!
//! # Implementation Details
//!
//! - Uses `#[allow(unused_mut)]` to handle Supplier cases where `mut` is not needed
//! - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
//!   or `FnMut` based on their internal operations
//! - The `into_fn` method uses the provided `$fn_trait` parameter to match the
//!   intended trait type
//!
//! # Author
//!
//! Haixing Hu

/// Generates conversion methods for Conditional Supplier implementations
///
/// This macro should be used inside an existing impl block (typically within
/// a trait implementation block). It generates individual conversion methods
/// but does not create a complete impl block itself. This macro generates the
/// conversion methods (`into_box`, `into_rc`, `into_fn`) for conditional supplier
/// types. It handles both immutable (Supplier) and mutable (StatefulSupplier)
/// cases using the `#[allow(unused_mut)]` annotation.
///
/// The macro works by always declaring variables as `mut`, which is necessary for
/// StatefulSupplier cases, while suppressing unused_mut warnings for Supplier cases
/// where the mutability is not needed.
///
/// # Parameters
///
/// * `$box_type<$t:ident>` - The box-based supplier type (e.g., `BoxSupplier<T>`)
/// * `$rc_type:ident` - The rc-based supplier type name (e.g., `RcSupplier`)
/// * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
///
/// # Usage Examples
///
/// For Supplier (immutable):
/// ```ignore
/// impl<T> Supplier<T> for BoxConditionalSupplier<T>
/// where
///     T: 'static,
/// {
///     fn get(&self) -> T {
///         if self.predicate.test(&self.supplier.get()) {
///             self.supplier.get()
///         } else {
///             // default value or something
///             unimplemented!()
///         }
///     }
///
///     impl_conditional_supplier_conversions!(
///         BoxSupplier<T>,
///         RcSupplier,
///         Fn
///     );
/// }
/// ```
///
/// For StatefulSupplier (mutable):
/// ```ignore
/// impl<T> StatefulSupplier<T> for BoxConditionalStatefulSupplier<T>
/// where
///     T: 'static,
/// {
///     fn get(&mut self) -> T {
///         if self.predicate.test(&self.supplier.get()) {
///             self.supplier.get()
///         } else {
///             // default value or something
///             unimplemented!()
///         }
///     }
///
///     impl_conditional_supplier_conversions!(
///         BoxStatefulSupplier<T>,
///         RcStatefulSupplier,
///         FnMut
///     );
/// }
/// ```
///
/// # Implementation Details
///
/// - Uses `#[allow(unused_mut)]` to handle Supplier cases where `mut` is not needed
/// - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
///   or `FnMut` based on their internal operations
/// - The `into_fn` method uses the provided `$fn_trait` parameter to match the
///   intended trait type
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_supplier_conversions {
    // Single generic parameter - Supplier
    (
        $box_type:ident < $t:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        #[allow(unused_mut)]
        fn into_box(self) -> $box_type<$t> {
            let pred = self.predicate;
            let mut supplier = self.supplier;
            $box_type::new(move || {
                let value = supplier.get();
                if pred.test(&value) {
                    value
                } else {
                    // For conditional suppliers, we need to handle the else case
                    // This is a simplified version - in practice this might need adjustment
                    supplier.get()
                }
            })
        }

        #[allow(unused_mut)]
        fn into_rc(self) -> $rc_type<$t> {
            let pred = self.predicate.into_rc();
            let mut supplier = self.supplier.into_rc();
            let mut supplier_fn = supplier;
            $rc_type::new(move || {
                let value = supplier_fn.get();
                if pred.test(&value) {
                    value
                } else {
                    supplier_fn.get()
                }
            })
        }

        #[allow(unused_mut)]
        fn into_fn(self) -> impl $fn_trait() -> $t {
            let pred = self.predicate;
            let mut supplier = self.supplier;
            move || {
                let value = supplier.get();
                if pred.test(&value) {
                    value
                } else {
                    supplier.get()
                }
            }
        }
    };
}

pub(crate) use impl_conditional_supplier_conversions;
