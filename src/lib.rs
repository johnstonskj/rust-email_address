#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(clippy::single_match, rustdoc::bare_urls, unused_qualifications)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use core::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Error type used when parsing an address.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// An invalid character was found in some component of the address.
    InvalidCharacter,
    /// The separator character between `local-part` and `domain` (character: '@') was missing.
    MissingSeparator,
    /// The `local-part` is an empty string.
    LocalPartEmpty,
    /// The `local-part` is is too long.
    LocalPartTooLong,
    /// The `domain` is an empty string.
    DomainEmpty,
    /// The `domain` is is too long.
    DomainTooLong,
    /// A `sub-domain` within the `domain` is is too long.
    SubDomainTooLong,
    /// Too few `sub-domain`s in `domain`.
    DomainTooFew,
    /// Invalid placement of the domain separator (character: '.').
    DomainInvalidSeparator,
    /// The quotes (character: '"') around `local-part` are unbalanced.
    UnbalancedQuotes,
    /// A Comment within the either the `local-part`, or `domain`, was malformed.
    InvalidComment,
    /// An IP address in a `domain-literal` was malformed.
    InvalidIPAddress,
}

///
/// Type representing a single email address. This is basically a wrapper around a String, the
/// email address is parsed for correctness with `FromStr::from_str`, which is the only want to
/// create an instance. The various components of the email _are not_ parsed out to be accessible
/// independently.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EmailAddress(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const LOCAL_PART_MAX_LENGTH: usize = 64;
const DOMAIN_MAX_LENGTH: usize = 254; // see: https://www.rfc-editor.org/errata_search.php?rfc=3696&eid=1690
const SUB_DOMAIN_MAX_LENGTH: usize = 63;

#[allow(dead_code)]
const CR: char = '\r';
#[allow(dead_code)]
const LF: char = '\n';
const SP: char = ' ';
const HTAB: char = '\t';
const ESC: char = '\\';

const AT: char = '@';
const DOT: char = '.';
const DQUOTE: char = '"';
const LBRACKET: char = '[';
const RBRACKET: char = ']';
#[allow(dead_code)]
const LPAREN: char = '(';
#[allow(dead_code)]
const RPAREN: char = ')';

const UTF8_START: char = '\u{0080}';

const MAILTO_URI_PREFIX: &str = "mailto:";

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidCharacter => write!(f, "Invalid character."),
            Error::LocalPartEmpty => write!(f, "Local part is empty."),
            Error::LocalPartTooLong => write!(
                f,
                "Local part is too long. Length limit: {}",
                LOCAL_PART_MAX_LENGTH
            ),
            Error::DomainEmpty => write!(f, "Domain is empty."),
            Error::DomainTooLong => {
                write!(f, "Domain is too long. Length limit: {}", DOMAIN_MAX_LENGTH)
            }
            Error::SubDomainTooLong => write!(
                f,
                "A sub-domain is too long. Length limit: {}",
                SUB_DOMAIN_MAX_LENGTH
            ),
            Error::MissingSeparator => write!(f, "Missing separator character '{}'.", AT),
            Error::DomainTooFew => write!(f, "Too few parts in the domain"),
            Error::DomainInvalidSeparator => {
                write!(f, "Invalid placement of the domain separator '{:?}", DOT)
            }
            Error::InvalidIPAddress => write!(f, "Invalid IP Address specified for domain."),
            Error::UnbalancedQuotes => write!(f, "Quotes around the local-part are unbalanced."),
            Error::InvalidComment => write!(f, "A comment was badly formed."),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl<T> From<Error> for std::result::Result<T, Error> {
    fn from(err: Error) -> Self {
        Err(err)
    }
}

// ------------------------------------------------------------------------------------------------

impl core::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::str::FromStr for EmailAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_address(s)
    }
}

#[cfg(feature = "alloc")]
impl From<EmailAddress> for String {
    fn from(email: EmailAddress) -> Self {
        email.0
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected, Visitor};

        struct EmailAddressVisitor;

        impl Visitor<'_> for EmailAddressVisitor {
            type Value = EmailAddress;

            fn expecting(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                fmt.write_str("data")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                EmailAddress::from_str(s).map_err(|err| {
                    let exp = format!("{}", err);
                    Error::invalid_value(Unexpected::Str(s), &exp.as_ref())
                })
            }
        }

        deserializer.deserialize_str(EmailAddressVisitor)
    }
}

impl EmailAddress {
    ///
    /// Creates an `EmailAddress` without checking if the email is valid. Only
    /// call this method if the address is known to be valid.
    ///
    /// ```
    /// use core::str::FromStr;
    /// use type_email_address::EmailAddress;
    ///
    /// let unchecked = "john.doe@example.com";
    /// let email = EmailAddress::from_str(unchecked).expect("email is not valid");
    /// let valid_email = String::from(email);
    /// let email = EmailAddress::new_unchecked(valid_email);
    ///
    /// assert_eq!("John Doe <john.doe@example.com>", email.to_display("John Doe"));
    /// ```
    #[cfg(feature = "alloc")]
    pub fn new_unchecked(address: String) -> Self {
        Self(address)
    }

    ///
    /// Determine whether the `address` string is a valid email address. Note this is equivalent to
    /// the following:
    ///
    /// ```rust
    /// use type_email_address::*;
    /// use core::str::FromStr;
    ///
    /// let is_valid = EmailAddress::from_str("johnstonskj@gmail.com").is_ok();
    /// ```
    ///
    pub fn is_valid(address: &str) -> bool {
        Self::from_str(address).is_ok()
    }

    ///
    /// Determine whether the `part` string would be a valid `local-part` if it were in an
    /// email address.
    ///
    pub fn is_valid_local_part(part: &str) -> bool {
        parse_local_part(part).is_ok()
    }

    ///
    /// Determine whether the `part` string would be a valid `domain` if it were in an
    /// email address.
    ///
    pub fn is_valid_domain(part: &str) -> bool {
        parse_domain(part).is_ok()
    }

    ///
    /// Return this email address formatted as a URI. This will also URI-encode the email
    /// address itself. So, `name@example.org` becomes `mailto:name@example.org`.
    ///
    /// ```rust
    /// use type_email_address::*;
    /// use core::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().to_uri(),
    ///     String::from("mailto:name@example.org")
    /// );
    /// ```
    ///
    pub fn to_uri(&self) -> String {
        let encoded = encode(&self.0);
        format!("{}{}", MAILTO_URI_PREFIX, encoded)
    }

    ///
    /// Return a string formatted as a display email with the user name. This is commonly used
    /// in email headers and other locations where a display name is associated with the
    /// address.
    ///
    /// ```rust
    /// use type_email_address::*;
    /// use core::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().to_display("My Name"),
    ///     String::from("My Name <name@example.org>")
    /// );
    /// ```
    ///
    pub fn to_display(&self, display_name: &str) -> String {
        format!("{} <{}>", display_name, self)
    }

    ///
    /// Returns the local part of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use type_email_address::*;
    /// use core::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().local_part(),
    ///     String::from("name")
    /// );
    /// ```
    ///
    pub fn local_part(&self) -> &str {
        let (left, _) = split_at(&self.0).unwrap();
        left
    }

    ///
    /// Returns the domain of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use type_email_address::*;
    /// use core::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().domain(),
    ///     String::from("example.org")
    /// );
    /// ```
    ///
    pub fn domain(&self) -> &str {
        let (_, right) = split_at(&self.0).unwrap();
        right
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn encode(address: &str) -> String {
    let mut result = String::new();
    for c in address.chars() {
        if is_uri_reserved(c) {
            result.push_str(&format!("%{:02X}", c as u8))
        } else {
            result.push(c);
        }
    }
    result
}

fn is_uri_reserved(c: char) -> bool {
    // No need to encode '@' as this is allowed in the email scheme.
    c == '!'
        || c == '#'
        || c == '$'
        || c == '%'
        || c == '&'
        || c == '\''
        || c == '('
        || c == ')'
        || c == '*'
        || c == '+'
        || c == ','
        || c == '/'
        || c == ':'
        || c == ';'
        || c == '='
        || c == '?'
        || c == '['
        || c == ']'
}

fn parse_address(address: &str) -> Result<EmailAddress, Error> {
    //
    // Deals with cases of '@' in `local-part`, if it is quoted they are legal, if
    // not then they'll return an `InvalidCharacter` error later.
    //
    let (left, right) = split_at(address)?;
    parse_local_part(left)?;
    parse_domain(right)?;
    Ok(EmailAddress(address.to_owned()))
}

fn split_at(address: &str) -> Result<(&str, &str), Error> {
    match address.rsplit_once(AT) {
        None => Error::MissingSeparator.into(),
        Some(left_right) => Ok(left_right),
    }
}

fn parse_local_part(part: &str) -> Result<(), Error> {
    if part.is_empty() {
        Error::LocalPartEmpty.into()
    } else if part.len() > LOCAL_PART_MAX_LENGTH {
        Error::LocalPartTooLong.into()
    } else if part.starts_with(DQUOTE) && part.ends_with(DQUOTE) {
        if part.len() == 2 {
            Error::LocalPartEmpty.into()
        } else {
            parse_quoted_local_part(&part[1..part.len() - 1])
        }
    } else {
        parse_unquoted_local_part(part)
    }
}

fn parse_quoted_local_part(part: &str) -> Result<(), Error> {
    if is_qcontent(part) {
        return Ok(());
    } else {
    }
    Error::InvalidCharacter.into()
}

fn parse_unquoted_local_part(part: &str) -> Result<(), Error> {
    if is_dot_atom_text(part) {
        return Ok(());
    }
    Error::InvalidCharacter.into()
}

fn parse_domain(part: &str) -> Result<(), Error> {
    if part.is_empty() {
        Error::DomainEmpty.into()
    } else if part.len() > DOMAIN_MAX_LENGTH {
        Error::DomainTooLong.into()
    } else if part.starts_with(LBRACKET) && part.ends_with(RBRACKET) {
        parse_literal_domain(&part[1..part.len() - 1])
    } else {
        parse_text_domain(part)
    }
}

fn parse_text_domain(part: &str) -> Result<(), Error> {
    if is_dot_atom_text(part) {
        for sub_part in part.split(DOT) {
            if sub_part.len() > SUB_DOMAIN_MAX_LENGTH {
                return Error::SubDomainTooLong.into();
            }
        }
        return Ok(());
    }
    Error::InvalidCharacter.into()
}

fn parse_literal_domain(part: &str) -> Result<(), Error> {
    if part.chars().all(is_dtext_char) {
        return Ok(());
    }
    Error::InvalidCharacter.into()
}

// ------------------------------------------------------------------------------------------------

fn is_atext(c: char) -> bool {
    c.is_alphanumeric()
        || c == '!'
        || c == '#'
        || c == '$'
        || c == '%'
        || c == '&'
        || c == '\''
        || c == '*'
        || c == '+'
        || c == '-'
        || c == '/'
        || c == '='
        || c == '?'
        || c == '^'
        || c == '_'
        || c == '`'
        || c == '{'
        || c == '|'
        || c == '}'
        || c == '~'
        || is_uchar(c)
}

#[allow(dead_code)]
fn is_special(c: char) -> bool {
    c == '('
        || c == ')'
        || c == '<'
        || c == '>'
        || c == '['
        || c == ']'
        || c == ':'
        || c == ';'
        || c == '@'
        || c == '\\'
        || c == ','
        || c == '.'
        || c == DQUOTE
}

fn is_uchar(c: char) -> bool {
    c >= UTF8_START
}

fn is_atom(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_atext)
}

fn is_dot_atom_text(s: &str) -> bool {
    s.split(DOT).all(is_atom)
}

fn is_vchar(c: char) -> bool {
    ('\x21'..='\x7E').contains(&c)
}

fn is_wsp(c: char) -> bool {
    c == SP || c == HTAB
}

fn is_qtext_char(c: char) -> bool {
    c == '\x21' || ('\x23'..='\x5B').contains(&c) || ('\x5D'..='\x7E').contains(&c) || is_uchar(c)
}

fn is_qcontent(s: &str) -> bool {
    let mut char_iter = s.chars();
    while let Some(c) = &char_iter.next() {
        if c == &ESC {
            // quoted-pair
            match char_iter.next() {
                Some(c2) if is_vchar(c2) => (),
                _ => return false,
            }
        } else if !(is_wsp(*c) || is_qtext_char(*c)) {
            // qtext
            return false;
        }
    }
    true
}

fn is_dtext_char(c: char) -> bool {
    ('\x21'..='\x5A').contains(&c) || ('\x5E'..='\x7E').contains(&c)
}

#[allow(dead_code)]
fn is_ctext_char(c: char) -> bool {
    (c >= '\x21' && c == '\x27') || ('\x2A'..='\x5B').contains(&c) || ('\x5D'..='\x7E').contains(&c)
}

#[allow(dead_code)]
fn is_ctext(s: &str) -> bool {
    s.chars().all(is_ctext_char)
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use core::str::FromStr;

    fn is_valid(address: &str, test_case: Option<&str>) {
        if let Some(test_case) = test_case {
            println!(">> test case: {}", test_case);
            println!("     <{}>", address);
        } else {
            println!(">> <{}>", address);
        }
        assert!(EmailAddress::is_valid(address));
    }

    #[test]
    fn test_good_examples_from_wikipedia_01() {
        is_valid("simple@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_02() {
        is_valid("very.common@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_03() {
        is_valid("disposable.style.email.with+symbol@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_04() {
        is_valid("other.email-with-hyphen@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_05() {
        is_valid("fully-qualified-domain@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_06() {
        is_valid(
            "user.name+tag+sorting@example.com",
            Some(" may go to user.name@example.com inbox depending on mail server"),
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_07() {
        is_valid("x@example.com", Some("one-letter local-part"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_08() {
        is_valid("example-indeed@strange-example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_09() {
        is_valid("admin@mailserver1", Some("local domain name with no TLD, although ICANN highly discourages dotless email addresses"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_10() {
        is_valid(
            "example@s.example",
            Some("see the List of Internet top-level domains"),
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_11() {
        is_valid("\" \"@example.org", Some("space between the quotes"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_12() {
        is_valid("\"john..doe\"@example.org", Some("quoted double dot"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_13() {
        is_valid(
            "mailhost!username@example.org",
            Some("bangified host route used for uucp mailers"),
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_14() {
        is_valid(
            "user%example.com@example.org",
            Some("% escaped mail route to user@example.com via example.org"),
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_15() {
        is_valid("jsmith@[192.168.2.1]", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_16() {
        is_valid("jsmith@[IPv6:2001:db8::1]", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_17() {
        is_valid("user+mailbox/department=shipping@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_18() {
        is_valid("!#$%&'*+-/=?^_`.{|}~@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_19() {
        // '@' is allowed in a quoted local part. Sorry.
        is_valid("\"Abc@def\"@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_20() {
        is_valid("\"Joe.\\\\Blow\"@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_21() {
        is_valid("用户@例子.广告", Some("Chinese"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_22() {
        is_valid("अजय@डाटा.भारत", Some("Hindi"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_23() {
        is_valid("квіточка@пошта.укр", Some("Ukranian"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_24() {
        is_valid("θσερ@εχαμπλε.ψομ", Some("Greek"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_25() {
        is_valid("Dörte@Sörensen.example.com", Some("German"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_26() {
        is_valid("коля@пример.рф", Some("Russian"));
    }

    // ------------------------------------------------------------------------------------------------

    #[test]
    fn test_to_strings() {
        let email = EmailAddress::from_str("коля@пример.рф").unwrap();

        assert_eq!(String::from(email.clone()), String::from("коля@пример.рф"));

        assert_eq!(email.to_string(), String::from("коля@пример.рф"));

        assert_eq!(email.as_ref(), "коля@пример.рф");
    }

    #[test]
    fn test_to_display() {
        let email = EmailAddress::from_str("коля@пример.рф").unwrap();

        assert_eq!(
            email.to_display("коля"),
            String::from("коля <коля@пример.рф>")
        );
    }

    #[test]
    fn test_touri() {
        let email = EmailAddress::from_str("коля@пример.рф").unwrap();

        assert_eq!(email.to_uri(), String::from("mailto:коля@пример.рф"));
    }

    // ------------------------------------------------------------------------------------------------

    fn expect(address: &str, error: Error, test_case: Option<&str>) {
        if let Some(test_case) = test_case {
            println!(">> test case: {}", test_case);
            println!("     <{}>, expecting {:?}", address, error);
        } else {
            println!(">> <{}>, expecting {:?}", address, error);
        }
        assert_eq!(EmailAddress::from_str(address), error.into());
    }

    #[test]
    fn test_bad_examples_from_wikipedia_00() {
        expect(
            "Abc.example.com",
            Error::MissingSeparator,
            Some("no @ character"),
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_01() {
        expect(
            "A@b@c@example.com",
            Error::InvalidCharacter,
            Some("only one @ is allowed outside quotation marks"),
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_02() {
        expect("a\"b(c)d,e:f;g<h>i[j\\k]l@example.com",
            Error::InvalidCharacter,
        Some("none of the special characters in this local-part are allowed outside quotation marks")
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_03() {
        expect(
            "just\"not\"right@example.com",
            Error::InvalidCharacter,
            Some(
                "quoted strings must be dot separated or the only element making up the local-part",
            ),
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_04() {
        expect("this is\"not\\allowed@example.com",
            Error::InvalidCharacter,
        Some("spaces, quotes, and backslashes may only exist when within quoted strings and preceded by a backslash")
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_05() {
        // ()
        expect("this\\ still\"not\\allowed@example.com",
            Error::InvalidCharacter,
        Some("even if escaped (preceded by a backslash), spaces, quotes, and backslashes must still be contained by quotes")
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_06() {
        expect(
            "1234567890123456789012345678901234567890123456789012345678901234+x@example.com",
            Error::LocalPartTooLong,
            Some("local part is longer than 64 characters"),
        );
    }

    #[test]
    fn test_bad_example_01() {
        expect(
            "foo@example.v1234567890123456789012345678901234567890123456789012345678901234v.com",
            Error::SubDomainTooLong,
            Some("domain part is longer than 64 characters"),
        );
    }

    #[test]
    fn test_bad_example_02() {
        expect(
            "@example.com",
            Error::LocalPartEmpty,
            Some("local-part is empty"),
        );
    }

    #[test]
    fn test_bad_example_03() {
        expect(
            "\"\"@example.com",
            Error::LocalPartEmpty,
            Some("local-part is empty"),
        );
    }

    #[test]
    fn test_bad_example_04() {
        expect("simon@", Error::DomainEmpty, Some("domain is empty"));
    }

    // make sure Error impl Send + Sync
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    #[test]
    fn test_error_traits() {
        is_send::<Error>();
        is_sync::<Error>();
    }
}
