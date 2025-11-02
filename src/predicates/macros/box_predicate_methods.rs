/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Box Predicate Methods Macro
//!
//! Generates logical operation method implementations for Box-based Predicate
//!
//! Generates basic logical operations (and, or, not, nand, xor, nor) for Box-based
//! predicates that consume self (because Box cannot be cloned).
//!
//! This macro supports both single-parameter and two-parameter predicates through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxPredicate<T>`
//!   - Two parameters: `BoxBiPredicate<T, U>`
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter predicate
//! impl_box_predicate_methods!(BoxPredicate<T>);
//!
//! // Two-parameter predicate
//! impl_box_predicate_methods!(BoxBiPredicate<T, U>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates logical operation method implementations for Box-based Predicate
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// Generates basic logical operations (and, or, not, nand, xor, nor) for Box-based
/// predicates that consume self (because Box cannot be cloned).
///
/// This macro supports both single-parameter and two-parameter predicates through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxPredicate<T>`
///   - Two parameters: `BoxBiPredicate<T, U>`
///
/// # Examples
///
/// ```ignore
/// // Single-parameter predicate
/// impl_box_predicate_methods!(BoxPredicate<T>);
///
/// // Two-parameter predicate
/// impl_box_predicate_methods!(BoxBiPredicate<T, U>);
/// ```

macro_rules! impl_box_predicate_methods {
    // Internal macro for generating logical operations
    (@logical_ops $struct_name:ident < $t:ident >, $trait_name:ident) => {
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
        pub fn and<P>(self, other: P) -> $struct_name<$t>
        where
            P: $trait_name<$t> + 'static,
            $t: 'static,
        {
            $struct_name::new(move |x| (self.function)(x) && other.test(x))
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
        pub fn or<P>(self, other: P) -> $struct_name<$t>
        where
            P: $trait_name<$t> + 'static,
            $t: 'static,
        {
            $struct_name::new(move |x| (self.function)(x) || other.test(x))
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
        pub fn not(self) -> $struct_name<$t>
        where
            $t: 'static,
        {
            $struct_name::new(move |x| !(self.function)(x))
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
        pub fn nand<P>(self, other: P) -> $struct_name<$t>
        where
            P: $trait_name<$t> + 'static,
            $t: 'static,
        {
            $struct_name::new(move |x| !((self.function)(x) && other.test(x)))
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
        pub fn xor<P>(self, other: P) -> $struct_name<$t>
        where
            P: $trait_name<$t> + 'static,
            $t: 'static,
        {
            $struct_name::new(move |x| (self.function)(x) ^ other.test(x))
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
        pub fn nor<P>(self, other: P) -> $struct_name<$t>
        where
            P: $trait_name<$t> + 'static,
            $t: 'static,
        {
            $struct_name::new(move |x| !((self.function)(x) || other.test(x)))
        }
    };

    // Two parameter version
    (@logical_ops $struct_name:ident < $t:ident, $u:ident >, $trait_name:ident) => {
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
        pub fn and<P>(self, other: P) -> $struct_name<$t, $u>
        where
            P: $trait_name<$t, $u> + 'static,
            $t: 'static,
            $u: 'static,
        {
            $struct_name::new(move |x, y| (self.function)(x, y) && other.test(x, y))
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
        pub fn or<P>(self, other: P) -> $struct_name<$t, $u>
        where
            P: $trait_name<$t, $u> + 'static,
            $t: 'static,
            $u: 'static,
        {
            $struct_name::new(move |x, y| (self.function)(x, y) || other.test(x, y))
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
        pub fn not(self) -> $struct_name<$t, $u>
        where
            $t: 'static,
            $u: 'static,
        {
            $struct_name::new(move |x, y| !(self.function)(x, y))
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
        pub fn nand<P>(self, other: P) -> $struct_name<$t, $u>
        where
            P: $trait_name<$t, $u> + 'static,
            $t: 'static,
            $u: 'static,
        {
            $struct_name::new(move |x, y| !((self.function)(x, y) && other.test(x, y)))
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
        pub fn xor<P>(self, other: P) -> $struct_name<$t, $u>
        where
            P: $trait_name<$t, $u> + 'static,
            $t: 'static,
            $u: 'static,
        {
            $struct_name::new(move |x, y| (self.function)(x, y) ^ other.test(x, y))
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
        pub fn nor<P>(self, other: P) -> $struct_name<$t, $u>
        where
            P: $trait_name<$t, $u> + 'static,
            $t: 'static,
            $u: 'static,
        {
            $struct_name::new(move |x, y| !((self.function)(x, y) || other.test(x, y)))
        }
    };

    // Single generic parameter - Predicate
    ($struct_name:ident < $t:ident >) => {
        impl_box_predicate_methods!(@logical_ops $struct_name<$t>, Predicate);
    };

    // Two generic parameters - BiPredicate
    ($struct_name:ident < $t:ident, $u:ident >) => {
        impl_box_predicate_methods!(@logical_ops $struct_name<$t, $u>, BiPredicate);
    };
}

pub(crate) use impl_box_predicate_methods;
