/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Predicate Macros Module
//!
//! Provides declarative macros to simplify Predicate implementations and
//! reduce code duplication.
//!
//! # Author
//!
//! Haixing Hu

pub(crate) mod constants;
mod box_predicate_methods;
mod predicate_clone;
mod predicate_common_methods;
mod predicate_debug_display;
mod shared_predicate_methods;

// Export all macros for use within the crate
pub(crate) use box_predicate_methods::impl_box_predicate_methods;
pub(crate) use predicate_clone::impl_predicate_clone;
pub(crate) use predicate_common_methods::impl_predicate_common_methods;
pub(crate) use predicate_debug_display::impl_predicate_debug_display;
pub(crate) use shared_predicate_methods::impl_shared_predicate_methods;
