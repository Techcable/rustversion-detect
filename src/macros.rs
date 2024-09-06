//! Defines the [`maybe_const_fn`] macro.

/// Mark a function as `const` if a `cfg!(...)` attribute evaluates to true.
///
/// This has the same effect as using `#[rustversion::attr(version_test, const)]`,
/// but is implemented as a declarative macro instead of a procedural one.
///
/// It is subject to some minor limitations (see below).
///
/// If these are unacceptable,
/// please use the `rustversion` procedural macro.
/// That is more robust and supports other markers like `unsafe` and `async`.
///
/// ## Example
/// ```
/// # use rustversion_detect::maybe_const_fn;
///
/// maybe_const_fn! {
///     #[cfg_const(all())] // always true
///     /// Example documentation
///     #[inline] // Another attribute
///     const fn example() -> u32 {
///         3 + 7
///     }
///
///     #[cfg_const(any())] // always false
///     const fn not_const() -> Vec<u32> {
///         vec![3, 7, 8]
///     }
/// }
///
/// const FOO: u32 = example();
/// ```
///
/// ## Limitations
/// Please remember to always place `const` before the `fn` declaration.
/// Otherwise, the macro will give a value error.
///
/// The `#[cfg_const(...)]` marker must be the first attitude declaration.
/// All other attributes and documentation comments must come after the `#[cfg_const(..)]` declartion.
///
/// The following two examples are **broken**:
/// ```compile_fail
/// # use rustversion_detect::maybe_const_fn;
///
/// maybe_const_fn! {
///     /// doc comment first
///     #[cfg_const(all())]
///     /// Example documentation
///     #[inline] // Another attribute
///     const unsafe fn example() -> u32 {
///         3 + 7
///     }
/// }
/// ```
///
/// ```compile_fail
/// # use rustversion_detect::maybe_const_fn;
///
/// maybe_const_fn! {
///     #[inline] // attribute first
///     #[cfg_const(all())]
///     /// Example documentation
///     #[inline] // Another attribute
///     const unsafe fn example() -> u32 {
///         3 + 7
///     }
/// }
/// ```
///
/// ## Additional markers (`unsafe`, `async`, etc..)
/// Additional markers like `async`, `unsafe`, and `extern "C"`
/// must be surrounded by `{...}` due to macro limitations.
///
/// This is **correct**:
/// ```
/// # use rustversion_detect::maybe_const_fn;
///
/// maybe_const_fn! {
///     #[cfg_const(all())] // always true
///     /// Example documentation
///     #[inline] // Another attribute
///     const {unsafe} fn example() -> u32 {
///         3 + 7
///     }
/// }
///
/// const FOO: u32 = unsafe { example() };
/// ```
///
/// This is **broken**:
/// ```compile_fail
/// # use rustversion_detect::maybe_const_fn;
///
/// maybe_const_fn! {
///     #[cfg_const(all())] // always true
///     /// Example documentation
///     #[inline] // Another attribute
///     const unsafe fn example() -> u32 {
///         3 + 7
///     }
/// }
///
/// const FOO: u32 = unsafe { example() };
/// ```
///
/// ### Macro Forwarding
/// When [forwarding a matched fragment] inside another macro,
/// the outer macro cannot use fragment specifiers like `item`
/// for the constant function declaration.
/// As explained in the docs,
/// using the `tt` and `ident` fragment specifiers are a special exception.
///
/// The following is **broken**:
/// ```compile_fail
/// # use rustversion_detect::maybe_const_fn;
///
/// macro_rules! item_spec {
///     ($bad:item) => {
///         maybe_const_fn! {
///             #[cfg_const(all())] // always true
///             $bad
///         }
///     }
/// }
///
/// // will give the following error message:
/// // > captured metavariables except for `:tt`, `:ident` and `:lifetime` cannot be compared to other tokens
/// item_spec! {
///     const fn foo() {}
/// }
/// ```
///
/// [forwarding a matched fragment]: https://doc.rust-lang.org/reference/macros-by-example.html#forwarding-a-matched-fragment
#[macro_export]
macro_rules! maybe_const_fn {
    ($(
        #[cfg_const($cond:meta)]
        $(#[$attr:meta])*
        $visibility:vis const
        // extra "specifiers" like `async` `unsafe` `extern "C"`
        // needs to be surrounded with {...} due to macro limitations
        //
        // NOTE: Need to use $()* because $()? not supported on 1.31
        $({$($extra_spec:tt)*})*
        fn $name:ident ($($args:tt)*) $( -> $return_tp:ty)* $code:block
    )*) => {$(
        #[cfg($cond)]
        $(#[$attr])*
        $visibility const $($($extra_spec)*)* fn $name ( $($args)* ) $(-> $return_tp)* $code

        #[cfg(not($cond))]
        $(#[$attr])*
        $visibility $($($extra_spec)*)* fn $name ( $($args)* ) $(-> $return_tp)* $code
    )*};
}
