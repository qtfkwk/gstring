# Changelog

* 0.1.0 (2024-12-03): Initial release
    * 0.1.1 (2024-12-03): Fix description
    * 0.1.2 (2024-12-03): Fix documentation
    * 0.1.3 (2024-12-03): Fix readme
    * 0.1.4 (2024-12-03): Fix documentation
    * 0.1.5 (2024-12-03): Miscellaneous fixes
    * 0.1.6 (2024-12-03): Improve documentation/tests
    * 0.1.7 (2024-12-04): Add makefile
* 0.2.0 (2024-12-05): Add the `GStringTrait` and implement its methods for `String` and `&str`
    * 0.2.1 (2024-12-11): Fix documentation; update dependencies
    * 0.2.2 (2024-12-12): Improve documentation, tests, and trait implementations
    * 0.2.3 (2024-12-12): Improve test for pop method
    * 0.2.4 (2024-12-12): Improve tests
* 0.3.0 (2024-12-14): Add `graphemes_iter` method to `GStringTrait` and `String`/`&str` implementations
    * 0.3.1 (2024-12-14): Fix documentation
* 0.4.0 (2025-01-30): Derive `Default` trait for `GString` and add `GString::new` associated function; implement `PartialEq<str>` for GString; update dependencies
* 0.5.0 (2025-02-12): Improve `GString::insert` and `GString::push` methods; improve documentation; update dependencies
    * 0.5.1 (2025-02-21): Update dependencies
* 0.6.0 (2025-02-22): Add `GString::get` method
* 0.7.0 (2025-02-22): Add `GString::find{,_str}` methods
* 0.8.0 (2025-02-23): Add `GString::{find{,_prev}_from{,_str}` methods; fix changelog
* 0.9.0 (2025-03-05): Add `GString::{lines,coordinates,newlines}` methods and `IsNewline` trait and implementation for `str`; update dependencies
    * 0.9.1 (2025-04-16): Update dependencies
* 0.10.0 (2025-05-12): Add `GString::position` method; track the "shape" in `GString` and add `GString::shape` method; fix `GString.get()` to return a reference instead of a copy; fix `GStringRefIter` to return references instead of copies; fix doc link to `GString::index`; fix `GString::splice` examples; fix changelog; update dependencies
    * 0.10.1 (2025-05-12): Add doc links between `GString::{coordinates,position}` methods
* 0.11.0 (2025-05-13): Add `GString::shape_string` method; add `serve-doc` target to makefile
* 0.12.0 (2025-09-02): Update dependencies; 2024 edition

