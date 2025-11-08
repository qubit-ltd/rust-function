////////////////////////////////////////////////////////////////////////////////
//
/*
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 */

////////////////////////////////////////////////////////////////////////////////

//! # Arc Conversions Macro
//!
//! Generates common into_xxx() conversion methods for all Arc-based function
//! wrappers.
//!
//! This macro automatically infers everything from the function signature:
//! - Number of parameters
//! - Parameter types
//! - Return type
//! - Call mode (Fn → direct, FnMut → lock_unwrap)
//!
//! # Author
//!
//! Haixing Hu

/// Public interface macro for Arc-based conversions.
///
/// This macro automatically infers everything from the function signature:
/// - Number of parameters
/// - Parameter types
/// - Return type
/// - Call mode (Fn → direct, FnMut → lock_unwrap)
///
/// # Syntax
///
/// ```ignore
/// impl_arc_conversions!(
///     ArcType<Generics>,           // Arc wrapper type with all generic params
///     BoxType,                     // Corresponding Box wrapper type
///     RcType,                      // Corresponding Rc wrapper type
///     OnceType,                    // Corresponding once wrapper type
///     Fn(args) [-> RetType]        // Fn or FnMut signature (auto-infers everything!)
/// );
/// ```
///
/// # Examples
///
/// ```ignore
/// // Consumer: Fn(&T) → direct call mode
/// impl_arc_conversions!(ArcConsumer<T>, BoxConsumer, RcConsumer,
///                       BoxConsumerOnce, Fn(t: &T));
///
/// // StatefulConsumer: FnMut(&T) → lock_unwrap call mode
/// impl_arc_conversions!(ArcStatefulConsumer<T>, BoxStatefulConsumer,
///                       RcStatefulConsumer, BoxConsumerOnce, FnMut(t: &T));
///
/// // BiConsumer: Fn(&T, &U) → direct call mode
/// impl_arc_conversions!(ArcBiConsumer<T, U>, BoxBiConsumer, RcBiConsumer,
///                       BoxBiConsumerOnce, Fn(t: &T, u: &U));
///
/// // Function: Fn(&T) -> R → direct call mode
/// impl_arc_conversions!(ArcFunction<T, R>, BoxFunction, RcFunction,
///                       BoxFunctionOnce, Fn(t: &T) -> R);
///
/// // StatefulFunction: FnMut(&T) -> R → lock_unwrap call mode
/// impl_arc_conversions!(ArcStatefulFunction<T, R>, BoxStatefulFunction,
///                       RcStatefulFunction, BoxFunctionOnce, FnMut(t: &T) -> R);
///
/// // MutatingFunction: Fn(&mut T) -> R → direct call mode
/// impl_arc_conversions!(ArcMutatingFunction<T, R>, BoxMutatingFunction,
///                       RcMutatingFunction, BoxMutatingFunctionOnce,
///                       Fn(input: &mut T) -> R);
/// ```
///
/// # Author
///
/// Haixing Hu

macro_rules! impl_arc_conversions {
    // ==================== Core Macro: Generate Single Method ====================

    // Helper: Generate a single conversion method (consuming self) - to Box
    (
        @method_into_box
        $arc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        fn into_box(self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $box_type::new_with_optional_name(
                impl_arc_conversions!(@make_closure $call_mode, self.function,
                                      $($arg),*),
                self.name
            )
        }
    };

    // Helper: Generate a single conversion method (consuming self) - to Rc
    (
        @method_into_rc
        $arc_type:ident < $($generics:ident),* >,
        $rc_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        fn into_rc(self) -> $rc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $rc_type::new_with_optional_name(
                impl_arc_conversions!(@make_closure $call_mode, self.function,
                                      $($arg),*),
                self.name
            )
        }
    };

    // Helper: Generate a single conversion method (consuming self) - to Once
    (
        @method_into_once
        $arc_type:ident < $($generics:ident),* >,
        $once_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        fn into_once(self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $once_type::new_with_optional_name(
                impl_arc_conversions!(@make_closure $call_mode, self.function,
                                      $($arg),*),
                self.name
            )
        }
    };

    // Helper: Generate a single conversion method (borrowing &self) - to Box
    (
        @method_to_box
        $arc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        fn to_box(&self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            let self_fn = self.function.clone();
            let self_name = self.name.clone();
            $box_type::new_with_optional_name(
                impl_arc_conversions!(@make_closure $call_mode, self_fn,
                                      $($arg),*),
                self_name
            )
        }
    };

    // Helper: Generate a single conversion method (borrowing &self) - to Rc
    (
        @method_to_rc
        $arc_type:ident < $($generics:ident),* >,
        $rc_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        fn to_rc(&self) -> $rc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            let self_fn = self.function.clone();
            let self_name = self.name.clone();
            $rc_type::new_with_optional_name(
                impl_arc_conversions!(@make_closure $call_mode, self_fn,
                                      $($arg),*),
                self_name
            )
        }
    };

    // Helper: Generate a single conversion method (borrowing &self) - to Once
    (
        @method_to_once
        $arc_type:ident < $($generics:ident),* >,
        $once_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        fn to_once(&self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            let self_fn = self.function.clone();
            let self_name = self.name.clone();
            $once_type::new_with_optional_name(
                impl_arc_conversions!(@make_closure $call_mode, self_fn,
                                      $($arg),*),
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
        fn into_fn(self) -> impl Fn($($arg_ty),*) -> $ret
        {
            move |$($arg),*| (self.function)($($arg),*)
        }
    };

    // Helper: Generate into_fn method (consuming self, no return type, lock_unwrap)
    (
        @fn_method_into
        lock_unwrap,
        ($($arg:ident : $arg_ty:ty),*)
    ) => {
        fn into_fn(self) -> impl FnMut($($arg_ty),*)
        {
            move |$($arg),*| (self.function.lock().unwrap())($($arg),*)
        }
    };

    // Helper: Generate into_fn method (consuming self, with return type,
    // lock_unwrap)
    (
        @fn_method_into
        lock_unwrap,
        ($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        fn into_fn(self) -> impl FnMut($($arg_ty),*) -> $ret
        {
            move |$($arg),*| (self.function.lock().unwrap())($($arg),*)
        }
    };

    // Helper: Generate to_fn method (borrowing &self, no return type, direct)
    (
        @fn_method_to
        direct,
        ($($arg:ident : $arg_ty:ty),*)
    ) => {
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
        fn to_fn(&self) -> impl Fn($($arg_ty),*) -> $ret
        {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn)($($arg),*)
        }
    };

    // Helper: Generate to_fn method (borrowing &self, no return type, lock_unwrap)
    (
        @fn_method_to
        lock_unwrap,
        ($($arg:ident : $arg_ty:ty),*)
    ) => {
        fn to_fn(&self) -> impl FnMut($($arg_ty),*)
        {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn.lock().unwrap())($($arg),*)
        }
    };

    // Helper: Generate to_fn method (borrowing &self, with return type,
    // lock_unwrap)
    (
        @fn_method_to
        lock_unwrap,
        ($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        fn to_fn(&self) -> impl FnMut($($arg_ty),*) -> $ret
        {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn.lock().unwrap())($($arg),*)
        }
    };

    // Helper: Make closure based on call mode
    (@make_closure direct, $fn_call:expr, $($arg:ident),*) => {
        move |$($arg),*| ($fn_call)($($arg),*)
    };
    (@make_closure lock_unwrap, $fn_call:expr, $($arg:ident),*) => {
        move |$($arg),*| ($fn_call.lock().unwrap())($($arg),*)
    };

    // ==================== Main Implementation ====================

    // Internal implementation: Generate all methods
    (
        @impl
        $arc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $rc_type:ident,
        $once_type:ident,
        $call_mode:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        // into_box: consumes self, returns Box
        impl_arc_conversions!(
            @method_into_box
            $arc_type<$($generics),*>, $box_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // into_rc: consumes self, returns Rc
        impl_arc_conversions!(
            @method_into_rc
            $arc_type<$($generics),*>, $rc_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // into_arc: consumes self, returns self (zero-cost)
        fn into_arc(self) -> $arc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self
        }

        // into_fn: consumes self, returns impl Fn/FnMut
        impl_arc_conversions!(
            @fn_method_into
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // into_once: consumes self, returns Once
        impl_arc_conversions!(
            @method_into_once
            $arc_type<$($generics),*>, $once_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // to_box: borrows self, clones and returns Box
        impl_arc_conversions!(
            @method_to_box
            $arc_type<$($generics),*>, $box_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // to_rc: borrows self, clones and returns Rc
        impl_arc_conversions!(
            @method_to_rc
            $arc_type<$($generics),*>, $rc_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // to_arc: borrows self, returns clone (cheap Arc clone)
        fn to_arc(&self) -> $arc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self.clone()
        }

        // to_fn: borrows self, clones and returns impl Fn/FnMut
        impl_arc_conversions!(
            @fn_method_to
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );

        // to_once: borrows self, clones and returns Once
        impl_arc_conversions!(
            @method_to_once
            $arc_type<$($generics),*>, $once_type,
            $call_mode,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };

    // ==================== Public Interface ====================

    // Fn(...) → direct call mode (immutable, no interior mutability)
    (
        $arc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $rc_type:ident,
        $once_type:ident,
        Fn($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl_arc_conversions!(
            @impl
            $arc_type<$($generics),*>,
            $box_type,
            $rc_type,
            $once_type,
            direct,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };

    // FnMut(...) → lock_unwrap call mode (mutable, needs Mutex)
    (
        $arc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $rc_type:ident,
        $once_type:ident,
        FnMut($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl_arc_conversions!(
            @impl
            $arc_type<$($generics),*>,
            $box_type,
            $rc_type,
            $once_type,
            lock_unwrap,
            ($($arg : $arg_ty),*) $(-> $ret)?
        );
    };
}

pub(crate) use impl_arc_conversions;
