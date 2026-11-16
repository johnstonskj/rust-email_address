//!
//! Tests for the `utoipa_support` feature of the `email_address` crate.
//!
//! Added by GitHub PR #47: [Add utoipa for OpenAPI docs](https://github.com/johnstonskj/rust-email_address/pull/47)
//!
#![cfg(feature = "utoipa_support")]

use email_address::EmailAddress;
use utoipa::OpenApi;

// Feature test: GitHub PR: #47
#[test]
fn test_openapi_correct_type() {
    #[derive(utoipa::OpenApi)]
    #[openapi(
        components(schemas(EmailAddress)),
        info(title = "", description = "", license(), version = "", contact())
    )]
    struct ApiDoc;

    assert_eq!(
        ApiDoc::openapi().to_json().unwrap(),
        r#"{"openapi":"3.1.0","info":{"title":"","description":"","contact":{},"license":{"name":""},"version":""},"paths":{},"components":{"schemas":{"EmailAddress":{"type":"string","format":"email","description":""}}}}"#
    );
}
