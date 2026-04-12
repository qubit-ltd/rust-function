/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Closure Once Trait Implementation Macro
//!
//! This module provides the `impl_closure_once_trait!` macro for implementing
//! common conversion methods for closure-based once traits.
//!
//! ## Overview
//!
//! The macro generates standard conversion methods (`into_box`, `into_fn`) for
//! traits that are implemented by closures with once semantics. It
//! automatically infers all necessary information from the function signature
//! and trait name.
//!
//! ## Generated Methods
//!
//! - `into_box()`: Converts the closure into a boxed wrapper type
//! - `into_fn()`: Returns the closure as a generic `FnOnce` implementation
//! - Core method: Direct delegation to the underlying closure
//!
//! ## Usage
//!
//! The macro is typically used in trait definitions to provide consistent
//! conversion methods across different once trait implementations.
//!
//! ## Author
//!
//! Haixing Hu

/// Implement common conversion methods for closure once traits
///
/// This macro generates standard conversion methods for all once traits
/// that are implemented by closures. It automatically infers everything from
/// the function signature and trait name.
///
/// # Parameters
///
/// * `$trait_name<$(generics),*>` - Full trait name with generics (e.g., `ConsumerOnce<T>`, `BiFunctionOnce<T, U, R>`)
/// * `$method_name` - Core method name (e.g., `accept`, `apply`)
/// * `$box_type` - Box wrapper type (e.g., `BoxConsumerOnce`, `BoxBiFunctionOnce`)
/// * `$fn_trait` - Function signature (e.g., `FnOnce(value: &T)`, `FnOnce(first: &T, second: &U) -> R`)
///
/// # Generated implementation
///
/// ```ignore
/// impl<F, Generics...> TraitName<Generics...> for F
/// where
///     F: FnOnce(...),
/// {
///     fn method_name(self, ...) {
///         self(...)
///     }
///     fn into_box(self) -> BoxType<...> { ... }
///     fn into_fn(self) -> impl FnOnce(...) { ... }
/// }
/// ```
///
/// # Examples
///
/// ```ignore
/// // ConsumerOnce<T>
/// impl_closure_once_trait!(
///     ConsumerOnce<T>,
///     accept,
///     BoxConsumerOnce,
///     FnOnce(value: &T)
/// );
///
/// // BiConsumerOnce<T, U>
/// impl_closure_once_trait!(
///     BiConsumerOnce<T, U>,
///     accept,
///     BoxBiConsumerOnce,
///     FnOnce(first: &T, second: &U)
/// );
///
/// // FunctionOnce<T, R>
/// impl_closure_once_trait!(
///     FunctionOnce<T, R>,
///     apply,
///     BoxFunctionOnce,
///     FnOnce(input: &T) -> R
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_closure_once_trait {
  // ==================== Internal Implementation ====================

  // Core implementation: Generate complete trait implementation
  (
      @impl
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      $box_type:ident,
      ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
  ) => {
      impl<F, $($generics),*> $trait_name<$($generics),*> for F
      where
          F: FnOnce($($arg_ty),*) $(-> $ret)?,
      {
          // Core method: Direct closure call
          #[inline]
          fn $method_name(self, $($arg : $arg_ty),*) $(-> $ret)? {
              self($($arg),*)
          }

          // into_box: Convert to Box type
          #[inline]
          fn into_box(self) -> $box_type<$($generics),*>
          where
              Self: Sized + 'static,
          {
              $box_type::new(self)
          }

          // into_fn: Convert to closure (always return self directly since F is already FnOnce)
          #[inline]
          fn into_fn(self) -> impl FnOnce($($arg_ty),*) $(-> $ret)?
          where
              Self: Sized + 'static,
          {
              // F is already FnOnce with the correct signature, return directly (zero cost)
              self
          }
      }
  };

  // ==================== Public Interface ====================

  // No return value version
  (
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      $box_type:ident,
      FnOnce($($arg:ident : $arg_ty:ty),*)
  ) => {
      impl_closure_once_trait!(
          @impl
          $trait_name<$($generics),*>,
          $method_name,
          $box_type,
          ($($arg : $arg_ty),*)
      );
  };

  // With return value version
  (
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      $box_type:ident,
      FnOnce($($arg:ident : $arg_ty:ty),*) -> $ret:ty
  ) => {
      impl_closure_once_trait!(
          @impl
          $trait_name<$($generics),*>,
          $method_name,
          $box_type,
          ($($arg : $arg_ty),*) -> $ret
      );
  };
}

pub(crate) use impl_closure_once_trait;
