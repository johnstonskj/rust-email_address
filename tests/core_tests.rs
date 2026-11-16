use core::str::FromStr;
use email_address::{EmailAddress, Error, Options};

fn is_valid(address: &str, test_case: Option<&str>) {
    if let Some(test_case) = test_case {
        println!(">> test case: {}", test_case);
        println!("     <{}>", address);
    } else {
        println!(">> <{}>", address);
    }
    assert!(EmailAddress::is_valid(address));
}

fn valid_with_options(address: &str, options: Options, test_case: Option<&str>) {
    if let Some(test_case) = test_case {
        println!(">> test case: {}", test_case);
        println!("     <{}>", address);
    } else {
        println!(">> <{}>", address);
    }
    assert!(EmailAddress::parse_with_options(address, options).is_ok());
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
    is_valid(
        "admin@mailserver1",
        Some(
            "local domain name with no TLD, although ICANN highly discourages dotless email addresses",
        ),
    );
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

#[test]
fn test_good_examples_01() {
    valid_with_options(
        "foo@example.com",
        Options {
            minimum_sub_domains: 2,
            ..Default::default()
        },
        Some("minimum sub domains"),
    );
}

#[test]
fn test_good_examples_02() {
    valid_with_options(
        "email@[127.0.0.256]",
        Options {
            allow_domain_literal: true,
            ..Default::default()
        },
        Some("minimum sub domains"),
    );
}

#[test]
fn test_good_examples_03() {
    valid_with_options(
        "email@[2001:db8::12345]",
        Options {
            allow_domain_literal: true,
            ..Default::default()
        },
        Some("minimum sub domains"),
    );
}

#[test]
fn test_good_examples_04() {
    valid_with_options(
        "email@[2001:db8:0:0:0:0:1]",
        Options {
            allow_domain_literal: true,
            ..Default::default()
        },
        Some("minimum sub domains"),
    );
}

#[test]
fn test_good_examples_05() {
    valid_with_options(
        "email@[::ffff:127.0.0.256]",
        Options {
            allow_domain_literal: true,
            ..Default::default()
        },
        Some("minimum sub domains"),
    );
}

#[test]
fn test_good_examples_06() {
    valid_with_options(
        "email@[2001:dg8::1]",
        Options {
            allow_domain_literal: true,
            ..Default::default()
        },
        Some("minimum sub domains"),
    );
}

#[test]
fn test_good_examples_07() {
    valid_with_options(
        "email@[2001:dG8:0:0:0:0:0:1]",
        Options {
            allow_domain_literal: true,
            ..Default::default()
        },
        Some("minimum sub domains"),
    );
}

#[test]
fn test_good_examples_08() {
    valid_with_options(
        "email@[::fTzF:127.0.0.1]",
        Options {
            allow_domain_literal: true,
            ..Default::default()
        },
        Some("minimum sub domains"),
    );
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

fn expect_with_options(address: &str, options: Options, error: Error, test_case: Option<&str>) {
    if let Some(test_case) = test_case {
        println!(">> test case: {}", test_case);
        println!("     <{}>, expecting {:?}", address, error);
    } else {
        println!(">> <{}>, expecting {:?}", address, error);
    }
    assert_eq!(
        EmailAddress::parse_with_options(address, options),
        error.into()
    );
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
    expect(
        "a\"b(c)d,e:f;g<h>i[j\\k]l@example.com",
        Error::InvalidCharacter,
        Some(
            "none of the special characters in this local-part are allowed outside quotation marks",
        ),
    );
}

#[test]
fn test_bad_examples_from_wikipedia_03() {
    expect(
        "just\"not\"right@example.com",
        Error::InvalidCharacter,
        Some("quoted strings must be dot separated or the only element making up the local-part"),
    );
}

#[test]
fn test_bad_examples_from_wikipedia_04() {
    expect(
        "this is\"not\\allowed@example.com",
        Error::InvalidCharacter,
        Some(
            "spaces, quotes, and backslashes may only exist when within quoted strings and preceded by a backslash",
        ),
    );
}

#[test]
fn test_bad_examples_from_wikipedia_05() {
    expect(
        "this\\ still\"not\\allowed@example.com",
        Error::InvalidCharacter,
        Some(
            "even if escaped (preceded by a backslash), spaces, quotes, and backslashes must still be contained by quotes",
        ),
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
    expect(
        "\"@example.com",
        Error::LocalPartEmpty,
        Some("local-part is empty"),
    );
}

#[test]
fn test_bad_example_04() {
    expect("simon@", Error::DomainEmpty, Some("domain is empty"));
}

#[test]
fn test_bad_example_05() {
    expect(
        "example@invalid-.com",
        Error::InvalidCharacter,
        Some("domain label ends with hyphen"),
    );
}

#[test]
fn test_bad_example_06() {
    expect(
        "example@-invalid.com",
        Error::InvalidCharacter,
        Some("domain label starts with hyphen"),
    );
}

#[test]
fn test_bad_example_07() {
    expect(
        "example@invalid.com-",
        Error::InvalidCharacter,
        Some("domain label starts ends hyphen"),
    );
}

#[test]
fn test_bad_example_08() {
    expect(
        "example@inv-.alid-.com",
        Error::InvalidCharacter,
        Some("subdomain label ends hyphen"),
    );
}

#[test]
fn test_bad_example_09() {
    expect(
        "example@-inv.alid-.com",
        Error::InvalidCharacter,
        Some("subdomain label starts hyphen"),
    );
}

#[test]
fn test_bad_example_10() {
    expect(
        "example@-.com",
        Error::InvalidCharacter,
        Some("domain label is hyphen"),
    );
}

#[test]
fn test_bad_example_11() {
    expect(
        "example@-",
        Error::InvalidCharacter,
        Some("domain label is hyphen"),
    );
}

#[test]
fn test_bad_example_12() {
    expect(
        "example@-abc",
        Error::InvalidCharacter,
        Some("domain label starts with hyphen"),
    );
}

#[test]
fn test_bad_example_13() {
    expect(
        "example@abc-",
        Error::InvalidCharacter,
        Some("domain label ends with hyphen"),
    );
}

#[test]
fn test_bad_example_14() {
    expect(
        "example@.com",
        Error::SubDomainEmpty,
        Some("subdomain label is empty"),
    );
}

#[test]
fn test_bad_example_15() {
    expect_with_options(
        "foo@localhost",
        Options::default().with_minimum_sub_domains(2),
        Error::DomainTooFew,
        Some("too few domains"),
    );
}

#[test]
fn test_bad_example_16() {
    expect_with_options(
        "foo@a.b.c.d.e.f.g.h.i",
        Options::default().with_minimum_sub_domains(10),
        Error::DomainTooFew,
        Some("too few domains"),
    );
}

#[test]
fn test_bad_example_17() {
    expect_with_options(
        "email@[127.0.0.256]",
        Options::default().without_domain_literal(),
        Error::UnsupportedDomainLiteral,
        Some("unsupported domain literal (1)"),
    );
}

#[test]
fn test_bad_example_18() {
    expect_with_options(
        "email@[2001:db8::12345]",
        Options::default().without_domain_literal(),
        Error::UnsupportedDomainLiteral,
        Some("unsupported domain literal (2)"),
    );
}

#[test]
fn test_bad_example_19() {
    expect_with_options(
        "email@[2001:db8:0:0:0:0:1]",
        Options::default().without_domain_literal(),
        Error::UnsupportedDomainLiteral,
        Some("unsupported domain literal (3)"),
    );
}

#[test]
fn test_bad_example_20() {
    expect_with_options(
        "email@[::ffff:127.0.0.256]",
        Options::default().without_domain_literal(),
        Error::UnsupportedDomainLiteral,
        Some("unsupported domain literal (4)"),
    );
}

// make sure Error impl Send + Sync
fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

#[test]
fn test_error_traits() {
    is_send::<Error>();
    is_sync::<Error>();
}

#[test]
fn test_parse_trimmed() {
    let email = EmailAddress::parse_with_options(
        "  Simons Email    <simon@example.com> ",
        Options::default(),
    )
    .unwrap();

    assert_eq!(email.display_part(), "Simons Email");

    let addr_spec: &str = email.email();
    assert_eq!(addr_spec, "simon@example.com");
}

#[test]
// [GitHub PR #15] Feature test
fn test_parse_display_name() {
    let email =
        EmailAddress::parse_with_options("Simons Email <simon@example.com>", Options::default())
            .unwrap();

    assert_eq!(email.display_part(), "Simons Email");

    let addr_spec: &str = email.email();
    assert_eq!(addr_spec, "simon@example.com");

    assert_eq!(email.local_part(), "simon");
    assert_eq!(email.domain(), "example.com");
}

#[test]
// [GitHub PR #15] Feature test
fn test_parse_display_empty_name() {
    expect(
        "<simon@example.com>",
        Error::MissingDisplayName,
        Some("missing display name"),
    );
}

#[test]
// [ GitHub PR #15, Reference, Issue #14] Feature test
fn test_parse_display_empty_name_2() {
    expect_with_options(
        "<simon@example.com>",
        Options::default().without_display_text(),
        Error::InvalidCharacter,
        Some("without display text '<' is invalid"),
    );
}

#[test]
// [GitHub PR #15, Issue #14] Feature test
fn test_parse_display_name_unsupported() {
    expect_with_options(
        "Simons Email <simon@example.com>",
        Options::default().without_display_text(),
        Error::UnsupportedDisplayName,
        Some("unsupported display name (1)"),
    );
}

#[test]
// [GitHub Issue #23] Regression test
fn test_missing_tld() {
    EmailAddress::parse_with_options("simon@localhost", Options::default()).unwrap();
    EmailAddress::parse_with_options(
        "simon@localhost",
        Options::default().with_no_minimum_sub_domains(),
    )
    .unwrap();

    expect_with_options(
        "simon@localhost",
        Options::default().with_required_tld(),
        Error::DomainTooFew,
        Some("too few domain segments"),
    );
}

#[test]
// [GitHub Issue #11] Regression test
fn test_eq_name_case_sensitive_local() {
    let email = EmailAddress::new_unchecked("simon@example.com");

    assert_eq!(email, EmailAddress::new_unchecked("simon@example.com"));
    assert_ne!(email, EmailAddress::new_unchecked("Simon@example.com"));
    assert_ne!(email, EmailAddress::new_unchecked("simoN@example.com"));
}

#[test]
// [GitHub Issue #11] Regression test
fn test_eq_name_case_insensitive_domain() {
    let email = EmailAddress::new_unchecked("simon@example.com");

    assert_eq!(email, EmailAddress::new_unchecked("simon@Example.com"));
    assert_eq!(email, EmailAddress::new_unchecked("simon@example.COM"));
}

// [GitHub PR #44] feat: Allow long local parts
#[test]
fn test_allow_long_local_parts() {
    let email = "reply+2a907e&3uofr1&&99cd5c22c2ca5b23655799316a8d8eb2dd83c3c487612cb9b9a00bf13f13afe2@mg1.substack.com";

    assert_eq!(
        EmailAddress::parse_with_options(email, Options::default()),
        Err(Error::LocalPartTooLong),
    );

    let parsed =
        EmailAddress::parse_with_options(email, Options::default().with_long_local_parts())
            .unwrap();
    assert_eq!(
        parsed.local_part(),
        "reply+2a907e&3uofr1&&99cd5c22c2ca5b23655799316a8d8eb2dd83c3c487612cb9b9a00bf13f13afe2"
    )
}

// [GitHub PR #53] Fix: handle display-like text in quoted local parts
#[test]
fn test_quoted_local_part_with_display_like_text() {
    let email = EmailAddress::from_str("\"User <user@example.com>\"@example.com").unwrap();

    assert_eq!(email.display_part(), "");
    assert_eq!(email.local_part(), "\"User <user@example.com>\"");
    assert_eq!(email.domain(), "example.com");
}
