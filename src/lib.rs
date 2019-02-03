extern crate log;

use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

mod macros;

/// Diagnostic context
enum Ndc {
    Str(&'static str),
    String(String),
    Rc(Rc<String>),
    Arc(Arc<String>),
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

thread_local! {
    static NDC: RefCell<Ndc> = RefCell::new(Ndc::Str(""));
}

fn set_ndc(ndc: Ndc) {
    NDC.with(|r| *r.borrow_mut() = ndc);
}

/// Set thread-local context to a string.
pub fn set_string(ndc: String) {
    set_ndc(Ndc::String(ndc));
}

/// Set thread-local context to a `&'static str`.
pub fn set_static_str(ndc: &'static str) {
    set_ndc(Ndc::Str(ndc))
}

/// Set thread-local context to `Rc<String>`
pub fn set_rc_string(ndc: Rc<String>) {
    set_ndc(Ndc::Rc(ndc))
}

/// Set thread-local context to `Arc<String>`
pub fn set_arc_string(ndc: Arc<String>) {
    set_ndc(Ndc::Arc(ndc))
}

/// Clear diagnostic context
pub fn clear() {
    set_static_str("");
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
