pub extern crate log;

use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::mem;

mod macros;

/// Diagnostic context
#[derive(Debug, Clone)]
pub enum Ndc {
    Str(&'static str),
    String(String),
    Rc(Rc<String>),
    Arc(Arc<String>),
}

impl Default for Ndc {
    fn default() -> Self {
        Ndc::Str("")
    }
}

impl Ndc {
    /// View as string
    fn to_str(&self) -> &str {
        match self {
            Ndc::Str(s) => s,
            Ndc::String(s) => &s,
            Ndc::Rc(s) => &**s,
            Ndc::Arc(s) => &**s,
        }

    }
}

impl From<&'static str> for Ndc {
    fn from(s: &'static str) -> Self {
        Ndc::Str(s)
    }
}

impl From<String> for Ndc {
    fn from(s: String) -> Self {
        Ndc::String(s)
    }
}

impl From<Rc<String>> for Ndc {
    fn from(s: Rc<String>) -> Self {
        Ndc::Rc(s)
    }
}

impl From<Arc<String>> for Ndc {
    fn from(s: Arc<String>) -> Self {
        Ndc::Arc(s)
    }
}

thread_local! {
    static NDC: RefCell<Ndc> = RefCell::new(Ndc::Str(""));
}

#[must_use]
pub struct SetGuard {
    ndc: Ndc,
}

impl Drop for SetGuard {
    fn drop(&mut self) {
        set(mem::replace(&mut self.ndc, Default::default()));
    }
}

/// Set thread-local context to a given string.
pub fn set<S: Into<Ndc>>(ndc: S) {
    replace(ndc);
}

/// Set thread-local context to a given string and return previously stored value.
pub fn replace<S: Into<Ndc>>(ndc: S) -> Ndc {
    NDC.with(|r| {
        mem::replace(&mut *r.borrow_mut(), ndc.into())
    })
}

/// Clear diagnostic context
pub fn clear() {
    set("");
}

/// Set thread-local context and return a guard which sets previous value on drop.
pub fn push<S: Into<Ndc>>(ndc: S) -> SetGuard {
    SetGuard { ndc: replace(ndc) }
}

/// Get a copy of the raw NDC object.
/// Return NDC with empty string if unset.
pub fn get_raw() -> Ndc {
    NDC.with(|r| r.borrow().clone())
}

/// Get a copy of the string in thread-local context.
/// Empty string is returned when there's no context
pub fn get_copy() -> String {
    get(|s| s.to_owned())
}

/// Read a thread-local context with provided callback.
pub fn get<R, F>(f: F) -> R where F: FnOnce(&str) -> R {
    NDC.with(|r| f(r.borrow().to_str()))
}

#[cfg(test)]
mod test {
    mod log_ndc {
        pub use super::super::*;
    }

    #[test]
    fn set_get() {
        let _g = log_ndc::push("");

        log_ndc::set("aa");
        assert_eq!("aa", log_ndc::get_copy());
    }

    #[test]
    fn push() {
        let _g = log_ndc::push("");

        let _g1 = log_ndc::push("aa");
        assert_eq!("aa", log_ndc::get_copy());

        let g2 = log_ndc::push("bb");
        assert_eq!("bb", log_ndc::get_copy());
        drop(g2);

        assert_eq!("aa", log_ndc::get_copy());
    }
}
