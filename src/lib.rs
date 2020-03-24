/*!
A Rust crate providing an implementation of an RFC-compliant `EmailAddress` newtype.

# Example

The following shoes the basic `is_valid` and `from_str` functions.

```rust
use email_address::*;
use std::str::FromStr;
assert!(EmailAddress::is_valid("user.name+tag+sorting@example.com"));

assert_eq!(
    EmailAddress::from_str("Abc.example.com"),
    Error::MissingSeparator.into()
);
```

The following shows the three format functions used to output an email address.

```rust
use email_address::*;
use std::str::FromStr;

let email = EmailAddress::from_str("johnstonsk@gmail.com").unwrap();

assert_eq!(
    email.to_string(),
    "johnstonsk@gmail.com".to_string()
);

assert_eq!(
    email.to_uri(),
    "mailto:johnstonsk@gmail.com".to_string()
);

assert_eq!(
    email.to_display("Simon Johnston"),
    "Simon Johnston <johnstonsk@gmail.com>".to_string()
);
```


# Specifications

1. RFC 3629: [_UTF-8, a transformation format of ISO 10646_](https://tools.ietf.org/html/rfc3629),
   IETF, Nov 2003.
1. RFC 3696: [_Application Techniques for Checking and Transformation of
   Names_](https://tools.ietf.org/html/rfc3696), IETF, Feb 2004.
1. RFC 5321: [_Simple Mail Transfer Protocol_](https://tools.ietf.org/html/rfc5321),
   IETF, Oct 2008.
1. RFC 5322: [_Internet Message Format_](https://tools.ietf.org/html/rfc5322), I
   ETF, Oct 2008.
1. RFC 5890: [_Internationalized Domain Names for Applications (IDNA): Definitions and Document
   Framework_](https://tools.ietf.org/html/rfc5890), IETF, Aug 2010.
1. RFC 6531: [_SMTP Extension for Internationalized Email_](https://tools.ietf.org/html/rfc6531),
   IETF, Feb 2012
1. RFC 5234: [_Augmented BNF for Syntax Specifications: ABNF_](https://tools.ietf.org/html/rfc5234),
   IETF, Jan 2008.

From RFC 5322: §3.2.1. [Quoted characters](https://tools.ietf.org/html/rfc5322#section-3.2.1):

```ebnf
quoted-pair     =   ("\" (VCHAR / WSP)) / obs-qp
```

From RFC 5322: §3.2.2. [Folding White Space and Comments](https://tools.ietf.org/html/rfc5322#section-3.2.2):

```ebnf
FWS             =   ([*WSP CRLF] 1*WSP) /  obs-FWS
                                       ; Folding white space

ctext           =   %d33-39 /          ; Printable US-ASCII
                    %d42-91 /          ;  characters not including
                    %d93-126 /         ;  "(", ")", or "\"
                    obs-ctext

ccontent        =   ctext / quoted-pair / comment

comment         =   "(" *([FWS] ccontent) [FWS] ")"

CFWS            =   (1*([FWS] comment) [FWS]) / FWS
```

From RFC 5322: §3.2.3. [Atom](https://tools.ietf.org/html/rfc5322#section-3.2.3):

```ebnf
atext           =   ALPHA / DIGIT /    ; Printable US-ASCII
                    "!" / "#" /        ;  characters not including
                    "$" / "%" /        ;  specials.  Used for atoms.
                    "&" / "'" /
                    "*" / "+" /
                    "-" / "/" /
                    "=" / "?" /
                    "^" / "_" /
                    "`" / "{" /
                    "|" / "}" /
                    "~"

atom            =   [CFWS] 1*atext [CFWS]

dot-atom-text   =   1*atext *("." 1*atext)

dot-atom        =   [CFWS] dot-atom-text [CFWS]

specials        =   "(" / ")" /        ; Special characters that do
                    "<" / ">" /        ;  not appear in atext
                    "[" / "]" /
                    ":" / ";" /
                    "@" / "\" /
                    "," / "." /
                    DQUOTE
```

From RFC 5322: §3.2.4. [Quoted Strings](https://tools.ietf.org/html/rfc5322#section-3.2.4):

```ebnf
qtext           =   %d33 /             ; Printable US-ASCII
                    %d35-91 /          ;  characters not including
                    %d93-126 /         ;  "\" or the quote character
                    obs-qtext

qcontent        =   qtext / quoted-pair

quoted-string   =   [CFWS]
                    DQUOTE *([FWS] qcontent) [FWS] DQUOTE
                    [CFWS]
```

From RFC 5322, §3.4.1. [Addr-Spec Specification](https://tools.ietf.org/html/rfc5322#section-3.4.1):

```ebnf
addr-spec       =   local-part "@" domain

local-part      =   dot-atom / quoted-string / obs-local-part

domain          =   dot-atom / domain-literal / obs-domain

domain-literal  =   [CFWS] "[" *([FWS] dtext) [FWS] "]" [CFWS]

dtext           =   %d33-90 /          ; Printable US-ASCII
                    %d94-126 /         ;  characters not including
                    obs-dtext          ;  "[", "]", or "\"
```

RFC 3696, §3. [Restrictions on email addresses](https://tools.ietf.org/html/rfc3696#section-3)
describes in detail the quoting of characters in an address.

RFC 6531, §3.3. [Extended Mailbox Address Syntax](https://tools.ietf.org/html/rfc6531#section-3.3)
extends the rules above for non-ASCII character sets.


```ebnf
UTF8-non-ascii  =   UTF8-2 / UTF8-3 / UTF8-4
```

This refers to RFC 6529 §4. [Syntax of UTF-8 Byte Sequences](https://tools.ietf.org/html/rfc3629#section-4):

> A UTF-8 string is a sequence of octets representing a sequence of UCS
> characters.  An octet sequence is valid UTF-8 only if it matches the
> following syntax, which is derived from the rules for encoding UTF-8
> and is expressed in the ABNF of [RFC2234].

```ebnf
   UTF8-octets = *( UTF8-char )
   UTF8-char   = UTF8-1 / UTF8-2 / UTF8-3 / UTF8-4
   UTF8-1      = %x00-7F
   UTF8-2      = %xC2-DF UTF8-tail
   UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
                 %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
   UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
                 %xF4 %x80-8F 2( UTF8-tail )
   UTF8-tail   = %x80-BF
```


Comments in addresses are discussed in RFC 5322 Appendix A.5. [White Space, Comments, and Other
Oddities](https://tools.ietf.org/html/rfc5322#appendix-A.5).

An informal description can be found on [Wikipedia](https://en.wikipedia.org/wiki/Email_address).

*/

#![warn(
    missing_debug_implementations,
    missing_docs,
    unused_extern_crates,
    rust_2018_idioms
)]

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

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
    /// More than one separator character (character: '@') was found.
    TooManySeparators,
    /// The `local-part` is an empty string.
    LocalPartEmpty,
    /// The `local-part` is is too long.
    LocalPartTooLong,
    /// The `domain` is an empty string.
    DomainEmpty,
    /// The `domain` is is too long.
    DomainTooLong,
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
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct EmailAddress(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const LOCAL_PART_MAX_LENGTH: usize = 64;
const DOMAIN_MAX_LENGTH: usize = 255;

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
const LT: char = '<';
const GT: char = '>';

const MAILTO_URI_PREFIX: &str = "mailto:";

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            Error::MissingSeparator => write!(f, "Missing separator character '{}'.", AT),
            Error::TooManySeparators => {
                write!(f, "Found more than one separator character '{}'.", AT)
            }
            Error::DomainTooFew => write!(f, "Too few parts in the domain"),
            Error::DomainInvalidSeparator => {
                write!(f, "Invalid placement of the domain separator '{:?}", DOT)
            }
            Error::InvalidIPAddress => write!(f, "Invalid IP Address specified for doamin."),
            Error::UnbalancedQuotes => write!(f, "Quotes around the local-part are unbalanced."),
            Error::InvalidComment => write!(f, "A comment was badly formed."),
        }
    }
}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}

impl std::error::Error for Error {}

impl<T> Into<std::result::Result<T, Error>> for Error {
    fn into(self) -> Result<T, Error> {
        Err(self)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EmailAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_address(s)
    }
}

impl EmailAddress {
    ///
    /// Determine whether the passed string is a valid email address. Note this is equivalent to
    /// the following:
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// let is_valid = EmailAddress::from_str("johnstonskj@gmail.com").is_ok();
    /// ```
    ///
    pub fn is_valid(address: &str) -> bool {
        Self::from_str(&address).is_ok()
    }

    ///
    /// Return this email address formatted as a URI.
    ///
    pub fn to_uri(&self) -> String {
        format!("{}{}", MAILTO_URI_PREFIX, self)
    }

    ///
    /// Return a string formatted as a display email with the user name.
    ///
    pub fn to_display(&self, display_name: &str) -> String {
        format!("{} <{}>", display_name, self)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_address(address: &str) -> Result<EmailAddress, Error> {
    let address = if address.starts_with(LT) && address.ends_with(GT) {
        &address[1..address.len() - 1]
    } else {
        address
    };
    let parts = address.split(AT).collect::<Vec<&str>>();
    match parts.len() {
        l if l < 2 => Error::MissingSeparator.into(),
        l if l > 2 => Error::TooManySeparators.into(),
        _ => {
            parse_local_part(parts.first().unwrap())?;
            parse_domain(parts.last().unwrap())?;
            Ok(EmailAddress(address.to_string()))
        }
    }
}

fn parse_local_part(part: &str) -> Result<(), Error> {
    if part.is_empty() {
        Error::LocalPartEmpty.into()
    } else if part.len() > LOCAL_PART_MAX_LENGTH {
        Error::LocalPartTooLong.into()
    } else if part.starts_with(DQUOTE) && part.ends_with(DQUOTE) {
        parse_quoted_local_part(&part[1..part.len() - 1])
    } else {
        parse_unquoted_local_part(part)
    }
}

fn parse_quoted_local_part(part: &str) -> Result<(), Error> {
    if part.is_ascii() && is_qcontent(part) {
        return Ok(());
    }
    Error::InvalidCharacter.into()
}

fn parse_unquoted_local_part(part: &str) -> Result<(), Error> {
    if part.is_ascii() && is_dot_atom_text(part) {
        return Ok(());
    }
    Error::InvalidCharacter.into()
}

fn parse_domain(part: &str) -> Result<(), Error> {
    if part.is_empty() {
        Error::DomainEmpty.into()
    } else if part.len() > LOCAL_PART_MAX_LENGTH {
        Error::DomainTooLong.into()
    } else if part.starts_with(LBRACKET) && part.ends_with(RBRACKET) {
        parse_literal_domain(&part[1..part.len() - 1])
    } else {
        parse_text_domain(part)
    }
}

fn parse_text_domain(part: &str) -> Result<(), Error> {
    if part.is_ascii() && is_dot_atom_text(part) {
        return Ok(());
    }
    Error::InvalidCharacter.into()
}

fn parse_literal_domain(part: &str) -> Result<(), Error> {
    if part.is_ascii() && part.chars().all(is_dtext_char) {
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
}

fn is_atom(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_atext)
}

fn is_dot_atom_text(s: &str) -> bool {
    s.split(DOT).all(is_atom)
}

fn is_vchar(c: char) -> bool {
    c >= '\x21' && c <= '\x7E'
}

fn is_wsp(c: char) -> bool {
    c == SP || c == HTAB
}

#[allow(dead_code)]
fn is_ctext_char(c: char) -> bool {
    (c >= '\x21' && c == '\x27') || (c >= '\x2A' && c <= '\x5B') || (c >= '\x5D' && c <= '\x7E')
}

fn is_qtext_char(c: char) -> bool {
    c == '\x21' || (c >= '\x23' && c <= '\x5B') || (c >= '\x5D' && c <= '\x7E')
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
    (c >= '\x21' && c <= '\x5A') || (c >= '\x5E' && c <= '\x7E')
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

    #[test]
    fn test_good_examples_from_wikipedia_01() {
        assert!(EmailAddress::is_valid("simple@example.com"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_02() {
        assert!(EmailAddress::is_valid("very.common@example.com"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_03() {
        assert!(EmailAddress::is_valid(
            "disposable.style.email.with+symbol@example.com"
        ));
    }

    #[test]
    fn test_good_examples_from_wikipedia_04() {
        assert!(EmailAddress::is_valid(
            "other.email-with-hyphen@example.com"
        ));
    }

    #[test]
    fn test_good_examples_from_wikipedia_05() {
        assert!(EmailAddress::is_valid("fully-qualified-domain@example.com"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_06() {
        // may go to user.name@example.com inbox depending on mail server
        assert!(EmailAddress::is_valid("user.name+tag+sorting@example.com"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_07() {
        // one-letter local-part
        assert!(EmailAddress::is_valid("x@example.com"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_08() {
        assert!(EmailAddress::is_valid("example-indeed@strange-example.com"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_09() {
        // local domain name with no TLD, although ICANN highly discourages dotless email addresses
        assert!(EmailAddress::is_valid("admin@mailserver1"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_10() {
        // see the List of Internet top-level domains
        assert!(EmailAddress::is_valid("example@s.example"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_11() {
        // space between the quotes
        assert!(EmailAddress::is_valid("\" \"@example.org"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_12() {
        // quoted double dot
        assert!(EmailAddress::is_valid("\"john..doe\"@example.org"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_13() {
        // bangified host route used for uucp mailers
        assert!(EmailAddress::is_valid("mailhost!username@example.org"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_14() {
        // % escaped mail route to user@example.com via example.org
        assert!(EmailAddress::is_valid("user%example.com@example.org"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_15() {
        assert!(EmailAddress::is_valid("jsmith@[192.168.2.1]"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_16() {
        assert!(EmailAddress::is_valid("jsmith@[IPv6:2001:db8::1]"));
    }

    #[test]
    fn test_bad_examples_from_wikipedia_00() {
        //  (no @ character)
        assert_eq!(
            EmailAddress::from_str("Abc.example.com"),
            Error::MissingSeparator.into()
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_01() {
        //  (only one @ is allowed outside quotation marks)
        assert_eq!(
            EmailAddress::from_str("A@b@c@example.com"),
            Error::TooManySeparators.into()
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_02() {
        //  (none of the special characters in this local-part are allowed outside quotation marks)
        assert_eq!(
            EmailAddress::from_str("a\"b(c)d,e:f;g<h>i[j\\k]l@example.com"),
            Error::InvalidCharacter.into()
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_03() {
        // (quoted strings must be dot separated or the only element making up the local-part)
        assert_eq!(
            EmailAddress::from_str("just\"not\"right@example.com"),
            Error::InvalidCharacter.into()
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_04() {
        // (spaces, quotes, and backslashes may only exist when within quoted strings and preceded by a backslash)
        assert_eq!(
            EmailAddress::from_str("this is\"not\\allowed@example.com"),
            Error::InvalidCharacter.into()
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_05() {
        // (even if escaped (preceded by a backslash), spaces, quotes, and backslashes must still be contained by quotes)
        assert_eq!(
            EmailAddress::from_str("this\\ still\"not\\allowed@example.com"),
            Error::InvalidCharacter.into()
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_06() {
        // (local part is longer than 64 characters)
        assert_eq!(
            EmailAddress::from_str(
                "1234567890123456789012345678901234567890123456789012345678901234+x@example.com"
            ),
            Error::LocalPartTooLong.into()
        );
    }
}
