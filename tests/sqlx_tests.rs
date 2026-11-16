//!
//! Tests for the `sqlx_support` feature of the `email_address` crate.
//!
//! Added by GitHub PR #33: [Add support for sqlx types](https://github.com/johnstonskj/rust-email_address/pull/33)
//!
#![cfg(feature = "sqlx")]

use claims::{assert_err, assert_matches, assert_ok, assert_ok_eq};
use core::str::FromStr;
use email_address::{EmailAddress, Error};
use sqlx::{
    Any, Decode, Encode, Value,
    any::{AnyArguments, AnyValue},
};
use sqlx_core::any::AnyValueKind;

#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;
#[cfg(feature = "std")]
use std::sync::Arc;

#[test]
fn test_encode() {
    let email = assert_ok!(EmailAddress::from_str("simple@example.com"));
    let mut buf = AnyArguments::default().values;
    let _ = assert_ok!(<EmailAddress as Encode<'_, Any>>::encode(email, &mut buf));

    assert_eq!(buf.0.len(), 1);
    assert_matches!(&buf.0[0], AnyValueKind::Text(text) if text.as_ref() == "simple@example.com");
}

#[test]
#[cfg(feature = "std")]
fn test_decode() {
    let value = AnyValue {
        kind: AnyValueKind::Text(Arc::new("simple@example.com".to_string())),
    };
    let email = assert_ok!(EmailAddress::from_str("simple@example.com"));

    assert_ok_eq!(
        <EmailAddress as Decode<'_, Any>>::decode(value.as_ref()),
        email
    );
}

#[test]
#[cfg(feature = "std")]
fn test_decode_invalid_value() {
    let value = AnyValue {
        kind: AnyValueKind::Text(Arc::new("Abc.example.com".to_string())),
    };
    let boxed_error = assert_err!(<EmailAddress as Decode<'_, Any>>::decode(value.as_ref()));
    let error = assert_ok!(boxed_error.downcast::<Error>());

    assert_matches!(*error, Error::MissingSeparator);
}

#[test]
fn test_decode_invalid_type() {
    let value = AnyValue {
        kind: AnyValueKind::Integer(42),
    };
    let boxed_error = assert_err!(<EmailAddress as Decode<'_, Any>>::decode(value.as_ref()));

    assert_eq!(boxed_error.to_string(), "expected TEXT, got Integer(42)");
}
