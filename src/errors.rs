use crate::{AT, DOMAIN_MAX_LENGTH, DOT, LOCAL_PART_MAX_LENGTH, SUB_DOMAIN_MAX_LENGTH};

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
    /// The `sub-domain` within the `domain` is empty.
    SubDomainEmpty,
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
    /// A `domain-literal` was supplied, but is unsupported by parser configuration.
    UnsupportedDomainLiteral,
    /// Display name was supplied, but is unsupported by parser configuration.
    UnsupportedDisplayName,
    /// Display name was not supplied, but email starts with '<'.
    MissingDisplayName,
    /// An email enclosed within <...> is missing the final '>'.
    MissingEndBracket,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
            Error::SubDomainEmpty => write!(f, "A sub-domain is empty."),
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
            Error::UnsupportedDomainLiteral => write!(f, "Domain literals are not supported."),
            Error::UnsupportedDisplayName => write!(f, "Display names are not supported."),
            Error::MissingDisplayName => write!(
                f,
                "Display name was not supplied, but email starts with '<'."
            ),
            Error::MissingEndBracket => write!(f, "Terminating '>' is missing."),
        }
    }
}

impl core::error::Error for Error {}

impl<T> From<Error> for core::result::Result<T, Error> {
    fn from(err: Error) -> Self {
        Err(err)
    }
}
