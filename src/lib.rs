#![no_std]
#![deny(missing_docs)]
//! Simple string context map.
//!
//! ```rust
//! # use str_ctx::*;
//! let ctx = StrCtx::from_iter([("hello", "zombies")]);
//! assert_eq!("[hello:zombies]", &format!("{ctx}"));
//!
//! let ctx = ctx.derive([("friend", "apple")]);
//! assert_eq!("[friend:apple,hello:zombies]", &format!("{ctx}"));
//!
//! let ctx = ctx.derive([("hello", "world")]);
//! assert_eq!("[friend:apple,hello:world]", &format!("{ctx}"));
//!
//! let ctx: Vec<(&'static str, String)> = ctx.into_iter().collect();
//! assert_eq!(
//!     &[("friend", "apple".into()), ("hello", "world".into())],
//!     ctx.as_slice(),
//! );
//! ```

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::sync::Arc;
use core::fmt;

/// Simple string context map.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrCtx(Arc<[(&'static str, String)]>);

impl fmt::Debug for StrCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        let mut first = true;
        for (k, v) in self.0.iter() {
            if first {
                first = false;
            } else {
                f.write_str(",")?;
            }
            f.write_str(k)?;
            f.write_str(":")?;
            f.write_str(v)?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl fmt::Display for StrCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<V: fmt::Display> FromIterator<(&'static str, V)> for StrCtx {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'static str, V)>,
    {
        Self(BTreeMap::from_iter(iter.into_iter().map(|(k, v)| {
            (k, v.to_string())
        })).into_iter().collect::<Vec<_>>().into_boxed_slice().into())
    }
}

impl IntoIterator for StrCtx {
    type Item = (&'static str, String);
    type IntoIter = <Vec<(&'static str, String)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_vec().into_iter()
    }
}

impl<'a> IntoIterator for &'a StrCtx {
    type Item = &'a (&'static str, String);
    type IntoIter = core::slice::Iter<'a, (&'static str, String)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl StrCtx {
    /// Derive a sub-context map by merging in additional k/v strings.
    pub fn derive<V, T>(&self, iter: T) -> Self
    where
        V: fmt::Display,
        T: IntoIterator<Item = (&'static str, V)>,
    {
        Self::from_iter(self.into_iter().cloned().chain(iter.into_iter().map(|(k, v)| {
            (k, v.to_string())
        })))
    }
}
