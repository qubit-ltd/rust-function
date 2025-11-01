/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Function Macros Module
//!
//! Provides declarative macros to simplify Function implementations and
//! reduce code duplication.
//!
//! # Author
//!
//! Haixing Hu

// Module declarations
mod box_conditional_function;
mod box_function_methods;
mod conditional_function_clone;
mod conditional_function_conversions;
mod conditional_function_debug_display;
mod function_clone;
mod function_common_methods;
mod function_constant_method;
mod function_debug_display;
mod function_identity_method;
mod shared_conditional_function;
mod shared_function_methods;

// Export all macros for use within the crate
pub(crate) use box_conditional_function::impl_box_conditional_function;
pub(crate) use box_function_methods::impl_box_function_methods;
pub(crate) use conditional_function_clone::impl_conditional_function_clone;
pub(crate) use conditional_function_conversions::impl_conditional_function_conversions;
pub(crate) use conditional_function_debug_display::impl_conditional_function_debug_display;
pub(crate) use function_clone::impl_function_clone;
pub(crate) use function_common_methods::impl_function_common_methods;
pub(crate) use function_constant_method::impl_function_constant_method;
pub(crate) use function_debug_display::impl_function_debug_display;
pub(crate) use function_identity_method::impl_function_identity_method;
pub(crate) use shared_conditional_function::impl_shared_conditional_function;
pub(crate) use shared_function_methods::impl_shared_function_methods;
