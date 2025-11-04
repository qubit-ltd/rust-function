/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Shared Transformer Methods Macro
//!
//! Generates when and and_then method implementations for Arc/Rc-based Transformer
//
//! Generates conditional execution when method and chaining and_then method
//! for Arc/Rc-based transformers that borrow &self (because Arc/Rc can be cloned).
//!
//! This macro supports both two-parameter and three-parameter transformers through
//! pattern matching on the struct signature.
//!
//! # Parameters
//
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Two parameters: `ArcTransformer<T, U>`
//!   - Three parameters: `ArcBiTransformer<T, U, V>`
//! * `$return_type` - The return type for when (e.g., ArcConditionalTransformer)
//! * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
//! * `$transformer_trait` - Transformer trait name (e.g., Transformer, BiTransformer)
//! * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Transformer Type | Struct Signature | `$return_type` |
//! |------------------|------------------|----------------|
//! | **ArcTransformer** | `ArcTransformer<T, U>` | ArcConditionalTransformer |
//! | **RcTransformer** | `RcTransformer<T, U>` | RcConditionalTransformer |
//! | **ArcStatefulTransformer** | `ArcStatefulTransformer<T, U>` | ArcConditionalStatefulTransformer |
//! | **RcStatefulTransformer** | `RcStatefulTransformer<T, U>` | RcConditionalStatefulTransformer |
//! | **ArcBiTransformer** | `ArcBiTransformer<T, U, V>` | ArcConditionalBiTransformer |
//! | **RcBiTransformer** | `RcBiTransformer<T, U, V>` | RcConditionalBiTransformer |
//! | **ArcStatefulBiTransformer** | `ArcStatefulBiTransformer<T, U, V>` | ArcConditionalStatefulBiTransformer |
//! | **RcStatefulBiTransformer** | `RcStatefulBiTransformer<T, U, V>` | RcConditionalStatefulBiTransformer |
//!
//! | `$predicate_conversion` | `$transformer_trait` | `$extra_bounds` |
//! |-------------------------|---------------------|----------------|
//! | into_arc | Transformer | Send + Sync + 'static |
//! | into_rc | Transformer | 'static |
//! | into_arc | StatefulTransformer | Send + Sync + 'static |
//! | into_rc | StatefulTransformer | 'static |
//! | into_arc | BiTransformer | Send + Sync + 'static |
//! | into_rc | BiTransformer | 'static |
//! | into_arc | StatefulBiTransformer | Send + Sync + 'static |
//! | into_rc | StatefulBiTransformer | 'static |
//
//! # Examples
//!
//! ```ignore
//! // Two-parameter with Arc
//! impl_shared_transformer_methods!(
//!     ArcTransformer<T, U>,
//!     ArcConditionalTransformer,
//!     into_arc,
//!     Transformer,
//!     Send + Sync + 'static
//! );
//!
//! // Three-parameter with Rc
//! impl_shared_transformer_methods!(
//!     RcBiTransformer<T, U, V>,
//!     RcConditionalBiTransformer,
//!     into_rc,
//!     BiTransformer,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Arc/Rc-based Transformer
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates conditional execution when method and chaining
/// and_then method for Arc/Rc-based transformers that borrow &self (because Arc/Rc
/// can be cloned).
///
/// This macro supports both two-parameter and three-parameter transformers through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Two parameters: `ArcTransformer<T, U>`
///   - Three parameters: `ArcBiTransformer<T, U, V>`
/// * `$return_type` - The return type for when (e.g., ArcConditionalTransformer)
/// * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
/// * `$transformer_trait` - Transformer trait name (e.g., Transformer, BiTransformer)
/// * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Transformer Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$transformer_trait` | `$extra_bounds` |
// |------------------|------------------|----------------|-------------------------|---------------------|----------------|
// | **ArcTransformer** | `ArcTransformer<T, U>` | ArcConditionalTransformer | into_arc | Transformer | Send + Sync + 'static |
// | **RcTransformer** | `RcTransformer<T, U>` | RcConditionalTransformer | into_rc | Transformer | 'static |
// | **ArcStatefulTransformer** | `ArcStatefulTransformer<T, U>` | ArcConditionalStatefulTransformer | into_arc | StatefulTransformer | Send + Sync + 'static |
// | **RcStatefulTransformer** | `RcStatefulTransformer<T, U>` | RcConditionalStatefulTransformer | into_rc | StatefulTransformer | 'static |
// | **ArcBiTransformer** | `ArcBiTransformer<T, U, V>` | ArcConditionalBiTransformer | into_arc | BiTransformer | Send + Sync + 'static |
// | **RcBiTransformer** | `RcBiTransformer<T, U, V>` | RcConditionalBiTransformer | into_rc | BiTransformer | 'static |
// | **ArcStatefulBiTransformer** | `ArcStatefulBiTransformer<T, U, V>` | ArcConditionalStatefulBiTransformer | into_arc | StatefulBiTransformer | Send + Sync + 'static |
// | **RcStatefulBiTransformer** | `RcStatefulBiTransformer<T, U, V>` | RcConditionalStatefulBiTransformer | into_rc | StatefulBiTransformer | 'static |
//
/// # Examples
///
/// ```ignore
/// // Two-parameter with Arc
/// impl_shared_transformer_methods!(
///     ArcTransformer<T, U>,
///     ArcConditionalTransformer,
//     into_arc,
//     Transformer,
//     Send + Sync + 'static
// );
//
// // Three-parameter with Rc
/// impl_shared_transformer_methods!(
///     RcBiTransformer<T, U, V>,
//     RcConditionalBiTransformer,
//     into_rc,
//     BiTransformer,
//     'static
// );
// ```
macro_rules! impl_shared_transformer_methods {
    // Two generic parameters
    ($struct_name:ident < $t:ident, $u:ident >, $return_type:ident, $predicate_conversion:ident, $transformer_trait:ident, $($extra_bounds:tt)+) => {
        pub fn when<P>(&self, predicate: P) -> $return_type<$t, $u>
        where
            P: Predicate<$t> + $($extra_bounds)+,
        {
            $return_type {
                transformer: self.clone(),
                predicate: predicate.$predicate_conversion(),
            }
        }

        #[allow(unused_mut)]
        pub fn and_then<C>(&self, mut after: C) -> $struct_name<$t, $u>
        where
            $t: 'static,
            $u: 'static,
            C: $transformer_trait<$u, $u> + $($extra_bounds)+,
        {
            let mut first = self.clone();
            $struct_name::new(move |t: &$t| {
                let intermediate = first.transform(t);
                after.transform(&intermediate)
            })
        }
    };
    // Three generic parameters
    ($struct_name:ident < $t:ident, $u:ident, $v:ident >, $return_type:ident, $predicate_conversion:ident, $transformer_trait:ident, $($extra_bounds:tt)+) => {
        pub fn when<P>(&self, predicate: P) -> $return_type<$t, $u, $v>
        where
            P: BiPredicate<$t, $u> + $($extra_bounds)+,
        {
            $return_type {
                transformer: self.clone(),
                predicate: predicate.$predicate_conversion(),
            }
        }

        #[allow(unused_mut)]
        pub fn and_then<C>(&self, mut after: C) -> $struct_name<$t, $u, $v>
        where
            $t: 'static,
            $u: 'static,
            $v: 'static,
            C: $transformer_trait<$v, $v, $v> + $($extra_bounds)+,
        {
            let mut first = self.clone();
            $struct_name::new(move |t: &$t, u: &$u| {
                let intermediate = first.transform(t, u);
                after.transform(&intermediate, &intermediate)
            })
        }
    };
}

pub(crate) use impl_shared_transformer_methods;
