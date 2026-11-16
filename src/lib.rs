//! A Rust crate providing an implementation of an RFC-compliant `EmailAddress` newtype.
//!
//! Primarily for validation, the `EmailAddress` type is constructed with `FromStr::from_str` which will raise any
//! parsing errors. Prior to constructions the functions `is_valid`, `is_valid_local_part`, and `is_valid_domain` may
//! also be used to test for validity without constructing an instance. This supports all of the RFC ASCII and UTF-8
//! character set rules, quoted and unquoted local parts but does not yet support all of the productions required for SMTP
//! headers; folding whitespace, comments, etc.
//!
//! ```text
//! "Simon Johnston <johnstonsk@gmail.com>"
//!                  ^------------------^ email()
//!                             ^-------^ domain()
//!                  ^--------^ local_part()
//!  ^------------^ display_part()
//! ```
//!
//! # Examples
//!
//! The following shows the basic `is_valid` and `from_str` functions.
//!
//! ```rust
//! use email_address::*;
//! use std::str::FromStr;
//!
//! assert!(EmailAddress::is_valid("user.name+tag+sorting@example.com"));
//!
//! assert_eq!(
//!     EmailAddress::from_str("Abc.example.com"),
//!     Error::MissingSeparator.into()
//! );
//! ```
//!
//! The following shows the three format functions used to output an email address.
//!
//! ```rust
//! use email_address::*;
//! use std::str::FromStr;
//!
//! let email = EmailAddress::from_str("johnstonsk@gmail.com").unwrap();
//!
//! assert_eq!(
//!     email.to_string(),
//!     "johnstonsk@gmail.com".to_string()
//! );
//!
//! assert_eq!(
//!     String::from(email.clone()),
//!     "johnstonsk@gmail.com".to_string()
//! );
//!
//! assert_eq!(
//!     email.as_ref(),
//!     "johnstonsk@gmail.com"
//! );
//!
//! assert_eq!(
//!     email.to_uri(),
//!     "mailto:johnstonsk@gmail.com".to_string()
//! );
//!
//! assert_eq!(
//!     email.to_display("Simon Johnston"),
//!     "Simon Johnston <johnstonsk@gmail.com>".to_string()
//! );
//! ```
//!
//!
//! # Specifications
//!
//! 1. RFC 1123: [_Requirements for Internet Hosts -- Application and Support_](https://tools.ietf.org/html/rfc1123),
//!    IETF,Oct 1989.
//! 1. RFC 3629: [_UTF-8, a transformation format of ISO 10646_](https://tools.ietf.org/html/rfc3629),
//!    IETF, Nov 2003.
//! 1. RFC 3696: [_Application Techniques for Checking and Transformation of
//!    Names_](https://tools.ietf.org/html/rfc3696), IETF, Feb 2004.
//! 1. RFC 4291 [_IP Version 6 Addressing Architecture_](https://tools.ietf.org/html/rfc4291),
//!    IETF, Feb 2006.
//! 1. RFC 5234: [_Augmented BNF for Syntax Specifications: ABNF_](https://tools.ietf.org/html/rfc5234),
//!    IETF, Jan 2008.
//! 1. RFC 5321: [_Simple Mail Transfer Protocol_](https://tools.ietf.org/html/rfc5321),
//!    IETF, Oct 2008.
//! 1. RFC 5322: [_Internet Message Format_](https://tools.ietf.org/html/rfc5322), I
//!    ETF, Oct 2008.
//! 1. RFC 5890: [_Internationalized Domain Names for Applications (IDNA): Definitions and Document
//!    Framework_](https://tools.ietf.org/html/rfc5890), IETF, Aug 2010.
//! 1. RFC 6531: [_SMTP Extension for Internationalized Email_](https://tools.ietf.org/html/rfc6531),
//!    IETF, Feb 2012
//! 1. RFC 6532: [_Internationalized Email Headers_](https://tools.ietf.org/html/rfc6532),
//!    IETF, Feb 2012.
//!
//! From RFC 5322: §3.2.1. [Quoted characters](https://tools.ietf.org/html/rfc5322#section-3.2.1):
//!
//! ```ebnf
//! quoted-pair     =   ("\" (VCHAR / WSP)) / obs-qp
//! ```
//!
//! From RFC 5322: §3.2.2. [Folding White Space and Comments](https://tools.ietf.org/html/rfc5322#section-3.2.2):
//!
//! ```ebnf
//! FWS             =   ([*WSP CRLF] 1*WSP) /  obs-FWS
//!                                        ; Folding white space
//!
//! ctext           =   %d33-39 /          ; Printable US-ASCII
//!                     %d42-91 /          ;  characters not including
//!                     %d93-126 /         ;  "(", ")", or "\"
//!                     obs-ctext
//!
//! ccontent        =   ctext / quoted-pair / comment
//!
//! comment         =   "(" *([FWS] ccontent) [FWS] ")"
//!
//! CFWS            =   (1*([FWS] comment) [FWS]) / FWS
//! ```
//!
//! From RFC 5322: §3.2.3. [Atom](https://tools.ietf.org/html/rfc5322#section-3.2.3):
//!
//! ```ebnf
//! atext           =   ALPHA / DIGIT /    ; Printable US-ASCII
//!                     "!" / "#" /        ;  characters not including
//!                     "$" / "%" /        ;  specials.  Used for atoms.
//!                     "&" / "'" /
//!                     "*" / "+" /
//!                     "-" / "/" /
//!                     "=" / "?" /
//!                     "^" / "_" /
//!                     "`" / "{" /
//!                     "|" / "}" /
//!                     "~"
//!
//! atom            =   [CFWS] 1*atext [CFWS]
//!
//! dot-atom-text   =   1*atext *("." 1*atext)
//!
//! dot-atom        =   [CFWS] dot-atom-text [CFWS]
//!
//! specials        =   "(" / ")" /        ; Special characters that do
//!                     "<" / ">" /        ;  not appear in atext
//!                     "[" / "]" /
//!                     ":" / ";" /
//!                     "@" / "\" /
//!                     "," / "." /
//!                     DQUOTE
//! ```
//!
//! From RFC 5322: §3.2.4. [Quoted Strings](https://tools.ietf.org/html/rfc5322#section-3.2.4):
//!
//! ```ebnf
//! qtext           =   %d33 /             ; Printable US-ASCII
//!                     %d35-91 /          ;  characters not including
//!                     %d93-126 /         ;  "\" or the quote character
//!                     obs-qtext
//!
//! qcontent        =   qtext / quoted-pair
//!
//! quoted-string   =   [CFWS]
//!                     DQUOTE *([FWS] qcontent) [FWS] DQUOTE
//!                     [CFWS]
//! ```
//!
//! From RFC 5322, §3.4.1. [Addr-Spec Specification](https://tools.ietf.org/html/rfc5322#section-3.4.1):
//!
//! ```ebnf
//! addr-spec       =   local-part "@" domain
//!
//! local-part      =   dot-atom / quoted-string / obs-local-part
//!
//! domain          =   dot-atom / domain-literal / obs-domain
//!
//! domain-literal  =   [CFWS] "[" *([FWS] dtext) [FWS] "]" [CFWS]
//!
//! dtext           =   %d33-90 /          ; Printable US-ASCII
//!                     %d94-126 /         ;  characters not including
//!                     obs-dtext          ;  "[", "]", or "\"
//! ```
//!
//! RFC 3696, §3. [Restrictions on email addresses](https://tools.ietf.org/html/rfc3696#section-3)
//! describes in detail the quoting of characters in an address.
//!
//! ## Unicode
//!
//! RFC 6531, §3.3. [Extended Mailbox Address Syntax](https://tools.ietf.org/html/rfc6531#section-3.3)
//! extends the rules above for non-ASCII character sets.
//!
//! ```ebnf
//! sub-domain   =/  U-label
//!     ; extend the definition of sub-domain in RFC 5321, Section 4.1.2
//!
//! atext   =/  UTF8-non-ascii
//!     ; extend the implicit definition of atext in
//!     ; RFC 5321, Section 4.1.2, which ultimately points to
//!     ; the actual definition in RFC 5322, Section 3.2.3
//!
//! qtextSMTP  =/ UTF8-non-ascii
//!     ; extend the definition of qtextSMTP in RFC 5321, Section 4.1.2
//!
//! esmtp-value  =/ UTF8-non-ascii
//!     ; extend the definition of esmtp-value in RFC 5321, Section 4.1.2
//! ```
//!
//! A "U-label" is an IDNA-valid string of Unicode characters, in Normalization Form C (NFC) and
//! including at least one non-ASCII character, expressed in a standard Unicode Encoding Form (such as
//! UTF-8). It is also subject to the constraints about permitted characters that are specified in
//! Section 4.2 of the Protocol document and the rules in the Sections 2 and 3 of the Tables document,
//! the Bidi constraints in that document if it contains any character from scripts that are written
//! right to left, and the symmetry constraint described immediately below. Conversions between U-labels
//! and A-labels are performed according to the "Punycode" specification RFC3492, adding or removing
//! the ACE prefix as needed.
//!
//! RFC 6532: §3.1 [UTF-8 Syntax and Normalization](https://tools.ietf.org/html/rfc6532#section-3.1),
//! and §3.2 [Syntax Extensions to RFC 5322](https://tools.ietf.org/html/rfc6532#section-3.2) extend
//! the syntax above with:
//!
//! ```ebnf
//! UTF8-non-ascii  =   UTF8-2 / UTF8-3 / UTF8-4
//!
//! ...
//!
//! VCHAR   =/  UTF8-non-ascii
//!
//! ctext   =/  UTF8-non-ascii
//!
//! atext   =/  UTF8-non-ascii
//!
//! qtext   =/  UTF8-non-ascii
//!
//! text    =/  UTF8-non-ascii
//!               ; note that this upgrades the body to UTF-8
//!
//! dtext   =/  UTF8-non-ascii
//! ```
//!
//! These in turn refer to RFC 6529 §4. [Syntax of UTF-8 Byte Sequences](https://tools.ietf.org/html/rfc3629#section-4):
//!
//! > A UTF-8 string is a sequence of octets representing a sequence of UCS
//! > characters.  An octet sequence is valid UTF-8 only if it matches the
//! > following syntax, which is derived from the rules for encoding UTF-8
//! > and is expressed in the ABNF of \[RFC2234\].
//!
//! ```ebnf
//!    UTF8-octets = *( UTF8-char )
//!    UTF8-char   = UTF8-1 / UTF8-2 / UTF8-3 / UTF8-4
//!    UTF8-1      = %x00-7F
//!    UTF8-2      = %xC2-DF UTF8-tail
//!    UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
//!                  %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
//!    UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
//!                  %xF4 %x80-8F 2( UTF8-tail )
//!    UTF8-tail   = %x80-BF
//! ```
//!
//! Comments in addresses are discussed in RFC 5322 Appendix A.5. [White Space, Comments, and Other
//! Oddities](https://tools.ietf.org/html/rfc5322#appendix-A.5).
//!
//! An informal description can be found on [Wikipedia](https://en.wikipedia.org/wiki/Email_address).
//!
//! # Features
//!
//! * **std** (*enabled by default*) — enables usage of the standard library, generally unnecessary.
//! * **serde_support** (*enabled by default*) — enables serialization and deserialization support
//!   via the [serde](https://crates.io/crates/serde) crate.
//! * **sqlx_support** — enables support for the [sqlx](https://crates.io/crates/sqlx) crate. For
//!   legacy reasons this feature is also named `sqlx`.
//! * **sqlx_postgres** — enables support for the PostgreSQL backend of the
//!   [sqlx](https://crates.io/crates/sqlx) crate.
//! * **utoipa_support** — enables support for the [utoipa](https://crates.io/crates/utoipa) crate,
//!   thereby allowing type-safe generation of OpenAPI documentation for the `EmailAddress` type.
//! * **zeroize_support** — enables support for the [zeroize](https://crates.io/crates/zeroize)
//!   crate, which allows securely zeroing sensitive data from memory, for example when an email
//!   address is dropped.
//!

#[cfg(not(feature = "std"))]
extern crate alloc;

use core::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    str::FromStr,
};

#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize, Serializer};

#[cfg(feature = "sqlx")]
use sqlx::{Database, Decode, Encode, Type, encode::IsNull, error::BoxDynError};

#[cfg(feature = "sqlx_postgres")]
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Type representing a single email address. This is basically a wrapper around a String, the
/// email address is parsed for correctness with `FromStr::from_str`, which is the only want to
/// create an instance. The various components of the email _are not_ parsed out to be accessible
/// independently.
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema), schema(format = Email, description = ""))]
pub struct EmailAddress(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

pub(crate) const LOCAL_PART_MAX_LENGTH: usize = 64;
// see: https://www.rfc-editor.org/errata_search.php?rfc=3696&eid=1690
pub(crate) const DOMAIN_MAX_LENGTH: usize = 254;
pub(crate) const SUB_DOMAIN_MAX_LENGTH: usize = 63;

const SP: char = ' ';
const HTAB: char = '\t';
const ESC: char = '\\';

pub(crate) const AT: char = '@';
pub(crate) const DOT: char = '.';
const DQUOTE: char = '"';
const LBRACKET: char = '[';
const RBRACKET: char = ']';
#[allow(dead_code)]
const LPAREN: char = '(';
#[allow(dead_code)]
const RPAREN: char = ')';

const DISPLAY_SEP: &str = " <";
const DISPLAY_START: char = '<';
const DISPLAY_END: char = '>';

const MAILTO_URI_PREFIX: &str = "mailto:";

// ------------------------------------------------------------------------------------------------

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// From RFC 5321, section 2.4:
//
// The local-part of a mailbox MUST BE treated as case sensitive. Therefore,
// SMTP implementations MUST take care to preserve the case of mailbox
// local-parts. In particular, for some hosts, the user "smith" is different
// from the user "Smith". However, exploiting the case sensitivity of mailbox
// local-parts impedes interoperability and is discouraged. Mailbox domains
// follow normal DNS rules and are hence not case sensitive.
//

// ------------------------------------------------------------------------------------------------

impl PartialEq for EmailAddress {
    fn eq(&self, other: &Self) -> bool {
        self.with_lowercase_domain() == other.with_lowercase_domain()
    }
}

impl Eq for EmailAddress {}

// ------------------------------------------------------------------------------------------------

impl Hash for EmailAddress {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (left, right) = split_at(&self.0).unwrap();
        left.hash(state);
        // Avoid allocating if right is already lowercase ASCII.
        if right
            .chars()
            .all(|char| !char.is_ascii() || char.is_ascii_lowercase())
        {
            right.hash(state)
        } else {
            right.to_ascii_lowercase().hash(state);
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Ord for EmailAddress {
    fn cmp(&self, other: &Self) -> Ordering {
        self.with_lowercase_domain()
            .cmp(&other.with_lowercase_domain())
    }
}

impl PartialOrd for EmailAddress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ------------------------------------------------------------------------------------------------

impl FromStr for EmailAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_address(s, Default::default())
    }
}

// ------------------------------------------------------------------------------------------------

impl From<EmailAddress> for String {
    #[cfg(not(feature = "zeroize"))]
    fn from(email: EmailAddress) -> Self {
        email.0
    }

    #[cfg(feature = "zeroize")]
    fn from(email: EmailAddress) -> Self {
        email.0.clone()
    }
}

// ------------------------------------------------------------------------------------------------

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// ------------------------------------------------------------------------------------------------

#[cfg(feature = "serde_support")]
impl Serialize for EmailAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde_support")]
impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected, Visitor};

        struct EmailAddressVisitor;

        impl Visitor<'_> for EmailAddressVisitor {
            type Value = EmailAddress;

            fn expecting(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
                fmt.write_str("string containing a valid email address")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                EmailAddress::from_str(s).map_err(|err| {
                    let exp = format!("{}", err);
                    #[cfg(feature = "zeroize")]
                    let s = "[secret email address]";
                    Error::invalid_value(Unexpected::Str(s), &exp.as_ref())
                })
            }
        }

        deserializer.deserialize_str(EmailAddressVisitor)
    }
}

// ------------------------------------------------------------------------------------------------

#[cfg(feature = "sqlx")]
impl<DB: Database> Type<DB> for EmailAddress
where
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB: Database> Encode<'a, DB> for EmailAddress
where
    String: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        <String as Encode<'a, DB>>::encode_by_ref(&self.0, buf)
    }

    fn produces(&self) -> Option<DB::TypeInfo> {
        <String as Encode<'a, DB>>::produces(&self.0)
    }

    fn size_hint(&self) -> usize {
        <String as Encode<'a, DB>>::size_hint(&self.0)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB: Database> Decode<'a, DB> for EmailAddress
where
    String: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        Ok(Self::from_str(&<String as Decode<'a, DB>>::decode(value)?)?)
    }
}

// ------------------------------------------------------------------------------------------------

#[cfg(feature = "sqlx_postgres")]
impl PgHasArrayType for EmailAddress
where
    String: PgHasArrayType,
{
    fn array_type_info() -> PgTypeInfo {
        <String as PgHasArrayType>::array_type_info()
    }
}

// ------------------------------------------------------------------------------------------------

impl EmailAddress {
    ///
    /// Creates an `EmailAddress` without checking if the email is valid. Only
    /// call this method if the address is known to be valid.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use email_address::EmailAddress;
    ///
    /// let unchecked = "john.doe@example.com";
    /// let email = EmailAddress::from_str(unchecked).expect("email is not valid");
    /// let valid_email = String::from(email);
    /// let email = EmailAddress::new_unchecked(valid_email);
    ///
    /// assert_eq!("John Doe <john.doe@example.com>", email.to_display("John Doe"));
    /// ```
    pub fn new_unchecked<S>(address: S) -> Self
    where
        S: Into<String>,
    {
        Self(address.into())
    }

    ///
    /// Parses an [EmailAddress] with custom [Options]. Useful for configuring validations
    /// that aren't mandatory by the specification.
    ///
    /// ```
    /// use email_address::{EmailAddress, Options};
    ///
    /// let options = Options { minimum_sub_domains: 2, ..Options::default() };
    /// let result = EmailAddress::parse_with_options("john.doe@localhost", options).is_ok();
    ///
    /// assert_eq!(result, false);
    /// ```
    pub fn parse_with_options(address: &str, options: Options) -> Result<Self, Error> {
        parse_address(address, options)
    }

    ///
    /// Determine whether the `address` string is a valid email address. Note this is equivalent to
    /// the following:
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// let is_valid = EmailAddress::from_str("johnstonskj@gmail.com").is_ok();
    /// ```
    ///
    #[cfg(not(feature = "zeroize"))]
    pub fn is_valid(address: &str) -> bool {
        Self::from_str(address).is_ok()
    }
    #[cfg(feature = "zeroize")]
    pub fn is_valid(address: &str) -> bool {
        match Self::from_str(address) {
            Ok(mut email) => {
                email.zeroize();
                true
            }
            _ => false,
        }
    }

    ///
    /// Determine whether the `part` string would be a valid `local-part` if it were in an
    /// email address.
    ///
    pub fn is_valid_local_part(part: &str) -> bool {
        parse_local_part(part, Default::default()).is_ok()
    }

    ///
    /// Determine whether the `part` string would be a valid `domain` if it were in an
    /// email address.
    ///
    pub fn is_valid_domain(part: &str) -> bool {
        parse_domain(part, Default::default()).is_ok()
    }

    ///
    /// Return this email address formatted as a URI. This will also URI-encode the email
    /// address itself. So, `name@example.org` becomes `mailto:name@example.org`.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().to_uri(),
    ///     String::from("mailto:name@example.org")
    /// );
    /// ```
    ///
    #[cfg(not(feature = "zeroize"))]
    pub fn to_uri(&self) -> String {
        let encoded = encode(&self.0);
        format!("{}{}", MAILTO_URI_PREFIX, encoded)
    }
    #[cfg(feature = "zeroize")]
    pub fn to_uri(&self) -> String {
        let mut encoded = encode(&self.0);
        let output = format!("{}{}", MAILTO_URI_PREFIX, encoded);
        encoded.zeroize();
        output
    }

    ///
    /// Return a string formatted as a display email with the user name. This is commonly used
    /// in email headers and other locations where a display name is associated with the
    /// address.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
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
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().local_part(),
    ///     String::from("name")
    /// );
    /// ```
    ///
    pub fn local_part(&self) -> &str {
        let (local, _, _) = split_parts(&self.0).unwrap();
        local
    }

    ///
    /// Returns the display part of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("Name <name@example.org>").unwrap().display_part(),
    ///     String::from("Name")
    /// );
    /// ```
    ///
    pub fn display_part(&self) -> &str {
        let (_, _, display) = split_parts(&self.0).unwrap();
        display
    }

    ///
    /// Returns the email part of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("Name <name@example.org>").unwrap().email(),
    ///     String::from("name@example.org")
    /// );
    /// ```
    ///
    pub fn email(&self) -> &str {
        let (_, email) = split_display_email(&self.0).unwrap();
        email
    }

    ///
    /// Returns the domain of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().domain(),
    ///     String::from("example.org")
    /// );
    /// ```
    ///
    pub fn domain(&self) -> &str {
        let (_, domain, _) = split_parts(&self.0).unwrap();
        domain
    }

    ///
    /// Returns the entire email address as a string reference.
    ///
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    ///
    /// Get a canonical version of this email address which can be compared with others.
    ///
    /// This returns a Cow to avoid allocating a replacement String in the hopefully common
    /// case of lowercase input domains.
    ///
    fn with_lowercase_domain<'a>(&'a self) -> Cow<'a, str> {
        let (left, right) = split_at(&self.0).unwrap();
        if right
            .chars()
            .all(|char| !char.is_ascii() || char.is_ascii_lowercase())
        {
            Cow::Borrowed(&self.0)
        } else {
            Cow::Owned(format!("{}@{}", left, right.to_ascii_lowercase()))
        }
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

fn parse_address(address: &str, options: Options) -> Result<EmailAddress, Error> {
    //
    // Deals with cases of '@' in `local-part`, if it is quoted they are legal, if
    // not then they'll return an `InvalidCharacter` error later.
    //
    let (local_part, domain, display) = split_parts(address)?;
    match (
        display.is_empty(),
        local_part.starts_with(DISPLAY_START),
        options.allow_display_text,
    ) {
        (false, _, false) => Err(Error::UnsupportedDisplayName),
        (true, true, true) => Err(Error::MissingDisplayName),
        (true, true, false) => Err(Error::InvalidCharacter),
        _ => {
            parse_local_part(local_part, options)?;
            parse_domain(domain, options)?;
            Ok(EmailAddress(address.to_owned()))
        }
    }
}

fn split_parts(address: &str) -> Result<(&str, &str, &str), Error> {
    let (display, email) = split_display_email(address)?;
    let (local_part, domain) = split_at(email)?;
    Ok((local_part, domain, display))
}

fn split_display_email(text: &str) -> Result<(&str, &str), Error> {
    match find_display_separator(text) {
        None => Ok(("", text)),
        Some(index) => {
            let left = &text[..index];
            let right = &text[index + DISPLAY_SEP.len()..];
            let right = right.trim();
            if !right.ends_with(DISPLAY_END) {
                Err(Error::MissingEndBracket)
            } else {
                let email = &right[0..right.len() - 1];
                let display_name = left.trim();

                Ok((display_name, email))
            }
        }
    }
}

fn split_at(address: &str) -> Result<(&str, &str), Error> {
    match address.rsplit_once(AT) {
        None => Error::MissingSeparator.into(),
        Some(left_right) => Ok(left_right),
    }
}

fn parse_local_part(part: &str, options: Options) -> Result<(), Error> {
    if part.is_empty() {
        Error::LocalPartEmpty.into()
    } else if part.len() > LOCAL_PART_MAX_LENGTH && !options.allow_long_local_parts {
        Error::LocalPartTooLong.into()
    } else if part.starts_with(DQUOTE) && part.ends_with(DQUOTE) {
        // <= to handle `part` = `"` (single quote).
        if part.len() <= 2 {
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
        Ok(())
    } else {
        Error::InvalidCharacter.into()
    }
}

fn parse_unquoted_local_part(part: &str) -> Result<(), Error> {
    if is_dot_atom_text(part) {
        Ok(())
    } else {
        Error::InvalidCharacter.into()
    }
}

fn parse_domain(part: &str, options: Options) -> Result<(), Error> {
    if part.is_empty() {
        Error::DomainEmpty.into()
    } else if part.len() > DOMAIN_MAX_LENGTH {
        Error::DomainTooLong.into()
    } else if part.starts_with(LBRACKET) && part.ends_with(RBRACKET) {
        if options.allow_domain_literal {
            parse_literal_domain(&part[1..part.len() - 1])
        } else {
            Error::UnsupportedDomainLiteral.into()
        }
    } else {
        parse_text_domain(part, options)
    }
}

fn parse_text_domain(part: &str, options: Options) -> Result<(), Error> {
    let mut sub_domains = 0;

    for sub_part in part.split(DOT) {
        // As per https://www.rfc-editor.org/rfc/rfc1034#section-3.5
        // and https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address,
        // at least one character must exist in a `subdomain`/`label` part of the domain
        if sub_part.is_empty() {
            return Error::SubDomainEmpty.into();
        }

        // As per https://www.rfc-editor.org/rfc/rfc1034#section-3.5,
        // the domain label needs to start with a `letter`;
        // however, https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
        // specifies a label can start
        // with a `let-dig` (letter or digit), so we allow the wider range

        if !sub_part.starts_with(char::is_alphanumeric) {
            return Error::InvalidCharacter.into();
        }
        // Both specifications mentioned above require the last character to be a
        // `let-dig` (letter or digit)
        if !sub_part.ends_with(char::is_alphanumeric) {
            return Error::InvalidCharacter.into();
        }

        if sub_part.len() > SUB_DOMAIN_MAX_LENGTH {
            return Error::SubDomainTooLong.into();
        }

        if !is_atom(sub_part) {
            return Error::InvalidCharacter.into();
        }

        sub_domains += 1;
    }

    if sub_domains < options.minimum_sub_domains {
        Error::DomainTooFew.into()
    } else {
        Ok(())
    }
}

fn parse_literal_domain(part: &str) -> Result<(), Error> {
    if part.chars().all(is_dtext_char) {
        return Ok(());
    }
    Error::InvalidCharacter.into()
}

// ------------------------------------------------------------------------------------------------

fn find_display_separator(text: &str) -> Option<usize> {
    let mut in_quote = false;
    let mut escaped = false;
    let mut separator = None;

    for (index, c) in text.char_indices() {
        if !in_quote && text[index..].starts_with(DISPLAY_SEP) {
            separator = Some(index);
        }

        if escaped {
            escaped = false;
        } else if in_quote && c == ESC {
            escaped = true;
        } else if c == DQUOTE {
            in_quote = !in_quote;
        }
    }

    separator
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
        || !c.is_ascii()
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
    c == '\x21' || ('\x23'..='\x5B').contains(&c) || ('\x5D'..='\x7E').contains(&c) || !c.is_ascii()
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
    ('\x21'..='\x5A').contains(&c) || ('\x5E'..='\x7E').contains(&c) || !c.is_ascii()
}

// ------------------------------------------------------------------------------------------------
// Sub Modules
// ------------------------------------------------------------------------------------------------

mod errors;
pub use errors::Error;

mod options;
pub use options::Options;
