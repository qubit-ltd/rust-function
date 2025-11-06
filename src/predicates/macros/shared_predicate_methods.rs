/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Predicate Methods Macro
//!
//! Generates when and and_then method implementations for Arc/Rc-based Predicate
//!
//! Generates conditional execution when method and logical combination and_then method
//! for Arc/Rc-based predicates that borrow &self (because Arc/Rc can be cloned).
//!
//! This macro supports both single-parameter and two-parameter predicates through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `ArcPredicate<T>`
//!   - Two parameters: `ArcBiPredicate<T, U>`
//! * `$predicate_trait_name` - Predicate trait name (e.g., Predicate, BiPredicate)
//! * `$predicate_extra_bounds` - Extra trait bounds (e.g., 'static for Rc,
//!   Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Predicate Type | Struct Signature | `$predicate_trait_name` | `$predicate_extra_bounds` |
//! |----------------|------------------|----------------|------------------------|-------------------|----------------|
//! | **BoxPredicate** | `BoxPredicate<T>` | Predicate | 'static |
//! | **ArcPredicate** | `ArcPredicate<T>` | Predicate | Send + Sync + 'static |
//! | **RcPredicate** | `RcPredicate<T>` | Predicate | 'static |
//! | **ArcBiPredicate** | `ArcBiPredicate<T, U>` | BiPredicate | Send + Sync + 'static |
//! | **RcBiPredicate** | `RcBiPredicate<T, U>` | BiPredicate | 'static |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter with Box
//! impl_shared_predicate_methods!(BoxPredicate<T>, 'static);
//! // Single-parameter with Arc
//! impl_shared_predicate_methods!(ArcPredicate<T>, Send + Sync + 'static);
//!
//! // Two-parameter with Rc
//! impl_shared_predicate_methods!(RcBiPredicate<T, U>, 'static);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Arc/Rc-based Predicate
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates conditional execution when method and logical
/// combination and_then method for Arc/Rc-based predicates that borrow &self
/// (because Arc/Rc can be cloned).
///
/// This macro supports both single-parameter and two-parameter predicates through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `ArcPredicate<T>`
///   - Two parameters: `ArcBiPredicate<T, U>`
/// * `$predicate_trait_name` - Predicate trait name (e.g., Predicate, BiPredicate)
/// * `$predicate_extra_bounds` - Extra trait bounds (e.g., 'static for Rc,
///   Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Predicate Type | Struct Signature | `$predicate_trait_name` | `$predicate_extra_bounds` |
/// |----------------|------------------|----------------|------------------------|-------------------|----------------|
/// | **BoxPredicate** | `BoxPredicate<T>` | Predicate | 'static |
/// | **ArcPredicate** | `ArcPredicate<T>` | Predicate | Send + Sync + 'static |
/// | **RcPredicate** | `RcPredicate<T>` | Predicate | 'static |
/// | **ArcBiPredicate** | `ArcBiPredicate<T, U>` | BiPredicate | Send + Sync + 'static |
/// | **RcBiPredicate** | `RcBiPredicate<T, U>` | BiPredicate | 'static |
///
/// # Examples
///
/// ```ignore
/// // Single-parameter with Arc
/// impl_shared_predicate_methods!(ArcPredicate<T>, Send + Sync + 'static);
/// // Single-parameter with Box
/// impl_shared_predicate_methods!(BoxPredicate<T>, 'static);
///
/// // Two-parameter with Arc
/// impl_shared_predicate_methods!(ArcBiPredicate<T, U>, Send + Sync + 'static);
/// // Two-parameter with Rc
/// impl_shared_predicate_methods!(RcBiPredicate<T, U>, 'static);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_shared_predicate_methods {
    // Internal macro for generating logical operations
    (
        @logical_ops
        $struct_name:ident < $t:ident >,
        $predicate_trait_name:ident,
        $($predicate_extra_bounds:tt)+
    ) => {
        /// Returns a predicate that represents the logical AND of this predicate
        /// and another.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new predicate representing the logical AND.
        pub fn and<P>(&self, other: P) -> $struct_name<$t>
        where
            P: $predicate_trait_name<$t> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x| self_fn(x) && other.test(x))
        }

        /// Returns a predicate that represents the logical OR of this predicate
        /// and another.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new predicate representing the logical OR.
        pub fn or<P>(&self, other: P) -> $struct_name<$t>
        where
            P: $predicate_trait_name<$t> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x| self_fn(x) || other.test(x))
        }

        /// Returns a predicate that represents the logical negation of this
        /// predicate.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Returns
        ///
        /// A new predicate representing the logical negation.
        #[allow(clippy::should_implement_trait)]
        pub fn not(&self) -> $struct_name<$t>
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x| !(self_fn(x)))
        }

        /// Returns a predicate that represents the logical NAND (NOT AND) of this
        /// predicate and another.
        ///
        /// NAND returns `true` unless both predicates are `true`.
        /// Equivalent to `!(self AND other)`.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new predicate representing the logical NAND.
        pub fn nand<P>(&self, other: P) -> $struct_name<$t>
        where
            P: $predicate_trait_name<$t> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x| !(self_fn(x) && other.test(x)))
        }

        /// Returns a predicate that represents the logical XOR (exclusive OR) of
        /// this predicate and another.
        ///
        /// XOR returns `true` if exactly one of the predicates is `true`.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new predicate representing the logical XOR.
        pub fn xor<P>(&self, other: P) -> $struct_name<$t>
        where
            P: $predicate_trait_name<$t> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x| self_fn(x) ^ other.test(x))
        }

        /// Returns a predicate that represents the logical NOR (NOT OR) of this
        /// predicate and another.
        ///
        /// NOR returns `true` only when both predicates are `false`.
        /// Equivalent to `!(self OR other)`.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new predicate representing the logical NOR.
        pub fn nor<P>(&self, other: P) -> $struct_name<$t>
        where
            P: $predicate_trait_name<$t> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x| !(self_fn(x) || other.test(x)))
        }
    };

    // Two parameter version
    (
        @logical_ops
        $struct_name:ident < $t:ident, $u:ident >,
        $predicate_trait_name:ident,
        $($predicate_extra_bounds:tt)+
    ) => {
        /// Returns a bi-predicate that represents the logical AND of this
        /// bi-predicate and another.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other bi-predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new bi-predicate representing the logical AND.
        pub fn and<P>(&self, other: P) -> $struct_name<$t, $u>
        where
            P: $predicate_trait_name<$t, $u> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x, y| self_fn(x, y) && other.test(x, y))
        }

        /// Returns a bi-predicate that represents the logical OR of this
        /// bi-predicate and another.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other bi-predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new bi-predicate representing the logical OR.
        pub fn or<P>(&self, other: P) -> $struct_name<$t, $u>
        where
            P: $predicate_trait_name<$t, $u> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x, y| self_fn(x, y) || other.test(x, y))
        }

        /// Returns a bi-predicate that represents the logical negation of
        /// this bi-predicate.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Returns
        ///
        /// A new bi-predicate representing the logical negation.
        #[allow(clippy::should_implement_trait)]
        pub fn not(&self) -> $struct_name<$t, $u>
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x, y| !(self_fn(x, y)))
        }

        /// Returns a bi-predicate that represents the logical NAND (NOT
        /// AND) of this bi-predicate and another.
        ///
        /// NAND returns `true` unless both bi-predicates are `true`.
        /// Equivalent to `!(self AND other)`.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other bi-predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new bi-predicate representing the logical NAND.
        pub fn nand<P>(&self, other: P) -> $struct_name<$t, $u>
        where
            P: $predicate_trait_name<$t, $u> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x, y| !(self_fn(x, y) && other.test(x, y)))
        }

        /// Returns a bi-predicate that represents the logical XOR
        /// (exclusive OR) of this bi-predicate and another.
        ///
        /// XOR returns `true` if exactly one of the bi-predicates is
        /// `true`.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other bi-predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new bi-predicate representing the logical XOR.
        pub fn xor<P>(&self, other: P) -> $struct_name<$t, $u>
        where
            P: $predicate_trait_name<$t, $u> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x, y| self_fn(x, y) ^ other.test(x, y))
        }

        /// Returns a bi-predicate that represents the logical NOR (NOT OR)
        /// of this bi-predicate and another.
        ///
        /// NOR returns `true` only when both bi-predicates are `false`.
        /// Equivalent to `!(self OR other)`.
        ///
        /// This method consumes `self` due to single-ownership semantics.
        ///
        /// # Parameters
        ///
        /// * `other` - The other bi-predicate to combine with.
        ///
        /// # Returns
        ///
        /// A new bi-predicate representing the logical NOR.
        pub fn nor<P>(&self, other: P) -> $struct_name<$t, $u>
        where
            P: $predicate_trait_name<$t, $u> + $($predicate_extra_bounds)+
        {
            let self_fn = self.function.clone();
            $struct_name::new(move |x, y| !(self_fn(x, y) || other.test(x, y)))
        }
    };

    // Single generic parameter - Predicate
    (
        $struct_name:ident < $t:ident >,
        $($predicate_extra_bounds:tt)+
    ) => {
        impl_shared_predicate_methods!(
            @logical_ops $struct_name<$t>,
            Predicate,
            $($predicate_extra_bounds)+,
        );
    };

    // Two generic parameters - BiPredicate
    (
        $struct_name:ident < $t:ident, $u:ident >,
        $($predicate_extra_bounds:tt)+
    ) => {
        impl_shared_predicate_methods!(
            @logical_ops $struct_name<$t, $u>,
            BiPredicate,
            $($predicate_extra_bounds)+,
        );
    };

}

pub(crate) use impl_shared_predicate_methods;
