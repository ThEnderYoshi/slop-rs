//! Defines the [Slop] struct and its implementations.
//!
//! Eveything in this module is publically imported into [crate], so you can
//! just import them from there.

use std::{
    collections::{HashMap, hash_map},
    str::FromStr,
    path::Path,
    fs,
    fmt::Display,
};

use crate::{
    SlopValue,
    error::{SlopError, SlopResult},
};

/// A parsed SLOP object loaded into memory.
/// Referred to simply as "a [Slop]" throughout the documentation.
///
/// ## Examples
///
/// ```
/// use slop_rs::Slop;
///
/// let slop_str = "\
///     some-key=some value
///     other-key{
///         other
///         value
///     }
/// ";
/// let slop: Slop = slop_str.parse().unwrap();
///
/// assert_eq!(slop.get("some-key"), Some(&"some value".into()));
/// assert_eq!(slop.get("other-key"), Some(&vec!["other", "value"].into()));
/// assert_eq!(slop.get("invalid key"), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slop {
    items: HashMap<String, SlopValue>,
}

impl Slop {
    /// Constructs an empty [Slop].
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    /// Reads the contents of a file, parses it as a SLOP string, then returns a
    /// new [Slop] with the resulting items.
    #[inline(always)]
    pub fn open<P: AsRef<Path>>(path: P) -> SlopResult<Self> {
        fs::read_to_string(path)?.parse()
    }

    /// Iterates over the [Slop]'s KVs in arbitrary order.
    /// The iterator element type is `(&'a String, &'a SlopValue)`.
    /// 
    /// This is the same iterator type returned by [HashMap::iter].
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use slop_rs::Slop;
    /// 
    /// let slop_str = "
    ///     a=1
    ///     b=2
    ///     c=3
    /// ";
    /// let slop: Slop = slop_str.parse().unwrap();
    /// 
    /// for (key, value) in slop.iter() {
    ///     println!("key: {key} val: {value:?}");
    /// }
    /// ```
    pub fn iter(&self) -> hash_map::Iter<'_, String, SlopValue> {
        self.items.iter()
    }

    /// Iterates over the [Slop]'s KVs in arbitrary order,
    /// with mutable references to the values.
    /// The iterator element type is `(&'a String, &'a mut SlopValue)`.
    /// 
    /// This is the same iterator type returned by [HashMap::iter_mut].
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use slop_rs::{Slop, SlopValue};
    /// 
    /// let slop_str = "
    ///     a=1
    ///     b=2
    ///     c{
    ///         alpha
    ///         beta
    ///     }
    /// ";
    /// let mut slop: Slop = slop_str.parse().unwrap();
    /// 
    /// for (_, value) in slop.iter_mut() {
    ///     match value {
    ///         SlopValue::String(s) => s.push_str("!!!"),
    ///         SlopValue::List(l) => l.push("gamma".to_string()),
    ///     }
    /// }
    /// 
    /// for (key, value) in slop.iter() {
    ///     println!("key: {key} val: {value:?}");
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, String, SlopValue> {
        self.items.iter_mut()
    }

    /// Returns `true` if the [Slop] is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns `true` if the [Slop] contains the provided key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Returns the [SlopValue] associated with the provided key,
    /// or [None] if no such KV exists.
    /// 
    /// See also: [Slop::get_string] and [Slop::get_list].
    pub fn get(&self, key: &str) -> Option<&SlopValue> {
        self.items.get(key)
    }

    /// Returns the [String] associated with the provided key,
    /// or [None] if no such KV exists or it holds a [Vec]<[String]>.
    /// 
    /// See also: [Slop::get] and [Slop::get_list].
    /// 
    /// ```
    /// use slop_rs::Slop;
    /// 
    /// let slop_str = "
    ///     str-kv=value
    ///     list-kv{
    ///         value 1
    ///         value 2
    ///     }
    /// ";
    /// let slop: Slop = slop_str.parse().unwrap();
    /// 
    /// assert_eq!(slop.get_string("str-kv"), Some(&"value".to_string()));
    /// assert_eq!(slop.get_string("list-kv"), None);
    /// ```
    #[inline(always)]
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.get(key)?.string()
    }

    /// Returns the [Vec]<[String]> associated with the provided key,
    /// or [None] if no such KV exists or it holds a [String].
    /// 
    /// See also: [Slop::get] and [Slop::get_string].
    /// 
    /// ```
    /// use slop_rs::Slop;
    /// 
    /// let slop_str = "
    ///     str-kv=value
    ///     list-kv{
    ///         value 1
    ///         value 2
    ///     }
    /// ";
    /// let slop: Slop = slop_str.parse().unwrap();
    /// let data = vec!["value 1".to_string(), "value 2".to_string()];
    /// 
    /// assert_eq!(slop.get_list("str-kv"), None);
    /// assert_eq!(slop.get_list("list-kv"), Some(&data));
    /// ```
    #[inline(always)]
    pub fn get_list(&self, key: &str) -> Option<&Vec<String>> {
        self.get(key)?.list()
    }

    /// Inserts `value` in the KV defined by `key`.
    ///
    /// Returns the previous value, or [None] if no such KV existed before.
    /// 
    /// Returns a [SlopError] if the key contains `=` or ends in `{`, as these
    /// keys would produce an invalid SLOP string. \
    /// If you know for a fact that the key is valid, you can use
    /// [Slop::insert_unchecked] instead.
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use slop_rs::Slop;
    /// 
    /// let mut slop = Slop::new();
    /// 
    /// let prev_value = slop.insert("key".to_string(), "value");
    /// assert!(prev_value.is_ok());
    /// assert_eq!(slop.get("key"), Some(&"value".into()));
    /// 
    /// let prev_value = slop.insert("this key = bad".to_string(), "value");
    /// assert!(prev_value.is_err());
    /// assert_eq!(slop.get("this key = bad"), None);
    /// ```
    pub fn insert<V: Into<SlopValue>>(&mut self, key: String, value: V)
        -> SlopResult<Option<SlopValue>>
    {
        if key.chars().any(|c| c == '=') || key.ends_with('{') {
            Err(SlopError::InvalidKey(key))
        } else {
            Ok(self.items.insert(key, value.into()))
        }
    }

    /// A variation of [Slop::insert] that doesn't check whether the key
    /// is valid.
    /// 
    /// Use if you know ahead of time that the key is always valid.
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use slop_rs::Slop;
    /// 
    /// let mut slop = Slop::new();
    /// 
    /// let prev_value = slop.insert_unchecked("key".to_string(), "value");
    /// assert_eq!(prev_value, None);
    /// assert_eq!(slop.get("key"), Some(&"value".into()));
    /// ```
    pub fn insert_unchecked<V: Into<SlopValue>>(&mut self, key: String, value: V)
        -> Option<SlopValue>
    {
        self.items.insert(key, value.into())
    }

    /// Parses the provided SLOP string and appends the results.
    ///
    /// If you are creating the [Slop] just before parsing, consider
    /// using [str::parse] instead.
    ///
    /// **Note:** The parser pushes any items it finds as it goes; if an error
    /// occours while parsing, any previously parsed items will already be in
    /// the [Slop].
    ///
    /// ## Examples
    /// 
    /// ```
    /// use slop_rs::Slop;
    ///
    /// let slop_str = "
    ///     a=b
    ///     c{
    ///         d
    ///         e
    ///     }
    /// ";
    ///
    /// let mut slop = Slop::new();
    /// slop.append_slop_string(slop_str).unwrap();
    ///
    /// assert_eq!(slop.get("a"), Some(&"b".into()));
    /// assert_eq!(slop.get("c"), Some(&["d", "e"][..].into()));
    /// 
    /// // Using parse() instead:
    /// let slop: Slop = slop_str.parse().unwrap();
    ///
    /// assert_eq!(slop.get("a"), Some(&"b".into()));
    /// assert_eq!(slop.get("c"), Some(&["d", "e"][..].into()));
    /// ```
    pub fn append_slop_string(&mut self, slop_str: &str) -> Result<(), SlopError>
    {
        let lines: Vec<&str> = slop_str.split('\n').collect();
        let mut skip_lines = 0usize;

        for i in 0..lines.len() {
            if skip_lines > 0 {
                skip_lines -= 1;
                continue;
            }

            // SAFETY: `i` is always in range.
            let line = clean_up_line(lines[i]);

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = parse_string_kv(line) {
                self.insert(key.to_string(), value)?;
            } else if let Some((key, value, skip))
                = parse_list_kv(&lines, i)?
            {
                self.insert(key.to_string(), value)?;
                skip_lines = skip;
            } else {
                return Err(SlopError::InvalidLine(i, line.to_string()));
            }
        }

        Ok(())
    }

    /// Same as [Slop::to_string], but indents the values of lists. Uses 4
    /// spaces for indentation.
    pub fn to_string_pretty(&self) -> String {
        self.items.iter().fold(String::new(), |mut acc, (k, v)| {
            acc.push_str(&k);
            acc.push_str(&v.to_string_pretty());
            acc.push('\n');
            acc
        })
    }

    /// Converts the [Slop] into a SLOP string and writes it to the text file at
    /// the provided path.
    /// 
    /// If you want the list values to be indented, see [Slop::save_pretty].
    #[inline(always)]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> SlopResult<()> {
        Ok(fs::write(path, self.to_string())?)
    }

    /// Same as [Slop::save], but indents the values of lists. Uses 4
    /// spaces for indentation.
    #[inline(always)]
    pub fn save_pretty<P: AsRef<Path>>(&self, path: P) -> SlopResult<()> {
        Ok(fs::write(path, self.to_string_pretty())?)
    }
}

impl Display for Slop {
    /// Displays the [Slop] as a valid SLOP string. For a pretty-print version,
    /// see [Slop::to_string_pretty].
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.items.iter().fold(String::new(), |mut acc, (k, v)| {
            acc.push_str(k);
            acc.push_str(&v.to_string());
            acc.push('\n');
            acc
        }))
    }
}

impl FromStr for Slop {
    type Err = SlopError;

    /// Parses a valid SLOP string into a new [Slop].
    /// 
    /// Uses the same parser as [Slop::append_slop_string].
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slop = Slop::new();
        slop.append_slop_string(s)?;
        Ok(slop)
    }
}

impl IntoIterator for Slop {
    type Item = (String, SlopValue);
    type IntoIter = <HashMap<String, SlopValue> as IntoIterator>::IntoIter;

    /// Creates a consuming iterator out of the [Slop]'s KVs.
    /// This is the same iterator type as the one from [HashMap::into_iter].
    /// 
    /// ```
    /// use slop_rs::{Slop, SlopValue};
    /// 
    /// let slop_str = "
    ///     str-kv=value
    ///     list-kv{
    ///         value 1
    ///         value 2
    ///     }
    /// ";
    /// let slop: Slop = slop_str.parse().unwrap();
    /// 
    /// let vec: Vec<(String, SlopValue)> = slop.into_iter().collect();
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// Removes leading (not trailing) whitespace and a potential trailing `\r`.
// This function is zero-copy.
#[inline]
fn clean_up_line(line: &str) -> &str {
    line.strip_suffix('\r').unwrap_or(line).trim_start()
}

// Returns the parsed KV, or [None] if the line does not define a string KV.
fn parse_string_kv(line: &str) -> Option<(&str, SlopValue)> {
    let (key, value) = line.split_once('=')?;
    Some((key, value.into()))
}

// Returns the parsed KV + skip_lines, or [None] if the line does not define the
// start of a list KV.
//
// ## Panics
//
// Panics if `start_index` or `start_index + 1` is not in the range of `lines`.
fn parse_list_kv<'a>(lines: &'a Vec<&'a str>, start_index: usize)
    -> Result<Option<(&'a str, SlopValue, usize)>, SlopError>
{
    let key = if let Some(k) = clean_up_line(lines[start_index]).strip_suffix('{') {
        k
    } else {
        return Ok(None);
    };

    let mut values = vec![];

    for i in (start_index + 1)..lines.len() {
        let line = clean_up_line(lines[i]);

        if line == "}" {
            return Ok(Some((key, values.into(), i - start_index)));
        }

        values.push(line.to_string());
    }

    Err(SlopError::UnclosedList(start_index, lines[start_index].to_string()))
}
