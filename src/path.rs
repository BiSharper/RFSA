use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

pub const SEPARATOR: char = '/';

pub trait PathLike: AsRef<str> + Deref<Target = str> + Debug + Sized + Display + Clone + Hash + Eq + PartialEq + Ord + PartialOrd  {
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

    fn to_path<T: PathLike>(self) -> T { T::normalized(self.as_str()) }

    fn as_directory_string(&self) -> String { format!("{}{}", self.as_str(), SEPARATOR) }

    fn to_directory_string(self) -> String { self.as_directory_string() }

    fn join<T: AsRef<str>>(&self, other: T) -> Self {
        Self::exact(format!("{}{}", self.as_str(), other.as_ref()).as_str())
    }

    fn join_into<T: AsRef<str>>(self, other: T) -> Self {
        Self::exact(format!("{}{}", self, other.as_ref()).as_str())
    }

    fn exact(path: &str) -> Self;

    fn normalized(path: &str) -> Self;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct VPath(Arc<str>);

impl From<String> for VPath {
    fn from(value: String) -> Self {
        Self::normalized(&*value)
    }
}

impl From<&str> for VPath {
    fn from(value: &str) -> Self { Self::normalized(value) }
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

    fn exact(path: &str) -> Self { Self { 0: Arc::from(path) } }

    fn normalized(path: &str) -> Self {
        let mut result = Vec::with_capacity(path.len());
        let mut last_was_separator = true;
        let mut chars_written = 1;
        result.push(SEPARATOR);
        for c in path.chars() {
            match c {
                '/' | '\\' => {
                    if last_was_separator { continue; }
                    result.push(SEPARATOR);
                    chars_written += 1;
                    last_was_separator = true;
                }
                _ => {
                    last_was_separator = false;
                    result.push(c.to_ascii_lowercase());
                    chars_written += 1;
                }
            }
        }
        if chars_written > 0 && result[chars_written - 1] == '\\' { result.pop(); };
        Self::exact(&*result[..chars_written].iter().collect::<String>())
    }
}


