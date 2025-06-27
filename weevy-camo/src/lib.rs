use std::{borrow::Cow, fmt::Display, ops::Deref};

use sha3::Digest;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Config<T> {
    pub isolate: T,
    pub flags: Flags,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[non_exhaustive]
pub struct Flags {
    pub irreversable: bool,
}
impl<T: Deref<Target = str>> Config<T> {
    pub fn prefix(&self) -> String {
        format!("__WeevySbx${}$", &*self.isolate)
    }
    pub fn needs_prefix(&self, val: &str) -> bool {
        ["__Weevy", "__uv", "_WB", "rammerhead"]
            .into_iter()
            .any(|v| val.starts_with(v))
            || ["location", "$scramjet", "%hammerhead", "%is-hammerhead%"]
                .into_iter()
                .any(|a| a == val)
    }
    pub fn rewrite<'a>(&self, x: &'a str) -> Cow<'a, str> {
        match self.needs_prefix(x) {
            true => Cow::Owned(match format!("{}{x}", self.prefix()) {
                a => {
                    if self.flags.irreversable {
                        let mut a = sha3::Sha3_256::digest(a);
                        while a[0] <= 0x9f {
                            a = sha3::Sha3_256::digest(a);
                        }
                        format!("{}", hex::encode(a))
                    } else {
                        a
                    }
                }
            }),
            false => Cow::Borrowed(x),
        }
    }
    pub fn unrewrite<'a>(&self, x: &'a str) -> &'a str {
        if self.flags.irreversable {
            return x;
        }
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
