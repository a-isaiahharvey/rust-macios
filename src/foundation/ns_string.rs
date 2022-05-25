use std::{
    ffi::{CStr, CString},
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use libc::c_char;
use objc::{
    class, msg_send,
    runtime::{Object, BOOL, NO},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{traits::t_NSString, ComparisonResult, Int, Locale, UInt},
    id,
    objective_c_runtime::traits::t_NSObject,
    utils::to_bool,
};

use super::{string::Encoding, unichar, CompareOptions, StringTransform, UTF8_ENCODING};

/// This is a mapping to the Objective-C NSString class.
#[repr(C)]
pub struct String {
    /// The raw pointer to the Objective-C object.
    pub objc: Id<Object>,
    marker: PhantomData<()>,
}

impl String {
    /// Returns the UTF8 bytes for this `NSString`.
    pub fn bytes(&self) -> *const c_char {
        unsafe {
            let bytes: *const c_char = msg_send![&*self.objc, UTF8String];
            bytes
        }
    }

    /// Convert this `NSString` into a `&str`.
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let bytes = self.bytes();

        unsafe {
            let bytes = CStr::from_ptr(bytes);
            bytes.to_str()
        }
    }
}

impl t_NSObject for String {
    fn init() -> Self {
        unsafe { msg_send![class!(NSString), new] }
    }

    fn toId(mut self) -> id {
        &mut *self.objc
    }

    unsafe fn fromId(obj: id) -> Self {
        String {
            objc: Id::from_ptr(obj),
            marker: PhantomData,
        }
    }

    fn description(&self) -> String {
        unsafe {
            let description: id = msg_send![&*self.objc, description];
            String::fromId(description)
        }
    }

    fn debugDescription(&self) -> String {
        unsafe {
            let description: id = msg_send![&*self.objc, debugDescription];
            String::fromId(description)
        }
    }

    fn retain(&self) -> Self {
        unsafe { msg_send![&*self.objc, retain] }
    }
}

impl t_NSString for String {
    fn string() -> Self {
        unsafe { msg_send![class!(NSString), string] }
    }

    fn init() -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, init])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }

    fn initWithBytesLenEncoding(bytes: *const c_char, len: UInt, encoding: Encoding) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:bytes
                length:len
                encoding:encoding
            ])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }

    fn initWithNoCpyStr(s: &str) -> Self {
        String {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithBytesNoCopy:s.as_ptr()
                    length:s.len()
                    encoding:UTF8_ENCODING
                    freeWhenDone:NO
                ])
            },

            marker: PhantomData,
        }
    }

    fn initWithCharactersLen(characters: *const unichar, len: UInt) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithCharacters:characters
                length:len
            ])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }

    fn initWithStr(s: &str) -> Self {
        String {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithString:s.as_ptr()])
            },

            marker: PhantomData,
        }
    }

    fn length(&self) -> Int {
        unsafe { msg_send![&*self.objc, length] }
    }

    fn lengthOfBytesUsingEncoding(&self, enc: Encoding) -> Int {
        unsafe { msg_send![&*self.objc, lengthOfBytesUsingEncoding: enc] }
    }

    fn maximumLengthOfBytesUsingEncoding(&self, enc: Encoding) -> Int {
        unsafe { msg_send![&*self.objc, maximumLengthOfBytesUsingEncoding: enc] }
    }

    fn characterAtIndex(&self, index: Int) -> char {
        unsafe { msg_send![&*self.objc, characterAtIndex: index] }
    }

    fn caseInsensitiveCompare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<String>,
    {
        unsafe { msg_send![&*self.objc, caseInsensitiveCompare: string.into()] }
    }

    fn localizedCaseInsensitiveCompare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedCaseInsensitiveCompare: string.into()] }
    }

    fn compare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into()] }
    }

    fn localizedCompare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedCompare: string.into()] }
    }

    fn localized_standard_compare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedStandardCompare: string.into()] }
    }

    fn compareWithOptions<S>(&self, string: S, options: CompareOptions) -> ComparisonResult
    where
        S: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into() options: options] }
    }

    fn compareWithOptionsRange<S>(
        &self,
        string: S,
        options: CompareOptions,
        range: Range<UInt>,
    ) -> ComparisonResult
    where
        S: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into() options: options range: range] }
    }

    fn compareWithOptionsRangeLocale<S>(
        &self,
        string: S,
        options: CompareOptions,
        range: Range<UInt>,
        locale: Locale,
    ) -> ComparisonResult
    where
        S: Into<String>,
    {
        unsafe {
            msg_send![&*self.objc, compare: string.into() options: options range: range locale: locale]
        }
    }

    fn hasPrefix<S>(&self, prefix: S) -> bool
    where
        S: Into<String>,
    {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, hasPrefix: prefix.into()];
            to_bool(result)
        }
    }

    fn hasSuffix<S>(&self, suffix: S) -> bool
    where
        S: Into<String>,
    {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, hasSuffix: suffix.into()];
            to_bool(result)
        }
    }

    fn isEqualTo<S>(&self, string: S) -> bool
    where
        S: Into<String>,
    {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, isEqualToString: string.into()];
            to_bool(result)
        }
    }

    fn appending<s>(&self, string: s) -> String
    where
        s: Into<String>,
    {
        unsafe { msg_send![&*self.objc, stringByAppendingString: string.into()] }
    }

    fn padding<S>(&self, new_length: Int, pad_string: S, starting_at: Int) -> String
    where
        S: Into<String>,
    {
        unsafe {
            msg_send![&*self.objc, stringByPaddingToLength: new_length withString: pad_string.into() startingAtIndex: starting_at]
        }
    }

    fn contains<S>(&self, other: S) -> bool
    where
        S: Into<String>,
    {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, containsString: other.into()];
            to_bool(result)
        }
    }

    fn lowercased(&self) -> String {
        unsafe { msg_send![&*self.objc, lowercaseString] }
    }

    fn localizedLowercase(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedLowercaseString] }
    }

    fn uppercased(&self) -> String {
        unsafe { msg_send![&*self.objc, uppercaseString] }
    }

    fn localizedUppercase(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedUppercaseString] }
    }

    fn capitalized(&self) -> String {
        unsafe { msg_send![&*self.objc, capitalizedString] }
    }

    fn localizedCapitalized(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedCapitalizedString] }
    }

    fn stringByApplyingTransform(
        &mut self,
        transform: StringTransform,
        reverse: bool,
    ) -> Option<String> {
        let result: id = unsafe {
            msg_send![
            &*self.objc,
            stringByApplyingTransform: transform
            reverse: reverse
            ]
        };
        if result.is_null() {
            None
        } else {
            Some(unsafe { String::fromId(result) })
        }
    }
}

impl Default for String {
    fn default() -> Self {
        Self::string()
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description().as_str().unwrap())
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debugDescription().as_str().unwrap())
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        unsafe { msg_send![&*self.objc, retain] }
    }
}

impl Deref for String {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.objc
    }
}

impl DerefMut for String {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.objc
    }
}

impl From<std::string::String> for String {
    /// Creates a new `NSString` from a `String`.
    fn from(s: std::string::String) -> Self {
        String {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithString:s.as_str()])
            },

            marker: PhantomData,
        }
    }
}

impl From<&str> for String {
    /// Creates a new `NSString` from a `&str`.
    fn from(s: &str) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: CString::new(s).unwrap().into_raw() as *mut Object
                    length:s.len()
                    encoding:UTF8_ENCODING
                ],
            )
        };

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl From<char> for String {
    /// Creates a new `NSString` from a `char`.
    fn from(c: char) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: c.encode_utf8(&mut [0; 4]).as_ptr()
                    length:1
                    encoding:UTF8_ENCODING
                ],
            )
        };

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl From<String> for &str {
    /// Converts a `NSString` to a `&str`.
    fn from(string: String) -> Self {
        unsafe {
            let ptr: *const c_char = msg_send![string.objc, UTF8String];
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl From<(&str, Encoding)> for String {
    /// Creates a new `NSString` from a `&str` and an encoding.
    fn from((s, encoding): (&str, Encoding)) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr()
                length:s.len()
                encoding:encoding
            ])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl PartialEq for String {
    /// Checks if two `NSString`s are equal.
    fn eq(&self, other: &Self) -> bool {
        self.localizedCompare(other.clone()) == ComparisonResult::OrderedSame
    }
}

impl PartialEq<&str> for String {
    /// Checks if a `NSString` is equal to a `&str`.
    fn eq(&self, other: &&str) -> bool {
        self.as_str().unwrap() == *other
    }
}

#[cfg(test)]
mod tests {
    use crate::foundation::{
        string::Encoding, string_transform::LatinToKatakana, ComparisonResult,
    };

    use super::*;

    #[test]
    fn test_from_str() {
        let s = String::from("Hello, World!");
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_from_no_cpy_str() {
        let s = String::initWithNoCpyStr("Hello, World!");
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_bytes() {
        let s = String::from("Hello, World!");
        let other = s.bytes();
        assert_eq!(s.bytes(), other);
    }

    #[test]
    fn test_bytes_len() {
        let s = String::from("Hello, World!");
        assert_eq!(s.lengthOfBytesUsingEncoding(Encoding::UTF8), 13);
    }

    #[test]
    fn test_as_str() {
        let s = String::from("Hello, World!");
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_to_string() {
        let s = String::from("Hello, World!");
        assert_eq!(s.to_string(), "Hello, World!".to_string());
    }

    #[test]
    fn test_length() {
        let s = String::from("Hello, World!");
        assert_eq!(s.length(), 13);
    }

    #[test]
    fn test_contains() {
        let s = String::from("Hello, World!");
        assert!(s.contains("Hello"));
        assert!(!s.contains("Goodbye"));
    }

    #[test]
    fn test_character_at() {
        let s = String::from("Hello, World!");
        assert_eq!(s.characterAtIndex(0), 'H');
        assert_eq!(s.characterAtIndex(1), 'e');
        assert_eq!(s.characterAtIndex(2), 'l');
        assert_eq!(s.characterAtIndex(3), 'l');
        assert_eq!(s.characterAtIndex(4), 'o');
        assert_eq!(s.characterAtIndex(5), ',');
        assert_eq!(s.characterAtIndex(6), ' ');
        assert_eq!(s.characterAtIndex(7), 'W');
        assert_eq!(s.characterAtIndex(8), 'o');
        assert_eq!(s.characterAtIndex(9), 'r');
        assert_eq!(s.characterAtIndex(10), 'l');
        assert_eq!(s.characterAtIndex(11), 'd');
        assert_eq!(s.characterAtIndex(12), '!');
    }

    #[test]
    fn test_has_prefix() {
        let s = String::from("Hello, World!");
        assert!(s.hasPrefix("Hello"));
        assert!(!s.hasPrefix("Goodbye"));
    }

    #[test]
    fn test_has_suffix() {
        let s = String::from("Hello, World!");
        assert!(s.hasSuffix("World!"));
        assert!(!s.hasSuffix("Goodbye"));
    }

    #[test]
    fn test_is_equal_to() {
        let s = String::from("Hello, World!");
        assert!(s.isEqualTo("Hello, World!"));
        assert!(!s.isEqualTo("Goodbye, World!"));
    }

    #[test]
    fn test_length_of_bytes() {
        let s = String::from("Hello, World!");
        assert_eq!(s.lengthOfBytesUsingEncoding(Encoding::UTF8), 13);
    }

    #[test]
    fn test_maximum_length_of_bytes() {
        let s = String::from("Hello, World!");
        assert_eq!(s.maximumLengthOfBytesUsingEncoding(Encoding::UTF8), 39);
    }

    #[test]
    fn test_case_insensitive_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.caseInsensitiveCompare("hello, world!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.caseInsensitiveCompare("goodbye, world!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_case_insensitive_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.localizedCaseInsensitiveCompare("hello, world!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.localizedCaseInsensitiveCompare("goodbye, world!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(s.compare("Hello, World!"), ComparisonResult::OrderedSame);
        assert_eq!(
            s.compare("Goodbye, World!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_standard_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.localized_standard_compare("Hello, World!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.localized_standard_compare("Goodbye, World!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_applying_transform() {
        let mut s = String::from("Katakana!");
        let transform = unsafe { LatinToKatakana };
        assert_eq!(
            s.stringByApplyingTransform(transform, false).unwrap(),
            "カタカナ!"
        );
    }
}
