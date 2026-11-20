use email_address::EmailAddress;

fn child_text(node: roxmltree::Node<'_, '_>, name: &str) -> String {
    node.children()
        .find(|child| child.has_tag_name(name))
        .and_then(|child| child.text())
        .unwrap_or_default()
        .to_owned()
}

fn decode_control_pictures(address: &str) -> String {
    address
        .chars()
        .map(|character| match character {
            '\u{2400}'..='\u{241f}' => {
                char::from_u32(character as u32 - 0x2400).expect("control picture has a mapping")
            }
            '\u{2420}' => ' ',
            '\u{2421}' => '\u{7f}',
            other => other,
        })
        .collect()
}

#[test]
// Feature test: isemail syntax vectors (version 3.05).
fn test_isemail_syntax_vectors() {
    let document = roxmltree::Document::parse(include_str!("vectors/isemail/test/tests.xml"))
        .expect("isemail test vectors are valid XML");
    let total_cases = document
        .descendants()
        .filter(|node| node.has_tag_name("test"))
        .count();
    let mut covered = 0;
    let mut skipped = 0;

    for test_case in document
        .descendants()
        .filter(|node| node.has_tag_name("test"))
    {
        let case_id = test_case.attribute("id").expect("test case has an ID");
        let category = child_text(test_case, "category");
        let address = decode_control_pictures(&child_text(test_case, "address"));

        let expected = match category.as_str() {
            "ISEMAIL_ERR" => false,
            "ISEMAIL_VALID_CATEGORY" | "ISEMAIL_DNSWARN" => true,
            "ISEMAIL_RFC5321" | "ISEMAIL_CFWS" | "ISEMAIL_DEPREC" | "ISEMAIL_RFC5322" => {
                skipped += 1;
                continue; // README.md documents incomplete SMTP-header syntax support.
            }
            _ => panic!("unexpected isemail category in case {case_id}: {category}"),
        };

        assert_eq!(
            EmailAddress::is_valid(&address),
            expected,
            "isemail test case {case_id} ({category})"
        );
        covered += 1;
    }

    assert_eq!(covered + skipped, total_cases);
}
