/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Rc Conversions Macro
//!
//! Generates common into_xxx() conversion methods for all Rc-based function
//! wrappers.
//!
//! This macro generates the standard conversion methods (`into_box`, `into_rc`,
//! `into_fn`, `into_once`) for all Rc-based function wrapper types using a
//! single unified pattern.
//!
//! # Author
//!
//! Haixing Hu

/// Public interface macro for Rc-based conversions.
///
/// This macro automatically infers everything from the function signature:
/// - Number of parameters
/// - Parameter types
/// - Return type
/// - Call mode (Fn → direct, FnMut → borrow_mut)
///
/// # Syntax
///
/// ```ignore
/// // 2-parameter version (no once type, for predicates and similar pure functions)
/// impl_rc_conversions!(
///     RcType<Generics>,           // Rc wrapper type with all generic parameters
///     BoxType,                     // Corresponding Box wrapper type
///     Fn(args) [-> RetType]        // Fn or FnMut signature (auto-infers everything!)
/// );
///
/// // 3-parameter version (with once type, for consumers, functions, etc.)
/// impl_rc_conversions!(
///     RcType<Generics>,           // Rc wrapper type with all generic parameters
///     BoxType,                     // Corresponding Box wrapper type
///     OnceType,                    // Corresponding once wrapper type
///     Fn(args) [-> RetType]        // Fn or FnMut signature (auto-infers everything!)
/// );
/// ```
///
/// # Generated methods
///
/// * `into_box(self) -> BoxType` - Converts to Box-based wrapper
/// * `into_rc(self) -> RcType` - Converts to Rc-based wrapper
/// * `into_fn(self) -> impl FnTrait` - Converts to function pointer
/// * `into_once(self) -> OnceType` - Converts to once wrapper
/// * `to_box(&self) -> BoxType` - Converts to Box-based wrapper
/// * `to_rc(&self) -> RcType` - Converts to Rc-based wrapper
/// * `to_fn(&self) -> impl FnTrait` - Converts to function pointer
/// * `to_once(&self) -> OnceType` - Converts to once wrapper
///
/// # Examples
///
/// ```ignore
/// // Predicate: Fn(&T) -> bool → direct call mode (no once type)
/// impl_rc_conversions!(RcPredicate<T>, BoxPredicate, Fn(t: &T) -> bool);
///
/// // BiPredicate: Fn(&T, &U) -> bool → direct call mode (no once type)
/// impl_rc_conversions!(RcBiPredicate<T, U>, BoxBiPredicate, Fn(t: &T, u: &U) -> bool);
///
/// // Consumer: Fn(&T) → direct call mode (with once type)
/// impl_rc_conversions!(RcConsumer<T>, BoxConsumer, BoxConsumerOnce, Fn(t: &T));
///
/// // StatefulConsumer: FnMut(&T) → borrow_mut call mode (with once type)
/// impl_rc_conversions!(RcStatefulConsumer<T>, BoxStatefulConsumer, BoxConsumerOnce, FnMut(t: &T));
///
/// // BiConsumer: Fn(&T, &U) → direct call mode (with once type)
/// impl_rc_conversions!(RcBiConsumer<T, U>, BoxBiConsumer, BoxBiConsumerOnce, Fn(t: &T, u: &U));
///
/// // Function: Fn(&T) -> R → direct call mode (with once type)
/// impl_rc_conversions!(RcFunction<T, R>, BoxFunction, BoxFunctionOnce, Fn(t: &T) -> R);
///
/// // StatefulFunction: FnMut(&T) -> R → borrow_mut call mode (with once type)
/// impl_rc_conversions!(RcStatefulFunction<T, R>, BoxStatefulFunction, BoxFunctionOnce, FnMut(t: &T) -> R);
///
/// // MutatingFunction: Fn(&mut T) -> R → direct call mode (with once type)
/// impl_rc_conversions!(RcMutatingFunction<T, R>, BoxMutatingFunction, BoxMutatingFunctionOnce, Fn(input: &mut T) -> R);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_rc_conversions {
    // ==================== Core Macro: Generate Single Method ====================

    // Helper: Generate a single conversion method (consuming self)
    (
        @method_into
        $method_name:ident,                              // Method name: into_box, into_once
        $rc_type:ident < $($generics:ident),* >,        // Rc type with generics
        $target_type:ident,                              // Target type: BoxType or OnceType
        $call_mode:ident,                                // direct or borrow_mut
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?   // Function signature
    ) => {
        #[inline]
        fn $method_name(self) -> $target_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $target_type::new_with_optional_name(
                impl_rc_conversions!(@make_closure $call_mode, self.function, $($arg),*),
                self.name
            )
        }
    };

    // Helper: Generate a single conversion method (borrowing &self)
    (
        @method_to
        $method_name:ident,                              // Method name: to_box, to_once
        $rc_type:ident < $($generics:ident),* >,        // Rc type with generics
        $target_type:ident,                              // Target type: BoxType or OnceType
        $call_mode:ident,                                // direct or borrow_mut
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?   // Function signature
    ) => {
        #[inline]
        fn $method_name(&self) -> $target_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            let self_fn = self.function.clone();
            let self_name = self.name.clone();
            $target_type::new_with_optional_name(
                impl_rc_conversions!(@make_closure $call_mode, self_fn, $($arg),*),
                self_name
            )
        }
    };

    // Helper: Generate into_fn method (consuming self, no return type, direct)
    (
        @fn_method_into
        direct,
        ($($arg:ident : $arg_ty:ty),*)
    ) => {
        #[inline]
        fn into_fn(self) -> impl Fn($($arg_ty),*)
        {
            move |$($arg),*| (self.function)($($arg),*)
        }
    };

    // Helper: Generate into_fn method (consuming self, with return type, direct)
    (
        @fn_method_into
        direct,
        ($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        #[inline]
        fn into_fn(self) -> impl Fn($($arg_ty),*) -> $ret
        {
            move |$($arg),*| (self.function)($($arg),*)
        }
    };

    // Helper: Generate into_fn method (consuming self, no return type, borrow_mut)
    (
        @fn_method_into
        borrow_mut,
        ($($arg:ident : $arg_ty:ty),*)
    ) => {
        #[inline]
        fn into_fn(self) -> impl FnMut($($arg_ty),*)
        {
            move |$($arg),*| (self.function.borrow_mut())($($arg),*)
        }
    };

    // Helper: Generate into_fn method (consuming self, with return type, borrow_mut)
    (
        @fn_method_into
        borrow_mut,
        ($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        #[inline]
        fn into_fn(self) -> impl FnMut($($arg_ty),*) -> $ret
        {
            move |$($arg),*| (self.function.borrow_mut())($($arg),*)
        }
    };

    // Helper: Generate to_fn method (borrowing &self, no return type, direct)
    (
        @fn_method_to
        direct,
        ($($arg:ident : $arg_ty:ty),*)
    ) => {
        #[inline]
        fn to_fn(&self) -> impl Fn($($arg_ty),*)
        {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn)($($arg),*)
        }
    };

    // Helper: Generate to_fn method (borrowing &self, with return type, direct)
    (
        @fn_method_to
        direct,
        ($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        #[inline]
        fn to_fn(&self) -> impl Fn($($arg_ty),*) -> $ret
        {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn)($($arg),*)
        }
    };

    // Helper: Generate to_fn method (borrowing &self, no return type, borrow_mut)
    (
        @fn_method_to
        borrow_mut,
        ($($arg:ident : $arg_ty:ty),*)
    ) => {
        #[inline]
        fn to_fn(&self) -> impl FnMut($($arg_ty),*)
        {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn.borrow_mut())($($arg),*)
        }
    };

    // Helper: Generate to_fn method (borrowing &self, with return type, borrow_mut)
    (
        @fn_method_to
        borrow_mut,
        ($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        #[inline]
        fn to_fn(&self) -> impl FnMut($($arg_ty),*) -> $ret
        {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn.borrow_mut())($($arg),*)
        }
    };

    // Helper: Make closure based on call mode
    (@make_closure direct, $fn_call:expr, $($arg:ident),*) => {
        move |$($arg),*| ($fn_call)($($arg),*)
    };
    (@make_closure borrow_mut, $fn_call:expr, $($arg:ident),*) => {
        move |$($arg),*| ($fn_call.borrow_mut())($($arg),*)
    };

    // ==================== Main Implementation ====================

    // Internal implementation: Generate common methods (shared by both variants)
    (
        @impl_common
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        // into_box: consumes self, returns Box
        impl_rc_conversions!(
            @method_into into_box,
            $rc_type<$($generics),*>, $box_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // into_rc: consumes self, returns self (zero-cost)
        #[inline]
        fn into_rc(self) -> $rc_type<$($generics),*>
        {
            self
        }

        // into_fn: consumes self, returns impl Fn/FnMut
        impl_rc_conversions!(
            @fn_method_into
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // to_box: borrows self, clones and returns Box
        impl_rc_conversions!(
            @method_to to_box,
            $rc_type<$($generics),*>, $box_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // to_rc: borrows self, returns clone (cheap Rc clone)
        #[inline]
        fn to_rc(&self) -> $rc_type<$($generics),*>
        {
            self.clone()
        }

        // to_fn: borrows self, clones and returns impl Fn/FnMut
        impl_rc_conversions!(
            @fn_method_to
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };

    // Internal implementation: Generate all methods (with once type)
    (
        @impl
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $once_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        // Generate common methods
        impl_rc_conversions!(
            @impl_common
            $rc_type<$($generics),*>,
            $box_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // into_once: consumes self, returns Once
        impl_rc_conversions!(
            @method_into into_once,
            $rc_type<$($generics),*>,
            $once_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // to_once: borrows self, clones and returns Once
        impl_rc_conversions!(
            @method_to to_once,
            $rc_type<$($generics),*>,
            $once_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };

    // Internal implementation: Generate methods without once type
    (
        @impl_no_once
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        // Generate common methods only
        impl_rc_conversions!(
            @impl_common
            $rc_type<$($generics),*>,
            $box_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };

    // ==================== Public Interface ====================

    // Fn(...) → direct call mode (immutable, no interior mutability) - no once type
    (
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        Fn($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl_rc_conversions!(
            @impl_no_once
            $rc_type<$($generics),*>,
            $box_type,
            direct,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };

    // FnMut(...) → borrow_mut call mode (mutable, needs RefCell/Mutex) - no once type
    (
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        FnMut($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl_rc_conversions!(
            @impl_no_once
            $rc_type<$($generics),*>,
            $box_type,
            borrow_mut,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };

    // Fn(...) → direct call mode (immutable, no interior mutability) - with once type
    (
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $once_type:ident,
        Fn($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl_rc_conversions!(
            @impl
            $rc_type<$($generics),*>,
            $box_type,
            $once_type,
            direct,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };

    // FnMut(...) → borrow_mut call mode (mutable, needs RefCell/Mutex) - with once type
    (
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $once_type:ident,
        FnMut($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl_rc_conversions!(
            @impl
            $rc_type<$($generics),*>,
            $box_type,
            $once_type,
            borrow_mut,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };
}

pub(crate) use impl_rc_conversions;
