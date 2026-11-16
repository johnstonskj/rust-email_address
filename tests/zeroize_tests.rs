//!
//! Tests for the `zeroize_support` feature of the `email_address` crate.
//!
//! Added by GitHub PR #46: [Support zeroization](https://github.com/johnstonskj/rust-email_address/pull/46)
//!
#![cfg(feature = "zeroize")]

use claims::{assert_err_eq, assert_ok};
use core::str::FromStr;
use email_address::*;
use zeroize::Zeroize;

#[cfg(feature = "serde_support")]
use serde::de::{Deserialize, Error as _, Unexpected};
#[cfg(feature = "serde_support")]
use serde_assert::{Deserializer, Token};

#[test]
fn test_zeroize() {
    let mut email = assert_ok!(EmailAddress::from_str("secret@example.com"));
    assert_eq!(email.as_str(), "secret@example.com");

    email.zeroize();
    assert_eq!(email.as_str(), "");
}

#[test]
fn test_has_drop() {
    assert!(std::mem::needs_drop::<EmailAddress>());
}

#[test]
#[cfg(feature = "serde_support")]
fn test_deserialize_invalid_secret_value() {
    let mut deserializer =
        Deserializer::builder([Token::Str("secret.example.com".to_owned())]).build();

    assert_err_eq!(
        EmailAddress::deserialize(&mut deserializer),
        serde_assert::de::Error::invalid_value(
            Unexpected::Str("[secret email address]"),
            &"Missing separator character '@'."
        )
    );
}
