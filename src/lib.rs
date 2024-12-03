use {
    serde::Serialize,
    std::ops::{Range, RangeBounds},
    unicode_segmentation::UnicodeSegmentation,
};

//--------------------------------------------------------------------------------------------------

/// String with support for Unicode graphemes
#[derive(Clone, Serialize)]
pub struct GString {
    data: Vec<String>,
}

impl GString {
    /**
    Create a new [`GString`] from a `&str`

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

    let g = GString::from("a\u{310}e\u{301}o\u{308}\u{332}")
        .graphemes()
        .to_vec();

    assert_eq!(g.len(), 3);
    assert_eq!(g, ["a\u{310}", "e\u{301}", "o\u{308}\u{332}"]);
    ```
    */
    pub fn graphemes(&self) -> &[String] {
        &self.data
    }

    /**
    Consume the [`GString`] and convert to a [`Vec`] of graphemes

    ```
    use gstring::*;

    let g = GString::from("a\u{310}e\u{301}o\u{308}\u{332}")
        .into_graphemes();

    assert_eq!(g.len(), 3);
    assert_eq!(g, ["a\u{310}", "e\u{301}", "o\u{308}\u{332}"]);
    ```
    */
    pub fn into_graphemes(self) -> Vec<String> {
        self.data
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

    let s = GString::from("");

    assert!(s.is_empty());
    ```
    */
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /**
    Return a [`Vec`] of [`char`]s

    ```
    use gstring::*;

    let c = GString::from("a\u{310}e\u{301}o\u{308}\u{332}").chars();

    assert_eq!(c.len(), 7);
    assert_eq!(c, ['a', '\u{310}', 'e', '\u{301}', 'o', '\u{308}', '\u{332}']);
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

    let b = GString::from("a\u{310}e\u{301}o\u{308}\u{332}").bytes();

    assert_eq!(b.len(), 11);
    assert_eq!(
        b,
        [0x61, 0xcc, 0x90, 0x65, 0xcc, 0x81, 0x6f, 0xcc, 0x88, 0xcc, 0xb2],
    );
    ```
    */
    pub fn bytes(&self) -> Vec<u8> {
        self.data
            .iter()
            .flat_map(|x| x.bytes().collect::<Vec<_>>())
            .collect()
    }

    /*
    Insert a string at an index

    ```
    use gstring::*;

    let mut s = GString::from("a\u{310}o\u{308}\u{332}");

    s.insert(1, "e\u{301}");

    assert_eq!(s, "a\u{310}e\u{301}o\u{308}\u{332}");
    ```
    */
    pub fn insert(&mut self, idx: usize, string: &str) {
        for i in graphemes(string).into_iter().rev() {
            self.data.insert(idx, i);
        }
    }

    /**
    Remove a grapheme at an index

    ```
    use gstring::*;

    let mut s = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");

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
    Append a string

    ```
    use gstring::*;

    let mut s = GString::from("a\u{310}e\u{301}");

    s.push("o\u{308}\u{332}");

    assert_eq!(s, "a\u{310}e\u{301}o\u{308}\u{332}");
    ```
    */
    pub fn push(&mut self, string: &str) {
        for i in graphemes(string) {
            self.data.push(i);
        }
    }

    /**
    Remove the last grapheme

    ```
    use gstring::*;

    let mut s = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");

    assert_eq!(s.pop().unwrap(), "o\u{308}\u{332}");
    assert_eq!(s, "a\u{310}e\u{301}");
    ```
    */
    pub fn pop(&mut self) -> Option<GString> {
        self.data.pop().map(|x| GString { data: vec![x] })
    }

    /**
    Replace a range with the given string

    ```
    use gstring::*;

    let mut s = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");

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
    Delete and return a range

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
    Create a new [`GString`] from a given range

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
    assert_eq!(s, S);
    ```
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

    let s = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");
    let mut i = s.iter();

    assert_eq!(i.next().unwrap(), "a\u{310}");
    assert_eq!(i.next().unwrap(), "e\u{301}");
    assert_eq!(i.next().unwrap(), "o\u{308}\u{332}");
    assert_eq!(i.next(), None);
    ```
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

    let mut i = GString::from("a\u{310}e\u{301}o\u{308}\u{332}").into_iter();

    assert_eq!(i.next().unwrap(), "a\u{310}");
    assert_eq!(i.next().unwrap(), "e\u{301}");
    assert_eq!(i.next().unwrap(), "o\u{308}\u{332}");
    assert_eq!(i.next(), None);
    ```
    */
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> GStringIter {
        GStringIter {
            gstring: self,
            idx: 0,
        }
    }
}

impl std::fmt::Display for GString {
    /**
    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    assert_eq!(format!("{}", GString::from(S)), S);
    assert_eq!(GString::from(S).to_string(), S);
    ```
    */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data.join(""))
    }
}

impl std::fmt::Debug for GString {
    /**
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
    ```
    use gstring::*;

    let s = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");

    const GRAPHEMES: &[&str] = &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"];

    assert_eq!(&s[0], GRAPHEMES[0]);
    assert_eq!(&s[1], GRAPHEMES[1]);
    assert_eq!(&s[2], GRAPHEMES[2]);

    for start in 0..3 {
        for stop in 1..4 {
            if stop > start {
                assert_eq!(&s[start..stop], GRAPHEMES[start..stop].to_vec());
                assert_eq!(&s[..stop], GRAPHEMES[..stop].to_vec());
            }
        }
        assert_eq!(&s[start..], GRAPHEMES[start..].to_vec());
    }
    assert_eq!(&s[..], GRAPHEMES);
    ```
    */
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

impl std::cmp::PartialEq<GString> for GString {
    /**
    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    assert_eq!(GString::from(S), GString::from(S));
    assert_ne!(GString::from(S), GString::from(""));
    ```
    */
    fn eq(&self, other: &GString) -> bool {
        self.data == other.data
    }
}

impl std::cmp::PartialEq<GString> for &GString {
    /**
    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let s = GString::from(S);

    assert_eq!(&s, GString::from(S));
    assert_ne!(&s, GString::from(""));
    ```
    */
    fn eq(&self, other: &GString) -> bool {
        self.data == other.data
    }
}

impl std::cmp::PartialEq<String> for GString {
    /**
    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    assert_eq!(GString::from(S), S.to_string());
    assert_ne!(GString::from(S), String::new());
    ```
    */
    fn eq(&self, other: &String) -> bool {
        self == GString::from(other)
    }
}

impl std::cmp::PartialEq<&str> for GString {
    /**
    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    assert_eq!(GString::from(S), S);
    assert_ne!(GString::from(S), "");
    ```
    */
    fn eq(&self, other: &&str) -> bool {
        self == GString::from(other)
    }
}

//--------------------------------------------------------------------------------------------------

/// Created by [`GString.iter`] to iterate graphemes by reference
pub struct GStringRefIter<'a> {
    gstring: &'a GString,
    idx: usize,
}

impl Iterator for GStringRefIter<'_> {
    type Item = String;

    /**
    ```
    use gstring::*;

    let s = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");
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

/// Created by [`GString.into_iter`] to iterate graphemes
pub struct GStringIter {
    gstring: GString,
    idx: usize,
}

impl Iterator for GStringIter {
    type Item = String;

    /**
    ```
    use gstring::*;

    let mut i = GString::from("a\u{310}e\u{301}o\u{308}\u{332}").into_iter();

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
Convert a [`&str`] into a [`Vec`] of graphemes

```
use gstring::*;

let g = graphemes("a\u{310}e\u{301}o\u{308}\u{332}");

assert_eq!(g, ["a\u{310}", "e\u{301}", "o\u{308}\u{332}"]);
```
*/
pub fn graphemes(s: &str) -> Vec<String> {
    s.graphemes(true).map(String::from).collect()
}
