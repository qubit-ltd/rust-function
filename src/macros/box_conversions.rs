/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Box Conversions Macro
//!
//! Generate common conversion methods for all Box-based function wrappers.
//!
//! This macro uses a unified pattern to generate standard conversion methods
//! for all Box-based function wrapper types (into_box, into_rc, into_fn,
//! into_once).
//!
//! # Author
//!
//! Hu Haixing

/// Implement common conversion methods for Box types
///
/// This macro generates standard conversion methods for Box-based function
/// wrappers.
///
/// # Parameters
///
/// * `$box_type<$(generics),*>` - Box wrapper type (e.g., `BoxConsumer<T>`)
/// * `$rc_type` - Corresponding Rc wrapper type (e.g., `RcConsumer`)
/// * `$fn_trait` - Function trait (e.g., `Fn(&T)`, `Fn(&T) -> bool`)
/// * `$once_type` - Corresponding once wrapper type (optional, e.g.,
///   `BoxConsumerOnce`)
///
/// # Generated methods
///
/// * `into_box(self) -> BoxType` - Zero-cost conversion, returns self
/// * `into_rc(self) -> RcType` - Convert to Rc-based wrapper
/// * `into_fn(self) -> impl FnTrait` - Extract underlying function
/// * `into_once(self) -> OnceType` - Convert to once wrapper (only when
///   once_type is provided)
///
/// # Examples
///
/// ```ignore
/// // 3-parameter version (no once type)
/// impl_box_conversions!(
///     BoxPredicate<T>,
///     RcPredicate,
///     Fn(&T) -> bool
/// );
///
/// // 4-parameter version (with once type)
/// impl_box_conversions!(
///     BoxConsumer<T>,
///     RcConsumer,
///     Fn(&T),
///     BoxConsumerOnce
/// );
/// ```
///
/// # Author
///
/// Hu Haixing

macro_rules! impl_box_conversions {
    // 3-parameter pattern: box_type, rc_type, fn_trait (no once_type)
    (
        $box_type:ident < $($generics:ident),* >,
        $rc_type:ident,
        $fn_trait:path
    ) => {
        fn into_box(self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self
        }

        fn into_rc(self) -> $rc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $rc_type::new_with_optional_name(self.function, self.name)
        }

        fn into_fn(self) -> impl $fn_trait
        {
            self.function
        }
    };

    // 4-parameter pattern: box_type, rc_type, fn_trait, once_type
    // Reuse 3-parameter version to generate into_box, into_rc, into_fn
    (
        $box_type:ident < $($generics:ident),* >,
        $rc_type:ident,
        $fn_trait:path,
        $once_type:ident
    ) => {
        // Reuse 3-parameter version to generate into_box, into_rc, into_fn
        impl_box_conversions!(
            $box_type < $($generics),* >,
            $rc_type,
            $fn_trait
        );

        fn into_once(self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $once_type::new_with_optional_name(self.function, self.name)
        }
    };
}

pub(crate) use impl_box_conversions;

/// Implement common conversion methods for Box*Once types
///
/// This macro generates standard conversion methods for all Box*Once types
/// that implement their respective traits (into_box, into_fn).
///
/// The macro unifies the pattern for both void-returning functions (like
/// Consumer, Mutator) and value-returning functions (like Function,
/// Transformer, Supplier).
///
/// # Parameters
///
/// * `$box_type_with_generics` - Box type with generics (e.g.,
///   `BoxConsumerOnce<T>`, `BoxBiConsumerOnce<T, U>`)
/// * `$trait_name` - Trait name (for documentation, unused in expansion)
/// * `$fn_trait` - Function trait type (e.g., `FnOnce(&T)`,
///   `FnOnce(&T) -> R`, `FnOnce() -> T`)
///
/// # Generated methods
///
/// * `into_box(self) -> BoxType` - Zero-cost conversion, returns self
/// * `into_fn(self) -> impl FnOnce(...)` - Extract underlying function
///
/// # Examples
///
/// ```ignore
/// // Consumer: (&T) -> ()
/// impl_box_once_conversions!(BoxConsumerOnce<T>, ConsumerOnce, FnOnce(&T));
///
/// // BiConsumer: (&T, &U) -> ()
/// impl_box_once_conversions!(BoxBiConsumerOnce<T, U>, BiConsumerOnce,
///     FnOnce(&T, &U));
///
/// // Function: (&T) -> R
/// impl_box_once_conversions!(BoxFunctionOnce<T, R>, FunctionOnce,
///     FnOnce(&T) -> R);
///
/// // Transformer: (T) -> R
/// impl_box_once_conversions!(BoxTransformerOnce<T, R>, TransformerOnce,
///     FnOnce(T) -> R);
///
/// // Supplier: () -> T
/// impl_box_once_conversions!(BoxSupplierOnce<T>, SupplierOnce, FnOnce() -> T);
/// ```
///
/// # Author
///
/// Hu Haixing
macro_rules! impl_box_once_conversions {
    (
        $box_type:ident < $($generics:ident),* >,
        $trait_name:ident,
        $fn_trait:path
    ) => {
        fn into_box(self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self
        }

        fn into_fn(self) -> impl $fn_trait
        where
            $($generics: 'static),*
        {
            self.function
        }
    };
}

pub(crate) use impl_box_once_conversions;
