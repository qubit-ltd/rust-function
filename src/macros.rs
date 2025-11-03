/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Common Methods Macro
//!
//! Generates common methods for function-like structs (e.g., new, name).
//!
//! # Author
//!
//! Haixing Hu

/// Implements common constructor methods for function-like structs.
///
/// This macro generates `new`, `new_with_name`, and `new_with_optional_name`
/// methods for structs that wrap function pointers or closures. It provides
/// a standardized way to create instances with or without names for debugging
/// and logging purposes.
///
/// # Parameters
///
/// * `$($fn_trait_with_bounds)+` - Function trait bounds (e.g., Fn(i32) -> i32)
/// * `$f:ident` - Identifier for the function parameter
/// * `$wrapper_expr:expr` - Expression to wrap the function (e.g., Arc::new(f))
/// * `$type_desc:literal` - Description of the type (e.g., "consumer")
///
/// # Generated Methods
///
/// * `new<F>(f: F) -> Self` - Creates a new instance without a name
/// * `new_with_name<F>(name: &str, f: F) -> Self` - Creates a named instance
/// * `new_with_optional_name<F>(f: F, name: Option<String>) -> Self` -
///   Creates an instance with an optional name
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_common_new_methods {
    (
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        #[doc = concat!("Wraps the provided closure in the appropriate smart pointer type for this ", $type_desc, " implementation.")]
        pub fn new<F>($f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: None,
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, ".")]
        ///
        /// Wraps the provided closure and assigns it a name, which is
        /// useful for debugging and logging purposes.
        pub fn new_with_name<F>(name: &str, $f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: Some(name.to_string()),
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, " with an optional name.")]
        ///
        /// Wraps the provided closure and assigns it an optional name.
        pub fn new_with_optional_name<F>($f: F, name: Option<String>) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name,
            }
        }
    };
}

/// Implements common name management methods for function-like structs.
///
/// This macro generates `name`, and `set_name` methods for structs that have
/// an optional name field. These methods provide a standardized way to get
/// and set names for debugging and logging purposes.
///
/// # Parameters
///
/// * `$type_desc:literal` - Description of the type (e.g., "consumer")
///
/// # Generated Methods
///
/// * `name(&self) -> Option<&str>` - Gets the current name if set
/// * `set_name(&mut self, name: &str)` - Sets a new name for the instance
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_common_name_methods {
    ($type_desc:literal) => {
        #[doc = concat!("Gets the name of this ", $type_desc, ".")]
        ///
        /// # Returns
        ///
        /// Returns `Some(&str)` if a name was set, `None` otherwise.
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }

        #[doc = concat!("Sets the name of this ", $type_desc, ".")]
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name to set for this ", $type_desc)]
        pub fn set_name(&mut self, name: &str) {
            self.name = Some(name.to_string());
        }
    };
}

pub(crate) use impl_common_name_methods;
pub(crate) use impl_common_new_methods;
