#![doc = include_str!("../README.md")]

use {
    serde::Serialize,
    std::ops::{Range, RangeBounds},
    unicode_segmentation::{Graphemes, UnicodeSegmentation},
};

#[allow(unused_imports)]
use std::ops::{RangeFrom, RangeFull, RangeTo};
// NOTE: These are included for links in the documentation.

//--------------------------------------------------------------------------------------------------

/// String with support for Unicode graphemes
#[derive(Clone, Default, Serialize)]
pub struct GString {
    data: Vec<String>,
}

impl GString {
    /**
    Create a new empty [`GString`]

    ```
    use gstring::*;

    let s = GString::new();

    assert_eq!(s, "");
    ```
    */
    pub fn new() -> GString {
        GString::default()
    }

    /**
    Create a new [`GString`] from a [`&str`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);

    assert_eq!(s, S);
    ```
    */
    pub fn from(s: &str) -> GString {
        GString { data: graphemes(s) }
    }

    /**
    Return a slice reference to the internal graphemes

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";
    const G: &[&str] = &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"];

    let s = GString::from(S);
    let g = s.graphemes();

    assert_eq!(g, G);
    assert_eq!(g.len(), G.len());
    ```
    */
    pub fn graphemes(&self) -> &[String] {
        &self.data
    }

    /**
    Consume the [`GString`] and convert to a [`Vec`] of graphemes

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";
    const G: &[&str] = &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"];

    let g = GString::from(S).into_graphemes();

    assert_eq!(g, G);
    assert_eq!(g.len(), G.len());
    ```
    */
    pub fn into_graphemes(self) -> Vec<String> {
        self.data
    }

    /**
    Return a copy of the grapheme at `index`

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let g = GString::from(S);

    assert_eq!(g.get(0).unwrap(), "a\u{310}");
    assert_eq!(g.get(1).unwrap(), "e\u{301}");
    assert_eq!(g.get(2).unwrap(), "o\u{308}\u{332}");
    assert!(g.get(3).is_none());
    ```
    */
    pub fn get(&self, index: usize) -> Option<String> {
        (index < self.len()).then(|| self.data[index].clone())
    }

    /**
    Return the count of graphemes

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);
    assert_eq!(s.len(), 3);

    let s = GString::from("");
    assert_eq!(s.len(), 0);
    ```
    */
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /**
    Return [`true`] if the [`GString`] has zero graphemes otherwise return [`false`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from("");
    assert!(s.is_empty());

    let s = GString::from(S);
    assert!(!s.is_empty());
    ```
    */
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /**
    Return a [`Vec`] of [`char`]s

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";
    const C: &[char] = &['a', '\u{310}', 'e', '\u{301}', 'o', '\u{308}', '\u{332}'];

    let c = GString::from(S).chars();

    assert_eq!(c, C);
    assert_eq!(c.len(), C.len());
    ```
    */
    pub fn chars(&self) -> Vec<char> {
        self.data
            .iter()
            .flat_map(|x| x.chars().collect::<Vec<_>>())
            .collect()
    }

    /**
    Return a [`Vec`] of [`u8`]s

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";
    const B: &[u8] = &[0x61, 0xcc, 0x90, 0x65, 0xcc, 0x81, 0x6f, 0xcc, 0x88, 0xcc, 0xb2];

    let b = GString::from(S).bytes();

    assert_eq!(b, B);
    assert_eq!(b.len(), B.len());
    ```
    */
    pub fn bytes(&self) -> Vec<u8> {
        self.data
            .iter()
            .flat_map(|x| x.bytes().collect::<Vec<_>>())
            .collect()
    }

    /**
    Insert a string at an index

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let mut s = GString::from("a\u{310}o\u{308}\u{332}");
    s.insert(1, "e\u{301}");

    assert_eq!(s, S);
    ```
    */
    pub fn insert(&mut self, idx: usize, string: &str) {
        self.data.splice(idx..idx, graphemes(string));
    }

    /**
    Remove a grapheme at an index

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let mut s = GString::from(S);

    assert_eq!(s.remove(1), "e\u{301}");
    assert_eq!(s, "a\u{310}o\u{308}\u{332}");
    ```
    */
    pub fn remove(&mut self, idx: usize) -> GString {
        GString {
            data: vec![self.data.remove(idx)],
        }
    }

    /**
    Append a [`&str`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let mut s = GString::from("a\u{310}e\u{301}");
    s.push("o\u{308}\u{332}");

    assert_eq!(s, S);
    ```
    */
    pub fn push(&mut self, string: &str) {
        self.data.append(&mut graphemes(string));
    }

    /**
    Remove the last grapheme and return it as a new [`GString`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let mut s = GString::from(S);

    assert_eq!(s.pop().unwrap(), "o\u{308}\u{332}");
    assert_eq!(s, "a\u{310}e\u{301}");

    assert_eq!(s.pop().unwrap(), "e\u{301}");
    assert_eq!(s, "a\u{310}");

    assert_eq!(s.pop().unwrap(), "a\u{310}");
    assert_eq!(s, "");

    assert_eq!(s.pop(), None);
    assert_eq!(s, "");
    ```
    */
    pub fn pop(&mut self) -> Option<GString> {
        self.data.pop().map(|x| GString { data: vec![x] })
    }

    /**
    Replace a range with a [`&str`]

    The range can be a `a..b` [`Range<usize>`], `a..` [`RangeFrom<usize>`], `..b`
    [`RangeTo<usize>`], or `..` [`RangeFull`].

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let mut s = GString::from(S);

    s.splice(0..2, "e\u{301}a\u{310}");
    assert_eq!(s, "e\u{301}a\u{310}o\u{308}\u{332}");

    s.splice(1.., "o\u{308}\u{332}a\u{310}");
    assert_eq!(s, "e\u{301}o\u{308}\u{332}a\u{310}");

    s.splice(..1, "");
    assert_eq!(s, "o\u{308}\u{332}a\u{310}");

    s.splice(.., "");
    assert_eq!(s, "");
    ```
    */
    pub fn splice<R: RangeBounds<usize>>(&mut self, range: R, replace_with: &str) -> GString {
        GString {
            data: self.data.splice(range, graphemes(replace_with)).collect(),
        }
    }

    /**
    Remove and return a range of graphemes

    The range can be a `a..b` [`Range<usize>`], `a..` [`RangeFrom<usize>`], `..b`
    [`RangeTo<usize>`], or `..` [`RangeFull`].

    ```
    use gstring::*;

    let mut s = GString::from("a\u{310}e\u{301}o\u{308}\u{332}a\u{310}e\u{301}");

    assert_eq!(s.drain(0..2), "a\u{310}e\u{301}");
    assert_eq!(s, "o\u{308}\u{332}a\u{310}e\u{301}");

    assert_eq!(s.drain(2..), "e\u{301}");
    assert_eq!(s, "o\u{308}\u{332}a\u{310}");

    assert_eq!(s.drain(..1), "o\u{308}\u{332}");
    assert_eq!(s, "a\u{310}");

    assert_eq!(s.drain(..), "a\u{310}");
    assert_eq!(s, "");
    ```
    */
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> GString {
        GString {
            data: self.data.drain(range).collect(),
        }
    }

    /**
    Create a new [`GString`] from an `a..b` [`Range<usize>`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);

    assert_eq!(s.slice(0..1), "a\u{310}");
    assert_eq!(s.slice(1..2), "e\u{301}");
    assert_eq!(s.slice(2..3), "o\u{308}\u{332}");
    assert_eq!(s.slice(0..2), "a\u{310}e\u{301}");
    assert_eq!(s.slice(1..3), "e\u{301}o\u{308}\u{332}");
    assert_eq!(s.slice(0..3), S);
    ```

    See also the `GString::index` method.
    */
    pub fn slice(&self, range: Range<usize>) -> GString {
        GString {
            data: self.data[range].to_vec(),
        }
    }

    /**
    Create a [`GStringRefIter`] for iterating graphemes by reference

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);
    let mut i = s.iter();

    assert_eq!(i.next().unwrap(), "a\u{310}");
    assert_eq!(i.next().unwrap(), "e\u{301}");
    assert_eq!(i.next().unwrap(), "o\u{308}\u{332}");
    assert_eq!(i.next(), None);
    ```

    See also the [`GString::into_iter`] method.
    */
    pub fn iter(&self) -> GStringRefIter {
        GStringRefIter {
            gstring: self,
            idx: 0,
        }
    }

    /**
    Consume the [`GString`] and convert into a [`GStringIter`] for iterating graphemes

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);
    let mut i = s.into_iter();

    assert_eq!(i.next().unwrap(), "a\u{310}");
    assert_eq!(i.next().unwrap(), "e\u{301}");
    assert_eq!(i.next().unwrap(), "o\u{308}\u{332}");
    assert_eq!(i.next(), None);
    ```

    See also the [`GString::iter`] method.
    */
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> GStringIter {
        GStringIter {
            gstring: self,
            idx: 0,
        }
    }
}

//--------------------------------------------------------------------------------------------------

impl std::fmt::Display for GString {
    /**
    Print a [`GString`] directly in [`print`], [`println`], [`eprint`], [`eprintln`], and [`write`]
    macros or convert to a [`String`] using the [`format`] macro [`to_string`][ToString::to_string]
    method

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);

    assert_eq!(format!("{s}"), S);
    assert_eq!(format!("{}", s), S);
    assert_eq!(s.to_string(), S);
    ```
    */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data.join(""))
    }
}

impl std::fmt::Debug for GString {
    /**
    Debug print a [`GString`] in [`format`], [`print`], [`println`], [`write`], [`writeln`], etc
    macros

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    assert_eq!(
        format!("{:?}", GString::from(S)),
        format!("{:?}", S),
    );
    ```
    */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl<I> std::ops::Index<I> for GString
where
    I: std::slice::SliceIndex<[String]>,
{
    type Output = I::Output;

    /**
    Index a slice of [`GString`]'s graphemes with a [`usize`] index, `a..b` [`Range<usize>`], `a..`
    [`RangeFrom<usize>`], `..b` [`RangeTo<usize>`], or `..` [`RangeFull`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";
    const G: &[&str] = &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"];

    let s = GString::from(S);

    assert_eq!(&s[0], G[0]);
    assert_eq!(&s[1], G[1]);
    assert_eq!(&s[2], G[2]);

    for start in 0..3 {
        for stop in 1..4 {
            if stop > start {
                assert_eq!(&s[start..stop], G[start..stop].to_vec());
                assert_eq!(&s[..stop], G[..stop].to_vec());
            }
        }
        assert_eq!(&s[start..], G[start..].to_vec());
    }
    assert_eq!(&s[..], G);
    ```

    See also the [`GString::slice`] method.
    */
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

impl std::cmp::PartialEq<GString> for GString {
    /**
    Compare two [`GString`]s

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s1 = GString::from(S);
    let s2 = GString::from(S);
    let s3 = GString::from(S);

    assert_eq!(s1, s2);
    assert_ne!(s3, GString::from(""));
    ```
    */
    fn eq(&self, other: &GString) -> bool {
        self.data == other.data
    }
}

impl std::cmp::PartialEq<GString> for &GString {
    /**
    Compare a [`GString`] to a `&`[`GString`] (or two `&`[`GString`]s)

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s1 = GString::from(S);
    let s2 = GString::from(S);
    let empty = GString::from("");

    assert_eq!(&s1, s2);
    assert_ne!(&s1, empty);

    assert_eq!(&s1, &s2);
    assert_ne!(&s1, &empty);
    ```
    */
    fn eq(&self, other: &GString) -> bool {
        self.data == other.data
    }
}

impl std::cmp::PartialEq<String> for GString {
    /**
    Compare a [`GString`] to a [`String`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);

    assert_eq!(s, S.to_string());
    assert_ne!(s, String::new());
    ```
    */
    fn eq(&self, other: &String) -> bool {
        self == GString::from(other)
    }
}

impl std::cmp::PartialEq<&str> for GString {
    /**
    Compare a [`GString`] to a [`&str`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);

    assert_eq!(s, S);
    assert_ne!(s, "");
    ```
    */
    fn eq(&self, other: &&str) -> bool {
        self == GString::from(other)
    }
}

impl std::cmp::PartialEq<str> for GString {
    /**
    Compare a [`GString`] to a [`str`]

    ```
    use gstring::*;

    let s = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");

    assert_eq!(s, "a\u{310}e\u{301}o\u{308}\u{332}");
    assert_ne!(s, "");
    ```
    */
    fn eq(&self, other: &str) -> bool {
        self == GString::from(other)
    }
}

//--------------------------------------------------------------------------------------------------

/// Created by [`GString::iter`] to iterate graphemes by reference
pub struct GStringRefIter<'a> {
    gstring: &'a GString,
    idx: usize,
}

impl Iterator for GStringRefIter<'_> {
    type Item = String;

    /**
    Get the next grapheme by reference

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);
    let mut i = s.iter();

    assert_eq!(i.next().unwrap(), "a\u{310}");
    assert_eq!(i.next().unwrap(), "e\u{301}");
    assert_eq!(i.next().unwrap(), "o\u{308}\u{332}");
    assert_eq!(i.next(), None);
    ```
    */
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.gstring.data.get(self.idx).cloned();
        self.idx += 1;
        r
    }
}

//--------------------------------------------------------------------------------------------------

/// Created by [`GString::into_iter`] to iterate graphemes
pub struct GStringIter {
    gstring: GString,
    idx: usize,
}

impl Iterator for GStringIter {
    type Item = String;

    /**
    Get the next grapheme

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let mut i = GString::from(S).into_iter();

    assert_eq!(i.next().unwrap(), "a\u{310}");
    assert_eq!(i.next().unwrap(), "e\u{301}");
    assert_eq!(i.next().unwrap(), "o\u{308}\u{332}");
    assert_eq!(i.next(), None);
    ```
    */
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.gstring.data.get(self.idx).cloned();
        self.idx += 1;
        r
    }
}

//--------------------------------------------------------------------------------------------------

/**
Trait for easy conversion to [`GString`], [`Vec`] of graphemes, or [`Graphemes`] iterator from
custom or foreign types like [`&str`] and [`String`]

```
use gstring::*;

// From &str

const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";
const G: &[&str] = &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"];

// &str => GString
let s = S.gstring();
assert_eq!(s, S);

// &str => Vec<String>
let g = S.graphemes();
assert_eq!(g, G);

// &str => Graphemes
let mut g = S.graphemes_iter();
assert_eq!(g.count(), G.len());

// From String

let a = String::from(S);

// String => GString
let s = a.gstring();
assert_eq!(s, S);

// String => Vec<String>
let g = a.graphemes();
assert_eq!(g, G);

// String => Graphemes
let mut g = a.graphemes_iter();
assert_eq!(g.count(), G.len());
```
*/
pub trait GStringTrait {
    /// Create a new [`GString`]
    fn gstring(&self) -> GString;

    /// Create a new [`Vec`] of graphemes
    fn graphemes(&self) -> Vec<String>;

    /// Return a [`Graphemes`] iterator
    fn graphemes_iter(&self) -> Graphemes;
}

impl GStringTrait for String {
    /// Create a new [`GString`] from a [`String`]
    fn gstring(&self) -> GString {
        GString::from(self)
    }

    /// Create a new [`Vec`] of graphemes from a [`String`]
    fn graphemes(&self) -> Vec<String> {
        self.gstring().into_graphemes()
    }

    /// Return a [`Graphemes`] iterator from a [`String`]
    fn graphemes_iter(&self) -> Graphemes {
        UnicodeSegmentation::graphemes(self.as_str(), true)
    }
}

impl GStringTrait for &str {
    /// Create a new [`GString`] from a [`&str`]
    fn gstring(&self) -> GString {
        GString::from(self)
    }

    /// Create a new [`Vec`] of graphemes from a [`&str`]
    fn graphemes(&self) -> Vec<String> {
        self.gstring().into_graphemes()
    }

    /// Return a [`Graphemes`] iterator from a [`&str`]
    fn graphemes_iter(&self) -> Graphemes {
        UnicodeSegmentation::graphemes(*self, true)
    }
}

//--------------------------------------------------------------------------------------------------

/**
Create a [`Vec`] of graphemes from a [`&str`]

```
use gstring::*;

const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";
const G: &[&str] = &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"];

let g = graphemes(S);

assert_eq!(g, G);
```
*/
pub fn graphemes(s: &str) -> Vec<String> {
    s.graphemes(true).map(String::from).collect()
}
