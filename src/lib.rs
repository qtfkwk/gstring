#![doc = include_str!("../README.md")]

use {
    anyhow::{Result, anyhow},
    serde::Serialize,
    std::{
        fmt::Write,
        ops::{Index, Range, RangeBounds},
        slice::SliceIndex,
    },
    unicode_segmentation::{Graphemes, UnicodeSegmentation},
};

//--------------------------------------------------------------------------------------------------

#[derive(Clone, Default, PartialEq, Serialize)]
pub struct Grapheme {
    data: String,
}

impl Grapheme {
    /**
    Create a new [`Grapheme`] from a [`&str`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}";

    let g = Grapheme::from(S).unwrap();

    assert_eq!(g, S);
    ```

    # Errors

    Returns an error if the given `&str` does not contain exactly 1 grapheme
    */
    pub fn from(s: &str) -> Result<Grapheme> {
        let mut g = graphemes(s);
        match g.len() {
            1 => Ok(g.remove(0)),
            _ => Err(anyhow!("Input must contain 1 grapheme")),
        }
    }

    /**
    Return a [`Vec`] of [`char`]s

    ```
    use gstring::*;

    const S: &str = "a\u{310}";
    const C: &[char] = &['a', '\u{310}'];

    let c = Grapheme::from(S).unwrap().chars();

    assert_eq!(c, C);
    assert_eq!(c.len(), C.len());
    ```
    */
    #[must_use]
    pub fn chars(&self) -> Vec<char> {
        self.data.chars().collect()
    }

    /**
    Return a [`Vec`] of [`u8`]s

    ```
    use gstring::*;

    const S: &str = "a\u{310}";
    const B: &[u8] = &[0x61, 0xcc, 0x90];

    let b = Grapheme::from(S).unwrap().bytes();

    assert_eq!(b, B);
    assert_eq!(b.len(), B.len());
    ```
    */
    #[must_use]
    pub fn bytes(&self) -> Vec<u8> {
        self.data.bytes().collect()
    }

    /**
    Return a reference to the internal string
    ```
    use gstring::*;

    const S: &str = "a\u{310}";

    let g = Grapheme::from(S).unwrap();

    assert_eq!(g.as_str(), S);
    ```
    */
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.data
    }
}

impl std::fmt::Display for Grapheme {
    /**
    Print a [`Grapheme`] directly in [`print`], [`println`], [`eprint`], [`eprintln`], and [`write`]
    macros or convert to a [`String`] using the [`format`] macro [`to_string`][ToString::to_string]
    method

    ```
    use gstring::*;

    const S: &str = "a\u{310}";

    let g = Grapheme::from(S).unwrap();

    assert_eq!(format!("{g}"), S);
    assert_eq!(format!("{}", g), S);
    assert_eq!(g.to_string(), S);
    ```
    */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl std::fmt::Debug for Grapheme {
    /**
    Debug print a [`Grapheme`] in [`format`], [`print`], [`println`], [`write`], [`writeln`], etc
    macros

    ```
    use gstring::*;

    const S: &str = "a\u{310}";

    let s = GString::from(S);

    assert_eq!(
        format!("{:?}", s[0]),
        format!("{:?}", S),
    );
    ```
    */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl std::cmp::PartialEq<&str> for Grapheme {
    /**
    Compare a [`Grapheme`] to a [`&str`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}";

    let s = Grapheme::from(S).unwrap();

    assert_eq!(s, S);
    assert_ne!(s, "");
    ```
    */
    fn eq(&self, other: &&str) -> bool {
        self.data == *other
    }
}

impl std::cmp::PartialEq<str> for Grapheme {
    /**
    Compare a [`Grapheme`] to a [`str`]

    ```
    use gstring::*;

    const S: &str = "a\u{310}";

    let s = Grapheme::from(S).unwrap();

    assert_eq!(s, S);
    assert_ne!(s, "");
    ```
    */
    fn eq(&self, other: &str) -> bool {
        self.data == other
    }
}

//--------------------------------------------------------------------------------------------------

/// String with support for Unicode graphemes
#[derive(Clone, Default, Serialize)]
pub struct GString {
    data: Vec<Grapheme>,
    shape: Vec<usize>,
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
    #[must_use]
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
    #[must_use]
    pub fn from(s: &str) -> GString {
        let data = graphemes(s);
        let shape = calc_shape(&data);
        GString { data, shape }
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
    #[must_use]
    pub fn graphemes(&self) -> &[Grapheme] {
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
    #[must_use]
    pub fn into_graphemes(self) -> Vec<Grapheme> {
        self.data
    }

    /**
    Returns the index of the first grapheme of this string slice that matches the pattern

    ```
    use gstring::*;

    let g = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");

    assert_eq!(g.find(&GString::from("a\u{310}")), Some(0));
    assert_eq!(g.find(&GString::from("e\u{301}")), Some(1));
    assert_eq!(g.find(&GString::from("o\u{308}\u{332}")), Some(2));
    assert!(g.find(&GString::from("nonexistent")).is_none());
    ```
    */
    #[must_use]
    pub fn find(&self, pattern: &GString) -> Option<usize> {
        self.data
            .as_slice()
            .windows(pattern.len())
            .enumerate()
            .find(|(_, g)| g == &pattern.data)
            .map(|(i, _)| i)
    }

    /**
    Returns the index of the first grapheme of this string slice that matches the pattern

    ```
    use gstring::*;

    let g = GString::from("a\u{310}e\u{301}o\u{308}\u{332}");

    assert_eq!(g.find_str("a\u{310}"), Some(0));
    assert_eq!(g.find_str("e\u{301}"), Some(1));
    assert_eq!(g.find_str("o\u{308}\u{332}"), Some(2));
    assert!(g.find_str("nonexistent").is_none());
    ```
    */
    #[must_use]
    pub fn find_str(&self, pattern: &str) -> Option<usize> {
        self.find(&pattern.gstring())
    }

    /**
    Returns the index of the first grapheme of this string slice that matches the pattern after `n`
    graphemes

    ```
    use gstring::*;

    let g = GString::from("abc abc");

    assert_eq!(g.find_from(0, &GString::from("abc")), Some(0));
    assert_eq!(g.find_from(1, &GString::from("abc")), Some(4));
    assert!(g.find_from(0, &GString::from("nonexistent")).is_none());
    ```
    */
    #[must_use]
    pub fn find_from(&self, n: usize, pattern: &GString) -> Option<usize> {
        self.data[n..]
            .windows(pattern.len())
            .enumerate()
            .find(|(_, g)| g == &pattern.data)
            .map(|(i, _)| i + n)
    }

    /**
    Returns the index of the first grapheme of this string slice that matches the pattern after `n`
    graphemes

    ```
    use gstring::*;

    let g = GString::from("abc abc");

    assert_eq!(g.find_from_str(0, "abc"), Some(0));
    assert_eq!(g.find_from_str(1, "abc"), Some(4));
    assert!(g.find_from_str(0, "nonexistent").is_none());
    ```
    */
    #[must_use]
    pub fn find_from_str(&self, n: usize, pattern: &str) -> Option<usize> {
        self.find_from(n, &pattern.gstring())
    }

    /**
    Returns the index of the first grapheme of this string slice that matches the pattern before `n`
    graphemes

    ```
    use gstring::*;

    let g = GString::from("abc abc");

    assert_eq!(g.find_prev_from(7, &GString::from("abc")), Some(4));
    assert_eq!(g.find_prev_from(4, &GString::from("abc")), Some(0));
    assert!(g.find_prev_from(7, &GString::from("nonexistent")).is_none());
    ```
    */
    #[must_use]
    pub fn find_prev_from(&self, n: usize, pattern: &GString) -> Option<usize> {
        let mut pattern = pattern.clone().into_graphemes();
        pattern.reverse();

        let length = self.len();
        let n = length - n;

        let mut data = self.data.clone();
        data.reverse();

        data[n..]
            .windows(pattern.len())
            .enumerate()
            .find(|(_, g)| g == &pattern)
            .map(|(i, _)| length - i - n - pattern.len())
    }

    /**
    Returns the index of the first grapheme of this string slice that matches the pattern before `n`
    graphemes

    ```
    use gstring::*;

    let g = GString::from("abc abc");

    assert_eq!(g.find_prev_from_str(7, "abc"), Some(4));
    assert_eq!(g.find_prev_from_str(4, "abc"), Some(0));
    assert!(g.find_prev_from_str(7, "nonexistent").is_none());
    ```
    */
    #[must_use]
    pub fn find_prev_from_str(&self, n: usize, pattern: &str) -> Option<usize> {
        self.find_prev_from(n, &pattern.gstring())
    }

    /**
    Return a reference to the grapheme at `index`

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
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Grapheme> {
        (index < self.len()).then(|| &self.data[index])
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn chars(&self) -> Vec<char> {
        self.data.iter().flat_map(Grapheme::chars).collect()
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
    #[must_use]
    pub fn bytes(&self) -> Vec<u8> {
        self.data.iter().flat_map(Grapheme::bytes).collect()
    }

    /**
    Split into lines as a [`Vec`] of [`GString`]s

    ```
    use gstring::*;

    assert_eq!(GString::from("abc\ndef").lines(), &["abc\n", "def"]);
    assert_eq!(GString::from("abc\n").lines(), &["abc\n", ""]);
    assert_eq!(GString::from("\ndef").lines(), &["\n", "def"]);
    ```

    Note that unlike [`str::lines`], this method includes the original newline graphemes at the end
    of each line.
    */
    #[must_use]
    pub fn lines(&self) -> Vec<GString> {
        lines(&self.data)
    }

    /**
    Return the coordinates `(row, column)` for a given position

    ```
    use gstring::*;

    let g = GString::from("abc\ndef");

    assert_eq!(g.coordinates(0), Some((0, 0)));
    assert_eq!(g.coordinates(1), Some((0, 1)));
    assert_eq!(g.coordinates(2), Some((0, 2)));
    assert_eq!(g.coordinates(3), Some((0, 3)));
    assert_eq!(g.coordinates(4), Some((1, 0)));
    assert_eq!(g.coordinates(5), Some((1, 1)));
    assert_eq!(g.coordinates(6), Some((1, 2)));
    assert_eq!(g.coordinates(7), Some((1, 3)));
    assert_eq!(g.coordinates(8), None);
    ```

    Note that newlines are located at the end of each line.
    Also a valid coordinate exists at a [`GString`]'s position equal to its length, however no valid
    coordinate exists for any greater position.

    See also the [`GString::position`] method.
    */
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn coordinates(&self, position: usize) -> Option<(usize, usize)> {
        (position <= self.len()).then(|| {
            let n = newline_indices(&self.data[..position]);
            let row = n.len();
            let column = if row == 0 {
                position
            } else {
                position - n.last().unwrap() - 1
            };
            (row, column)
        })
    }

    /**
    Return the position for given coordinates `(row, column)`

    ```
    use gstring::*;

    /*
      0 1 2 3   column
    0 a b c \n
      0 1 2 3   position

      0 1 2 3   column
    1 d e f
      4 5 6 7   position
    */

    let g = GString::from("abc\ndef");

    assert_eq!(g.position((0, 0)), Some(0));
    assert_eq!(g.position((0, 1)), Some(1));
    assert_eq!(g.position((0, 2)), Some(2));
    assert_eq!(g.position((0, 3)), Some(3));
    assert_eq!(g.position((0, 4)), None);
    assert_eq!(g.position((1, 0)), Some(4));
    assert_eq!(g.position((1, 1)), Some(5));
    assert_eq!(g.position((1, 2)), Some(6));
    assert_eq!(g.position((1, 3)), Some(7));
    assert_eq!(g.position((1, 4)), None);
    assert_eq!(g.position((2, 0)), None);

    let g = GString::from("");

    assert_eq!(g.position((0, 0)), Some(0));
    assert_eq!(g.position((0, 1)), None);
    assert_eq!(g.position((1, 0)), None);
    ```

    Note that newlines are located at the end of each line.
    Also a valid coordinate exists at a [`GString`]'s position equal to its length, however no valid
    coordinate exists for any greater position.

    See also the [`GString::coordinates`] method.
    */
    #[must_use]
    pub fn position(&self, coordinates: (usize, usize)) -> Option<usize> {
        if self.is_empty() {
            // Empty, so only valid coordinate is `(0, 0)` and position is `0`
            (coordinates == (0, 0)).then_some(0)
        } else {
            // Not empty...
            match coordinates {
                (0, 0) => {
                    // Shortcut `(0, 0)` to `0`
                    Some(0)
                }
                (row, column) => {
                    // Not `(0, 0)`...
                    let newlines = self.newlines();
                    let last_row = newlines.len();
                    if row <= last_row {
                        // Valid row
                        let lines = self.lines();
                        let last_column = lines[row].len();
                        if row == last_row && column == last_column {
                            // Last row and last column
                            Some(self.len())
                        } else if column < last_column {
                            // Valid column
                            // Sum lengths of prior lines and add the column
                            Some(lines[..row].iter().map(GString::len).sum::<usize>() + column)
                        } else {
                            // Invalid column
                            None
                        }
                    } else {
                        // Invalid row
                        None
                    }
                }
            }
        }
    }

    /**
    Return the indices of all newlines

    ```
    use gstring::*;

    assert_eq!(GString::from("abc\ndef").newlines(), &[3]);
    assert_eq!(GString::from("abc\ndef\n").newlines(), &[3, 7]);
    assert_eq!(GString::from("abc").newlines(), &[]);
    assert_eq!(GString::from("").newlines(), &[]);
    assert_eq!(GString::from("\n").newlines(), &[0]);
    assert_eq!(GString::from("\n\n").newlines(), &[0, 1]);
    ```
    */
    #[must_use]
    pub fn newlines(&self) -> Vec<usize> {
        newline_indices(&self.data)
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
    pub fn insert(&mut self, index: usize, string: &str) {
        let _ = self.splice(index..index, string);
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
    pub fn remove(&mut self, index: usize) -> Grapheme {
        let r = self.data.remove(index);
        self.shape = calc_shape(&self.data);
        r
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
        self.shape = calc_shape(&self.data);
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
    pub fn pop(&mut self) -> Option<Grapheme> {
        self.data.pop()
    }

    /**
    Replace a range with a [`&str`]

    The range can be a `a..b` [`Range<usize>`], `a..` [`RangeFrom<usize>`], `..b`
    [`RangeTo<usize>`], or `..` [`RangeFull`].

    ```
    use gstring::*;

    const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

    let mut s = GString::from(S);

    assert_eq!(s.splice(0..2, "e\u{301}a\u{310}"), "a\u{310}e\u{301}");
    assert_eq!(s, "e\u{301}a\u{310}o\u{308}\u{332}");

    assert_eq!(s.splice(1.., "o\u{308}\u{332}a\u{310}"), "a\u{310}o\u{308}\u{332}");
    assert_eq!(s, "e\u{301}o\u{308}\u{332}a\u{310}");

    assert_eq!(s.splice(..1, ""), "e\u{301}");
    assert_eq!(s, "o\u{308}\u{332}a\u{310}");

    assert_eq!(s.splice(.., ""), "o\u{308}\u{332}a\u{310}");
    assert_eq!(s, "");
    ```

    [`RangeFrom<usize>`]: std::ops::RangeFrom

    [`RangeTo<usize>`]: std::ops::RangeTo

    [`RangeFull`]: std::ops::RangeFull
    */
    #[must_use]
    pub fn splice<R: RangeBounds<usize>>(&mut self, range: R, replace_with: &str) -> GString {
        let data = self
            .data
            .splice(range, graphemes(replace_with))
            .collect::<Vec<_>>();
        let shape = calc_shape(&data);
        self.shape = calc_shape(&self.data);
        GString { data, shape }
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

    [`RangeFrom<usize>`]: std::ops::RangeFrom

    [`RangeTo<usize>`]: std::ops::RangeTo

    [`RangeFull`]: std::ops::RangeFull
    */
    #[must_use]
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> GString {
        let data = self.data.drain(range).collect::<Vec<_>>();
        let shape = calc_shape(&data);
        self.shape = calc_shape(&self.data);
        GString { data, shape }
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

    See also the [`GString::index`] method.
    */
    #[must_use]
    pub fn slice(&self, range: Range<usize>) -> GString {
        let data = self.data[range].to_vec();
        let shape = calc_shape(&data);
        GString { data, shape }
    }

    /**
    Return a reference to the "shape" of the content

    - Length of the shape: Number of lines
    - Values: Maximum column index for each line (including the newline)

    | Line    | Count | Row Index | Max Column Index |
    |---------|------:|----------:|-----------------:|
    | `\n`    |     1 |         0 |                0 |
    | `a\n`   |     2 |         1 |                1 |
    | `bc\n`  |     3 |         2 |                2 |
    | `d\n`   |     4 |         3 |                1 |
    | `efg\n` |     5 |         4 |                3 |
    | `\n`    |     6 |         5 |                0 |

    ```
    use gstring::*;

    let s = GString::from("\na\nbc\nd\nefg\n");

    let shape = s.shape();

    assert_eq!(shape, &[0, 1, 2, 1, 3, 0]);

    // There are 6 lines
    assert_eq!(shape.len(), 6);

    // Max column index of the 5th line is 3
    assert_eq!(shape[4], 3);
    ```
    */
    #[must_use]
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /**
    Generate string showing the row, column, and position for each grapheme

    ```
    use gstring::*;

    let s = GString::from("a\nbc\ndef\nghij");

    let d = "  \
      0 1 2 3 4

      0 1
    0 a \\n
      0 1

      0 1 2
    1 b c \\n
      2 3 4

      0 1 2 3
    2 d e f \\n
      5 6 7 8

      0 1 2 3 4
    3 g h i j
      0 1 1 1 1
      9 0 1 2 3

    ";

    assert_eq!(s.shape_string(), d);
    ```

    # Notes

    1. There is a column header at the top to show the column header for the longest row.
    2. Each content row has:
        - Row column header above
        - Row index to the left
        - Position (offset) below
    3. The last row shows the column and position one grapheme past the end.
    */
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn shape_string(&self) -> String {
        let mut r = String::new();

        // Column header at top
        let max_column = *self
            .shape
            .iter()
            .max()
            .unwrap()
            .max(&(*self.shape.last().unwrap() + 1));
        let last_row = self.shape.len() - 1;
        let row_width = n_digits(last_row);
        let row_space = " ".repeat(row_width);
        let e = n_digits(max_column);
        for n in 0..e {
            writeln!(
                r,
                "{row_space} {}",
                (0..=max_column)
                    .map(|x| { format!("{x:0e$}").chars().nth(n).unwrap().to_string() })
                    .collect::<Vec<_>>()
                    .join(" "),
            )
            .unwrap();
        }
        writeln!(r).unwrap();

        // Content rows
        let mut position = 0;
        for (row, line) in self.lines().iter().enumerate() {
            // Row column header above
            let max_column = self.shape[row] + usize::from(row == last_row);
            let e = n_digits(max_column + 1);
            for n in 0..e {
                writeln!(
                    r,
                    "{row_space} {}",
                    (0..=max_column)
                        .map(|x| { format!("{x:0e$}").chars().nth(n).unwrap().to_string() })
                        .collect::<Vec<_>>()
                        .join(" "),
                )
                .unwrap();
            }

            // Content row
            writeln!(
                r,
                "{row:01$} {}",
                line.iter()
                    .map(|g| match g.as_str() {
                        "\n" => "\\n",
                        "\r\n" => "\\r\\n",
                        _ => g.as_str(),
                    })
                    .collect::<Vec<_>>()
                    .join(" "),
                row_width,
            )
            .unwrap();

            // Position (offset) below
            let a = position;
            let b = position + max_column;
            let e = n_digits(b + 1);
            for n in 0..e {
                writeln!(
                    r,
                    "{row_space} {}",
                    (a..=b)
                        .map(|x| { format!("{x:0e$}").chars().nth(n).unwrap().to_string() })
                        .collect::<Vec<_>>()
                        .join(" "),
                )
                .unwrap();
            }
            writeln!(r).unwrap();

            position += self.shape[row] + 1;
        }
        r
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
    #[allow(clippy::iter_without_into_iter)]
    #[must_use]
    pub fn iter(&self) -> GStringRefIter<'_> {
        GStringRefIter {
            gstring: self,
            index: 0,
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
    #[must_use]
    pub fn into_iter(self) -> GStringIter {
        GStringIter {
            gstring: self,
            index: 0,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Implementations

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
        for g in &self.data {
            write!(f, "{g}")?;
        }
        Ok(())
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

impl<I> Index<I> for GString
where
    I: SliceIndex<[Grapheme]>,
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

    [`RangeFrom<usize>`]: std::ops::RangeFrom

    [`RangeTo<usize>`]: std::ops::RangeTo

    [`RangeFull`]: std::ops::RangeFull
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
    index: usize,
}

impl<'a> Iterator for GStringRefIter<'a> {
    type Item = &'a Grapheme;

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
        let r = self.gstring.data.get(self.index);
        self.index += 1;
        r
    }
}

//--------------------------------------------------------------------------------------------------

/// Created by [`GString::into_iter`] to iterate graphemes
pub struct GStringIter {
    gstring: GString,
    index: usize,
}

impl Iterator for GStringIter {
    type Item = Grapheme;

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
        let r = self.gstring.data.get(self.index).cloned();
        self.index += 1;
        r
    }
}

//--------------------------------------------------------------------------------------------------
// Traits

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
    fn graphemes(&self) -> Vec<Grapheme>;

    /// Return a [`Graphemes`] iterator
    fn graphemes_iter(&self) -> Graphemes<'_>;
}

impl GStringTrait for String {
    /// Create a new [`GString`] from a [`String`]
    fn gstring(&self) -> GString {
        GString::from(self)
    }

    /// Create a new [`Vec`] of graphemes from a [`String`]
    fn graphemes(&self) -> Vec<Grapheme> {
        self.gstring().into_graphemes()
    }

    /// Return a [`Graphemes`] iterator from a [`String`]
    fn graphemes_iter(&self) -> Graphemes<'_> {
        UnicodeSegmentation::graphemes(self.as_str(), true)
    }
}

impl GStringTrait for &str {
    /// Create a new [`GString`] from a [`&str`]
    fn gstring(&self) -> GString {
        GString::from(self)
    }

    /// Create a new [`Vec`] of graphemes from a [`&str`]
    fn graphemes(&self) -> Vec<Grapheme> {
        graphemes(self)
    }

    /// Return a [`Graphemes`] iterator from a [`&str`]
    fn graphemes_iter(&self) -> Graphemes<'_> {
        UnicodeSegmentation::graphemes(*self, true)
    }
}

//--------------------------------------------------------------------------------------------------

/// Trait providing the `is_newline` method
pub trait IsNewline {
    /// Returns true if it is a newline grapheme
    fn is_newline(&self) -> bool;
}

impl IsNewline for str {
    /// Implemente the `is_newline` method for [`str`]
    fn is_newline(&self) -> bool {
        ["\n", "\r\n"].contains(&self)
    }
}

impl IsNewline for Grapheme {
    /// Implemente the `is_newline` method for [`Grapheme`]
    fn is_newline(&self) -> bool {
        self.data.is_newline()
    }
}

//--------------------------------------------------------------------------------------------------
// Functions

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
#[must_use]
pub fn graphemes(s: &str) -> Vec<Grapheme> {
    s.graphemes(true)
        .map(|g| Grapheme {
            data: g.to_string(),
        })
        .collect()
}

//--------------------------------------------------------------------------------------------------
// Helper functions

/// Return the indices of all newline graphemes
fn newline_indices(data: &[Grapheme]) -> Vec<usize> {
    data.iter()
        .enumerate()
        .filter(|(_, g)| g.is_newline())
        .map(|(i, _)| i)
        .collect()
}

/// Calculate the "shape" of the [`GString`] content
fn calc_shape(data: &[Grapheme]) -> Vec<usize> {
    lines(data)
        .iter()
        .map(|line| line.len().saturating_sub(1))
        .collect()
}

/// Split graphemes into lines as a [`Vec`] of [`GString`]s
fn lines(data: &[Grapheme]) -> Vec<GString> {
    let mut r = vec![];
    let mut t = vec![];
    for g in data {
        t.push(g.clone());
        if g.is_newline() {
            r.push(std::mem::take(&mut t));
        }
    }
    r.push(t);
    r.into_iter()
        .map(|data| {
            let shape = vec![data.len().saturating_sub(1)];
            GString { data, shape }
        })
        .collect()
}

/// Find the number of base 10 digits in a number
fn n_digits(number: usize) -> usize {
    format!("{number}").len()
}
