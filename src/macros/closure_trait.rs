/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

/// Implement trait for closures with automatic type inference
///
/// This macro generates blanket implementations for closures implementing
/// various function traits (Consumer, Function, Predicate, etc.). It
/// automatically infers everything from the function signature and trait name.
///
/// # Parameters
///
/// * `$trait_name<$(generics),*>` - Full trait name with generics (e.g., `Consumer<T>`, `Function<T, R>`)
/// * `$method_name` - Core method name (e.g., `accept`, `apply`, `test`)
/// * `$once_type` - Optional once wrapper type (e.g., `BoxConsumerOnce`)
/// * `$fn_signature` - Function signature (e.g., `Fn(value: &T)`, `FnMut(input: &T) -> R`)
///
/// # Generated implementation
///
/// Generates a blanket implementation for all closures matching the signature,
/// including:
/// - Core method implementation
/// - `into_box`, `into_rc`, `into_arc`, `into_fn` methods
/// - `to_box`, `to_rc`, `to_arc`, `to_fn` methods
/// - `into_once`, `to_once` methods (if once_type is provided)
///
/// # Examples
///
/// ```ignore
/// // Consumer trait (with once conversion)
/// impl_closure_trait!(
///     Consumer<T>,
///     accept,
///     BoxConsumerOnce,
///     Fn(value: &T)
/// );
///
/// // Function trait
/// impl_closure_trait!(
///     Function<T, R>,
///     apply,
///     BoxFunctionOnce,
///     Fn(input: &T) -> R
/// );
///
/// // Predicate trait (no once conversion)
/// impl_closure_trait!(
///     Predicate<T>,
///     test,
///     Fn(value: &T) -> bool
/// );
///
/// // StatefulConsumer trait
/// impl_closure_trait!(
///     StatefulConsumer<T>,
///     accept,
///     BoxConsumerOnce,
///     FnMut(value: &T)
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_closure_trait {
  // ==================== 辅助宏：生成 into_once 方法 ====================

  // Fn trait: into_once 方法
  (@into_once_fn_method $once_type:ident, ($($generics:ident),*)) => {
      #[inline]
      fn into_once(self) -> $once_type<$($generics),*>
      where
          Self: Sized + 'static,
          $($generics: 'static,)*
      {
          $once_type::new(self)
      }
  };

  // FnMut trait: into_once 方法
  (@into_once_fnmut_method $once_type:ident, ($($generics:ident),*), ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
      #[inline]
      fn into_once(mut self) -> $once_type<$($generics),*>
      where
          Self: Sized + 'static,
          $($generics: 'static,)*
      {
          $once_type::new(move |$($arg: $arg_ty),*| self($($arg),*))
      }
  };

  // ==================== 辅助宏：生成 to_once 方法 ====================

  // Fn trait: to_once 方法
  (@to_once_fn_method $once_type:ident, ($($generics:ident),*)) => {
      #[inline]
      fn to_once(&self) -> $once_type<$($generics),*>
      where
          Self: Clone + Sized + 'static,
          $($generics: 'static,)*
      {
          $once_type::new(self.clone())
      }
  };

  // FnMut trait: to_once 方法
  (@to_once_fnmut_method $once_type:ident, ($($generics:ident),*), ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
      #[inline]
      fn to_once(&self) -> $once_type<$($generics),*>
      where
          Self: Clone + Sized + 'static,
          $($generics: 'static,)*
      {
          let mut cloned = self.clone();
          $once_type::new(move |$($arg: $arg_ty),*| cloned($($arg),*))
      }
  };

  // ==================== 内部实现：通用部分（Fn trait）====================

  (
      @impl_common_fn
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      $closure_trait:path,
      ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
  ) => {
      // 核心方法：直接调用闭包
      #[inline]
      fn $method_name(&self, $($arg: $arg_ty),*) $(-> $ret)? {
          self($($arg),*)
      }

      // ===== 转换方法：使用 paste 自动推导类型名 =====

      #[inline]
      fn into_box(self) -> paste::paste! { [<Box $trait_name>] < $($generics),* > }
      where
          Self: Sized + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Box $trait_name>]::new(self) }
      }

      #[inline]
      fn into_rc(self) -> paste::paste! { [<Rc $trait_name>] < $($generics),* > }
      where
          Self: Sized + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Rc $trait_name>]::new(self) }
      }

      #[inline]
      fn into_fn(self) -> impl $closure_trait
      where
          Self: Sized + 'static,
      {
          self
      }

      // into_arc: Fn trait 需要 Send + Sync
      #[inline]
      fn into_arc(self) -> paste::paste! { [<Arc $trait_name>] < $($generics),* > }
      where
          Self: Sized + Send + Sync + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Arc $trait_name>]::new(self) }
      }

      // ===== to_* 方法：克隆版本 =====

      #[inline]
      fn to_box(&self) -> paste::paste! { [<Box $trait_name>] < $($generics),* > }
      where
          Self: Clone + Sized + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Box $trait_name>]::new(self.clone()) }
      }

      #[inline]
      fn to_rc(&self) -> paste::paste! { [<Rc $trait_name>] < $($generics),* > }
      where
          Self: Clone + Sized + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Rc $trait_name>]::new(self.clone()) }
      }

      #[inline]
      fn to_fn(&self) -> impl $closure_trait
      where
          Self: Clone + Sized + 'static,
      {
          self.clone()
      }

      // to_arc: Fn trait 需要 Send + Sync
      #[inline]
      fn to_arc(&self) -> paste::paste! { [<Arc $trait_name>] < $($generics),* > }
      where
          Self: Clone + Sized + Send + Sync + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Arc $trait_name>]::new(self.clone()) }
      }
  };

  // ==================== 内部实现：通用部分（FnMut trait）====================

  (
      @impl_common_fnmut
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      $closure_trait:path,
      ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
  ) => {
      // 核心方法：直接调用闭包
      #[inline]
      fn $method_name(&mut self, $($arg: $arg_ty),*) $(-> $ret)? {
          self($($arg),*)
      }

      // ===== 转换方法：使用 paste 自动推导类型名 =====

      #[inline]
      fn into_box(self) -> paste::paste! { [<Box $trait_name>] < $($generics),* > }
      where
          Self: Sized + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Box $trait_name>]::new(self) }
      }

      #[inline]
      fn into_rc(self) -> paste::paste! { [<Rc $trait_name>] < $($generics),* > }
      where
          Self: Sized + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Rc $trait_name>]::new(self) }
      }

      #[inline]
      fn into_fn(self) -> impl $closure_trait
      where
          Self: Sized + 'static,
      {
          self
      }

      // into_arc: FnMut trait 只需要 Send
      #[inline]
      fn into_arc(self) -> paste::paste! { [<Arc $trait_name>] < $($generics),* > }
      where
          Self: Sized + Send + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Arc $trait_name>]::new(self) }
      }

      // ===== to_* 方法：克隆版本 =====

      #[inline]
      fn to_box(&self) -> paste::paste! { [<Box $trait_name>] < $($generics),* > }
      where
          Self: Clone + Sized + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Box $trait_name>]::new(self.clone()) }
      }

      #[inline]
      fn to_rc(&self) -> paste::paste! { [<Rc $trait_name>] < $($generics),* > }
      where
          Self: Clone + Sized + 'static,
          $($generics: 'static,)*
      {
          paste::paste! { [<Rc $trait_name>]::new(self.clone()) }
      }

      #[inline]
      fn to_fn(&self) -> impl $closure_trait
      where
          Self: Clone + Sized + 'static,
      {
          self.clone()
      }

      // to_arc: FnMut trait 只需要 Send
      #[inline]
      fn to_arc(&self) -> paste::paste! { [<Arc $trait_name>] < $($generics),* > }
      where
          Self: Clone + Sized + Send + 'static,
          $($generics: 'static,)*
      {
          let cloned = self.clone();
          paste::paste! { [<Arc $trait_name>]::new(cloned) }
      }
  };


  // ==================== 公共接口：参考 impl_rc_conversions 的模式 ====================

  // Regular trait (Fn) - with once conversion
  (
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      $once_type:ident,
      Fn($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
  ) => {
      impl<$($generics,)* F> $trait_name<$($generics),*> for F
      where
          F: Fn($($arg_ty),*) $(-> $ret)?,
      {
          // 生成通用方法
          impl_closure_trait!(
              @impl_common_fn
              $trait_name<$($generics),*>,
              $method_name,
              Fn($($arg_ty),*) $(-> $ret)?,
              ($($arg : $arg_ty),*) $(-> $ret)?
          );

          // 生成 into_once 和 to_once 方法
          impl_closure_trait!(@into_once_fn_method $once_type, ($($generics),*));
          impl_closure_trait!(@to_once_fn_method $once_type, ($($generics),*));
      }
  };

  // Regular trait (Fn) - without once version
  (
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      Fn($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
  ) => {
      impl<$($generics,)* F> $trait_name<$($generics),*> for F
      where
          F: Fn($($arg_ty),*) $(-> $ret)?,
      {
          // 生成通用方法（不包含 into_once/to_once）
          impl_closure_trait!(
              @impl_common_fn
              $trait_name<$($generics),*>,
              $method_name,
              Fn($($arg_ty),*) $(-> $ret)?,
              ($($arg : $arg_ty),*) $(-> $ret)?
          );
      }
  };

  // Stateful trait (FnMut) - with once conversion
  (
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      $once_type:ident,
      FnMut($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
  ) => {
      impl<$($generics,)* F> $trait_name<$($generics),*> for F
      where
          F: FnMut($($arg_ty),*) $(-> $ret)?,
      {
          // 生成通用方法
          impl_closure_trait!(
              @impl_common_fnmut
              $trait_name<$($generics),*>,
              $method_name,
              FnMut($($arg_ty),*) $(-> $ret)?,
              ($($arg : $arg_ty),*) $(-> $ret)?
          );

          // 生成 into_once 和 to_once 方法
          impl_closure_trait!(@into_once_fnmut_method $once_type, ($($generics),*), ($($arg : $arg_ty),*) $(-> $ret)?);
          impl_closure_trait!(@to_once_fnmut_method $once_type, ($($generics),*), ($($arg : $arg_ty),*) $(-> $ret)?);
      }
  };

  // Stateful trait (FnMut) - without once version
  (
      $trait_name:ident < $($generics:ident),* >,
      $method_name:ident,
      FnMut($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
  ) => {
      impl<$($generics,)* F> $trait_name<$($generics),*> for F
      where
          F: FnMut($($arg_ty),*) $(-> $ret)?,
      {
          // 生成通用方法（不包含 into_once/to_once）
          impl_closure_trait!(
              @impl_common_fnmut
              $trait_name<$($generics),*>,
              $method_name,
              FnMut($($arg_ty),*) $(-> $ret)?,
              ($($arg : $arg_ty),*) $(-> $ret)?
          );
      }
  };
}

pub(crate) use impl_closure_trait;
