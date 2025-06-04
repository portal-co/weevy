use std::{borrow::Cow, fmt::Display, ops::Deref};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Config<T> {
    pub isolate: T,
}
impl<T: Deref<Target = str>> Config<T> {
    pub fn prefix(&self) -> String {
        format!("__WeevySbx${}$", &*self.isolate)
    }
    pub fn needs_prefix(&self, val: &str) -> bool {
        ["__Weevy"].into_iter().any(|v| val.starts_with(v))
            || ["location"].into_iter().any(|a| a == val)
    }
    pub fn rewrite<'a>(&self, x: &'a str) -> Cow<'a, str> {
        match self.needs_prefix(x) {
            true => Cow::Owned(format!("{}{x}", self.prefix())),
            false => Cow::Borrowed(x),
        }
    }
    pub fn unrewrite<'a>(&self, x: &'a str) -> &'a str {
        let Some(a) = x.strip_prefix("__WeevySbx$") else {
            return x;
        };
        let Some(a) = a.strip_prefix(&*self.isolate) else {
            return x;
        };
        let Some(a) = a.strip_prefix("$") else {
            return x;
        };
        return a;
    }
}
