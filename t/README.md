String with support for Unicode graphemes

```rust
use gstring::*;

const S: &str = "a\u{310}e\u{301}o\u{308}\u{332}";

// Create a GString
let mut s = GString::from(S);
assert_eq!(s, S);
assert_eq!(s.graphemes(), &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"]);
assert_eq!(s.len(), 3);
assert!(!s.is_empty());
assert_eq!(s.chars(), &['a', '\u{310}', 'e', '\u{301}', 'o', '\u{308}', '\u{332}']);
assert_eq!(s.bytes(), &[0x61, 0xcc, 0x90, 0x65, 0xcc, 0x81, 0x6f, 0xcc, 0x88, 0xcc, 0xb2]);

// Insert a &str
s.insert(0, "i\u{301}u\u{301}");
assert_eq!(s, "i\u{301}u\u{301}a\u{310}e\u{301}o\u{308}\u{332}");

// Remove a grapheme at an index
assert_eq!(s.remove(1), "u\u{301}");
assert_eq!(s, "i\u{301}a\u{310}e\u{301}o\u{308}\u{332}");

// Push a &str
s.push("i\u{301}u\u{301}");
assert_eq!(s, "i\u{301}a\u{310}e\u{301}o\u{308}\u{332}i\u{301}u\u{301}");

// Pop last grapheme
assert_eq!(s.pop(), Some("u\u{301}".gstring()));
assert_eq!(s, "i\u{301}a\u{310}e\u{301}o\u{308}\u{332}i\u{301}");

// Slice
assert_eq!(s.slice(1..4), "a\u{310}e\u{301}o\u{308}\u{332}");

// Splice
s.splice(1..4, "");
assert_eq!(s, "i\u{301}i\u{301}");

// Drain
assert_eq!(s.drain(..), "i\u{301}i\u{301}");
assert_eq!(s, "");
```

