//! Defines the [SlopValue] enum and its implementations.
//! 
//! Eveything in this module is publically imported into [crate], so you can
//! just import them from there.

use std::{str::FromStr, fmt::Display};

/// The possible values a [Slop]'s KVs can contain.
/// 
/// ## Examples
/// 
/// ```
/// use slop_rs::SlopValue;
/// 
/// let a = SlopValue::String("some string value".to_string());
/// let b = SlopValue::from(vec!["item a", "item b", "item c"]);
/// let c: SlopValue = (&["1", "2", "3"][..]).into();
/// 
/// assert_eq!(a.string(), Some(&"some string value".to_string()));
/// assert!(b.is_list());
/// assert_eq!(c.string(), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlopValue {
    /// The value of a string KV. (a [String] in Rust's case)
    String(String),

    /// The value of a list KV. (a [Vec]<[String]> in Rust's case)
    List(Vec<String>),
}

impl SlopValue {
    /// Returns true if this [SlopValue] is a [SlopValue::String].
    #[inline(always)]
    pub fn is_string(&self) -> bool {
        if let Self::String(_) = self { true } else { false }
    }
    
    /// Returns true if this [SlopValue] is a [SlopValue::List].
    #[inline(always)]
    pub fn is_list(&self) -> bool {
        if let Self::List(_) = self { true } else { false }
    }

    /// Returns the contained string,
    /// or [None] if this is not a [SlopValue::String].
    /// 
    /// To consume the [SlopValue], consider using [SlopValue::into]
    /// for [String].
    #[inline(always)]
    pub fn string(&self) -> Option<&String> {
        if let Self::String(s) = self { Some(&s) } else { None }
    }

    /// Returns the contained list,
    /// or [None] if this is not a [SlopValue::List].
    /// 
    /// To consume the [SlopValue], consider using [SlopValue::into]
    /// for [Vec]<[String]>.
    #[inline(always)]
    pub fn list(&self) -> Option<&Vec<String>> {
        if let Self::List(l) = self { Some(&l) } else { None }
    }

    /// If the value is a [SlopValue::String], attempts to parse it.
    /// 
    /// Returns [None] if the value is a [SlopValue::List].
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use slop_rs::SlopValue;
    /// 
    /// let val_1 = SlopValue::from("16").parse_into::<u8>();
    /// let val_2 = SlopValue::from(vec!["a", "b"]).parse_into::<u8>();
    /// 
    /// assert_eq!(val_1, Some(Ok(16)));
    /// assert_eq!(val_2, None);
    /// ```
    #[inline]
    pub fn parse_into<T>(&self) -> Option<Result<T, T::Err>> where T: FromStr {
        if let Some(s) = self.string() {
            Some(s.parse())
        } else {
            None
        }
    }

    /// Same as [SlopValue::to_string], but indents the values of
    /// [SlopValue::List]s. Uses 4 spaces for indentation.
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use slop_rs::SlopValue;
    /// 
    /// let val = SlopValue::from(&["a", "b", "c"][..]);
    /// 
    /// let standard = "{
    /// a
    /// b
    /// c
    /// }";
    /// let pretty = "{
    ///     a
    ///     b
    ///     c
    /// }";
    /// 
    /// assert_eq!(format!("{val}"), standard);
    /// assert_eq!(val.to_string_pretty(), pretty);
    /// ```
    #[inline]
    pub fn to_string_pretty(&self) -> String {
        match self {
            Self::String(s) => format!("={s}"),
            Self::List(l) => format!("{{\n    {}\n}}", l.join("\n    ")),
        }
    }
}

impl Display for SlopValue {
    /// Displays the [SlopValue] as a the value part of a SLOP key-value.
    /// (including `=` and `{...}`)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "={s}"),
            Self::List(l) => write!(f, "{{\n{}\n}}", l.join("\n")),
        }
    }
}

impl From<String> for SlopValue {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for SlopValue {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Vec<String>> for SlopValue {
    #[inline(always)]
    fn from(value: Vec<String>) -> Self {
        Self::List(value)
    }
}

impl From<Vec<&str>> for SlopValue {
    #[inline(always)]
    fn from(value: Vec<&str>) -> Self {
        Self::List(value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl From<&[String]> for SlopValue {
    #[inline(always)]
    fn from(value: &[String]) -> Self {
        Self::List(value.to_owned())
    }
}

impl From<&[&str]> for SlopValue {
    #[inline(always)]
    fn from(value: &[&str]) -> Self {
        Self::List(value.into_iter().map(|s| s.to_string()).collect())
    }
}
