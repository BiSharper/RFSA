use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

pub const GFS_SEPARATOR: char = '/';
// pub const ALT_SEPARATOR: char = '\\';
// const SEPARATORS: [char; 2] = [ALT_SEPARATOR, GFS_SEPARATOR];

pub trait PathLike: Deref<Target = str> + Debug + Sized + Display + Clone + Hash + Eq + PartialEq + Ord + PartialOrd  {
    fn as_str(&self) -> &str;

    fn to_string(&self) -> String;

    fn is_child(&self, path: &Self) -> bool {
        path.starts_with(self.as_directory_string().as_str())
    }

    fn filename(&self) -> String {
        let last_slash = self.rfind('/').unwrap();
        self[last_slash + 1..].to_owned()
    }

    fn filename_no_extension(&self) -> String {
        let last_slash = self.rfind('/').unwrap_or(0);
        let dot_pos = self.rfind('.');
        match dot_pos {
            Some(pos) if pos > last_slash => self[last_slash + 1..pos].to_owned(),
            _ => self[last_slash + 1..].to_owned(),
        }
    }

    fn extension(&self) -> Option<String> {
        let last_slash = self.rfind('/')?;
        let dot_pos = self.rfind('.');
        match dot_pos {
            Some(pos) if pos > last_slash => Some(self[pos + 1..].to_owned()),
            _ => None,
        }
    }

    fn parent_directory_string(&self) -> Option<String> {
        self.rfind('/')
            .map(|idx| Some(self[..idx].to_string()))
            .unwrap_or_else(|| None)
    }

    fn to_vpath(self) -> VPath;

    fn as_directory_string(&self) -> String { format!("{}{}", self.as_str(), GFS_SEPARATOR) }

    fn join<T: AsRef<str>>(&self, other: T) -> Self {
        Self::create(format!("{}{}", self.as_str(), other.as_ref()))
    }

    fn create(path: String) -> Self;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct VPath(Arc<str>);

impl From<String> for VPath {
    fn from(value: String) -> Self {
        Self::create(value)
    }
}

impl Deref for VPath {
    type Target = str;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl AsRef<str> for VPath {
    fn as_ref(&self) -> &str {
        &*self.0
    }
}

impl Display for VPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_str()) }
}

impl PathLike for VPath {
    fn as_str(&self) -> &str { &self.0 }

    fn to_string(&self) -> String { self.0.to_string() }

    fn to_vpath(self) -> VPath { self }

    fn create(path: String) -> Self { Self { 0: Arc::from(path), } }
}


