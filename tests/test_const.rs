use rustversion_detect::maybe_const_fn;

maybe_const_fn! {
    #[cfg_const(all())] // always true
    const fn _basic() {}
}
const _BASIC: () = _basic();

maybe_const_fn! {
    #[cfg_const(all())]
    const {unsafe} fn _unsafe() {}
}
const _UNSAFE: () = unsafe { _unsafe() };

/// using `:item` matcher won't work, but `:ident` matcher will
macro_rules! ident {
    ($fn:ident) => {
        maybe_const_fn! {
            #[cfg_const(all())]
            const $fn _ident() {}
        }
    };
}

ident! {fn}
const _IDENT: () = _ident();

maybe_const_fn! {
    #[cfg_const(all())]
    /// doc
    const fn _doc_below() {}
}
const _DOC_BELOW: () = _doc_below();
