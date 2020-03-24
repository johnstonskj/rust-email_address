# Crate email_address

A Rust crate providing an implementation of an RFC-compliant `EmailAddress` newtype. 

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/upnp-rs.svg)](https://crates.io/crates/email_address)
[![docs.rs](https://docs.rs/email_address/badge.svg)](https://docs.rs/email_address)
[![travis.ci](https://travis-ci.org/johnstonskj/rust-email_address.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-email_address)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-email_address.svg)](https://github.com/johnstonskj/rust-email_address/stargazers)

TBD

## Example

```rust
use email_address::*;

assert!(EmailAddress::is_valid("user.name+tag+sorting@example.com"));

assert_eq!(
    EmailAddress::from_str("Abc.example.com"),
    Error::MissingSeparator.into()
);
```

## Specifications

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

## Changes

**Version 0.1.0**

* Basic type implementation and structure based on RFC 5322.
* See TODO.

## TODO

1. Full Unicode support.
1. Support comments.
1. `to_uri` needs to support URI encoding.
1. Support line-feed and whitespace rules.
1. Does not parse _into_ `domain-literal` values, only does surface syntax check.
