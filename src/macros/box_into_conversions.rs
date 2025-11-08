/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Box Into Conversions Macro
//!
//! Generates common into_xxx() conversion methods for all Box-based function wrappers.
//!
//! This macro generates the standard conversion methods (`into_box`, `into_rc`, `into_fn`, `into_once`)
//! for all Box-based function wrapper types using a single unified pattern.
//!
//! # Author
//!
//! Haixing Hu

/// Implements common into_xxx() conversion methods for all Box-based function wrappers.
///
/// This macro generates the standard conversion methods (`into_box`, `into_rc`, `into_fn`)
/// for all Box-based function wrapper types using a single unified pattern.
/// Optionally, if a once type is provided, it also generates the `into_once` method.
///
/// # Parameters
///
/// * `$box_type<$(generics),*>` - The Box wrapper type (e.g., `BoxConsumer<T>`)
/// * `$rc_type` - The corresponding Rc wrapper type (e.g., `RcConsumer`)
/// * `$fn_type:ty` - The complete function type (e.g., `impl Fn(&T)`, `impl Fn(&T) -> R`)
/// * `$once_type` - The corresponding once wrapper type (e.g., `BoxConsumerOnce`) - optional
///
/// # Generated Methods
///
/// * `into_box(self) -> BoxType` - Zero-cost conversion, returns self
/// * `into_rc(self) -> RcType` - Converts to Rc-based wrapper
/// * `into_fn(self) -> FnType` - Extracts the underlying function
/// * `into_once(self) -> OnceType` - Converts to once-based wrapper (only when once_type is provided)
///
/// # Examples
///
/// ```ignore
/// // For Consumer types (with once type)
/// impl_box_into_conversions!(
///     BoxConsumer<T>,
///     RcConsumer,
///     BoxConsumerOnce,
///     impl Fn(&T)
/// );
///
/// // For Function types (with once type)
/// impl_box_into_conversions!(
///     BoxFunction<T, R>,
///     RcFunction,
///     BoxFunctionOnce,
///     impl Fn(&T) -> R
/// );
///
/// // For types without once conversion (3 parameters)
/// impl_box_into_conversions!(
///     BoxStatefulConsumer<T>,
///     RcStatefulConsumer,
///     impl FnMut(&T)
/// );
/// ```
///
/// # Author
///
/// Haixing Hu

macro_rules! impl_box_into_conversions {
    // Pattern for 3 parameters: box_type, rc_type, fn_type (no once_type)
    (
        $box_type:ident < $($generics:ident),* >,
        $rc_type:ident,
        $fn_type:ty
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

        fn into_fn(self) -> $fn_type
        {
            self.function
        }
    };

    // Pattern for 4 parameters: box_type, rc_type, once_type, fn_type
    // Uses the 3-parameter version plus into_once
    (
        $box_type:ident < $($generics:ident),* >,
        $rc_type:ident,
        $once_type:ident,
        $fn_type:ty
    ) => {
        // Use the 3-parameter version to get into_box, into_rc, into_fn
        impl_box_into_conversions!(
            $box_type < $($generics),* >,
            $rc_type,
            $fn_type
        );

        fn into_once(self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $once_type::new_with_optional_name(self.function, self.name)
        }
    };
}

pub(crate) use impl_box_into_conversions;
