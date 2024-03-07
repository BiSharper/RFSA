use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

pub const SEPARATOR: char = '/';

pub trait PathLike:
AsRef<str> + Deref<Target = str> + Debug + Sized + Display + Clone + Hash + Eq + PartialEq + Ord + PartialOrd  {
    fn as_str(&self) -> &str;

    fn to_string(&self) -> String;

    fn is_child(&self, path: &Self) -> bool {
        path.starts_with(self.as_directory_string().as_str())
    }

    fn filename(&self) -> String {
        let last_slash = self.rfind('/').unwrap();
        self[last_slash + 1..].to_owned()
    }

    fn filename_no_extension(&self) -> &str {
        let last_slash = self.rfind('/').unwrap_or(0);
        let dot_pos = self.rfind('.');
        match dot_pos {
            Some(pos) if pos > last_slash => &self[last_slash + 1..pos],
            _ => &self[last_slash + 1..],
        }
    }

    fn extension(&self) -> Option<&str> {
        let last_slash = self.rfind('/')?;
        let dot_pos = self.rfind('.');
        match dot_pos {
            Some(pos) if pos > last_slash => Some(&self[pos + 1..]),
            _ => None,
        }
    }

    fn parent_directory_string(&self) -> Option<String> {
        self.rfind('/')
            .map(|idx| Some(self[..idx].to_string()))
            .unwrap_or_else(|| None)
    }

    fn directory_str_len(&self) -> usize { self.len() + 1 }

    fn as_vpath(&self) -> &VPath;

    fn to_vpath(self) -> VPath { self.as_vpath().clone() }

    fn as_directory_string(&self) -> String { format!("{}{}", self.as_str(), SEPARATOR) }

    fn to_directory_string(self) -> String { self.as_directory_string() }

    fn join<T: AsRef<str>>(&self, other: T) -> Self {
        Self::exact(format!("{}{}", self.as_str(), other.as_ref()).as_str())
    }

    fn join_into<T: AsRef<str>>(self, other: T) -> Self {
        Self::exact(format!("{}{}", self, other.as_ref()).as_str())
    }

    fn exact(path: &str) -> Self;

    fn normalize(path: &str) -> String {
        todo!("path = {}", path)
    }

    fn create(path: &str) -> Self { Self::exact(&*Self::normalize(path)) }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct VPath(Arc<str>);

impl From<String> for VPath {
    fn from(value: String) -> Self {
        Self::create(&*value)
    }
}

impl From<&str> for VPath {
    fn from(value: &str) -> Self { Self::create(value) }
}

impl Deref for VPath {
    type Target = str;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Display for VPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_str()) }
}

impl AsRef<str> for VPath {
    fn as_ref(&self) -> &str { self.0.as_ref() }
}

impl PathLike for VPath {
    fn as_str(&self) -> &str { &self.0 }

    fn to_string(&self) -> String { self.0.to_string() }

    fn as_vpath(&self) -> &VPath { self }

    fn exact(path: &str) -> Self { Self { 0: Arc::from(path) } }
}


