// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Struct of options that can be configured when parsing with `parse_with_options`.
///
#[derive(Debug, Copy, Clone)]
pub struct Options {
    ///
    /// Sets the minimum number of domain segments that must exist to parse successfully.
    ///
    /// ```rust
    /// use email_address::*;
    ///
    /// assert!(
    ///     EmailAddress::parse_with_options(
    ///         "simon@localhost",
    ///         Options::default().with_no_minimum_sub_domains(),
    ///     ).is_ok()
    /// );
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         "simon@localhost",
    ///         Options::default().with_required_tld()
    ///     ),
    ///     Err(Error::DomainTooFew)
    /// );
    /// ```
    ///
    pub minimum_sub_domains: usize,

    ///
    /// Specifies if domain literals are allowed. Defaults to `true`.
    ///
    /// ```rust
    /// use email_address::*;
    ///
    /// assert!(
    ///     EmailAddress::parse_with_options(
    ///         "email@[127.0.0.256]",
    ///         Options::default().with_domain_literal()
    ///     ).is_ok()
    /// );
    ///
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         "email@[127.0.0.256]",
    ///         Options::default().without_domain_literal()
    ///     ),
    ///     Err(Error::UnsupportedDomainLiteral),
    /// );
    /// ```
    ///
    pub allow_domain_literal: bool,

    ///
    /// Specified whether display text is allowed. Defaults to `true`. If you want strict
    /// email-only checking setting this to `false` will remove support for the prefix string
    /// and therefore the '<' and '>' brackets around the email part.
    ///
    /// ```rust
    /// use email_address::*;
    ///
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         "Simon <simon@example.com>",
    ///         Options::default().without_display_text()
    ///     ),
    ///     Err(Error::UnsupportedDisplayName),
    /// );
    ///
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         "<simon@example.com>",
    ///         Options::default().without_display_text()
    ///     ),
    ///     Err(Error::InvalidCharacter),
    /// );
    /// ```
    ///
    pub allow_display_text: bool,

    /// Enable this to allow local parts which exceed the RFC's value to be accepted.
    ///
    /// ```rust
    /// use email_address::*;
    /// let email = "reply+2a907e&3uofr1&&99cd5c22c2ca5b23655799316a8d8eb2dd83c3c487612cb9b9a00bf13f13afe2@mg1.substack.com";
    ///
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         email,
    ///         Options::default()
    ///     ),
    ///     Err(Error::LocalPartTooLong),
    /// );
    ///
    /// assert!(
    ///     EmailAddress::parse_with_options(
    ///         email,
    ///         Options::default().with_long_local_parts()
    ///     ).is_ok()
    /// );
    ///
    /// ```
    ///
    pub allow_long_local_parts: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Options {
    fn default() -> Self {
        Self {
            minimum_sub_domains: Default::default(),
            allow_domain_literal: true,
            allow_display_text: true,
            allow_long_local_parts: false,
        }
    }
}

impl Options {
    /// Set the value of `minimum_sub_domains`.
    #[inline(always)]
    pub const fn with_minimum_sub_domains(self, min: usize) -> Self {
        Self {
            minimum_sub_domains: min,
            ..self
        }
    }
    #[inline(always)]
    /// Set the value of `minimum_sub_domains` to zero.
    pub const fn with_no_minimum_sub_domains(self) -> Self {
        Self {
            minimum_sub_domains: 0,
            ..self
        }
    }
    #[inline(always)]
    /// Set the value of `minimum_sub_domains` to two, this has the effect of requiring a
    /// domain name with a top-level domain (TLD).
    pub const fn with_required_tld(self) -> Self {
        Self {
            minimum_sub_domains: 2,
            ..self
        }
    }
    /// Set the value of `allow_domain_literal` to `true`.
    #[inline(always)]
    pub const fn with_domain_literal(self) -> Self {
        Self {
            allow_domain_literal: true,
            ..self
        }
    }
    /// Set the value of `allow_domain_literal` to `false`.
    #[inline(always)]
    pub const fn without_domain_literal(self) -> Self {
        Self {
            allow_domain_literal: false,
            ..self
        }
    }
    /// Set the value of `allow_display_text` to `true`.
    #[inline(always)]
    pub const fn with_display_text(self) -> Self {
        Self {
            allow_display_text: true,
            ..self
        }
    }
    /// Set the value of `allow_display_text` to `false`.
    #[inline(always)]
    pub const fn without_display_text(self) -> Self {
        Self {
            allow_display_text: false,
            ..self
        }
    }

    /// Set the value of `allow_large_local_parts` to `true`.
    #[inline(always)]
    pub const fn with_long_local_parts(self) -> Self {
        Self {
            allow_long_local_parts: true,
            ..self
        }
    }
}
