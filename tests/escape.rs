use pretty_assertions::assert_eq;
use quick_xml::escape::{self, EscapeError};
use std::borrow::Cow;

#[test]
fn escape() {
    let unchanged = escape::escape("test");
    // assert_eq does not check that Cow is borrowed, but we explicitly use Cow
    // because it influences diff
    // TODO: use assert_matches! when stabilized and other features will bump MSRV
    assert_eq!(unchanged, Cow::Borrowed("test"));
    assert!(matches!(unchanged, Cow::Borrowed(_)));

    assert_eq!(escape::escape("<&\"'>"), "&lt;&amp;&quot;&apos;&gt;");
    assert_eq!(escape::escape("<test>"), "&lt;test&gt;");
    assert_eq!(escape::escape("\"a\"bc"), "&quot;a&quot;bc");
    assert_eq!(escape::escape("\"a\"b&c"), "&quot;a&quot;b&amp;c");
    assert_eq!(
        escape::escape("prefix_\"a\"b&<>c"),
        "prefix_&quot;a&quot;b&amp;&lt;&gt;c"
    );
}

#[test]
fn partial_escape() {
    let unchanged = escape::partial_escape("test");
    // assert_eq does not check that Cow is borrowed, but we explicitly use Cow
    // because it influences diff
    // TODO: use assert_matches! when stabilized and other features will bump MSRV
    assert_eq!(unchanged, Cow::Borrowed("test"));
    assert!(matches!(unchanged, Cow::Borrowed(_)));

    assert_eq!(escape::partial_escape("<&\"'>"), "&lt;&amp;\"'&gt;");
    assert_eq!(escape::partial_escape("<test>"), "&lt;test&gt;");
    assert_eq!(escape::partial_escape("\"a\"bc"), "\"a\"bc");
    assert_eq!(escape::partial_escape("\"a\"b&c"), "\"a\"b&amp;c");
    assert_eq!(
        escape::partial_escape("prefix_\"a\"b&<>c"),
        "prefix_\"a\"b&amp;&lt;&gt;c"
    );
}

#[test]
fn minimal_escape() {
    assert_eq!(escape::minimal_escape("test"), Cow::Borrowed("test"));
    assert_eq!(escape::minimal_escape("<&\"'>"), "&lt;&amp;\"'>");
    assert_eq!(escape::minimal_escape("<test>"), "&lt;test>");
    assert_eq!(escape::minimal_escape("\"a\"bc"), "\"a\"bc");
    assert_eq!(escape::minimal_escape("\"a\"b&c"), "\"a\"b&amp;c");
    assert_eq!(
        escape::minimal_escape("prefix_\"a\"b&<>c"),
        "prefix_\"a\"b&amp;&lt;>c"
    );
}

#[test]
fn unescape() {
    let unchanged = escape::unescape("test");
    // assert_eq does not check that Cow is borrowed, but we explicitly use Cow
    // because it influences diff
    // TODO: use assert_matches! when stabilized and other features will bump MSRV
    assert_eq!(unchanged, Ok(Cow::Borrowed("test")));
    assert!(matches!(unchanged, Ok(Cow::Borrowed(_))));

    assert_eq!(
        escape::unescape("&lt;&amp;test&apos;&quot;&gt;"),
        Ok("<&test'\">".into())
    );
    assert_eq!(escape::unescape("&#x30;"), Ok("0".into()));
    assert_eq!(escape::unescape("&#48;"), Ok("0".into()));
    assert_eq!(
        escape::unescape("&foo;"),
        Err(EscapeError::UnrecognizedEntity(1..4, "foo".into()))
    );
}

/// XML allows any number of leading zeroes. That is not explicitly mentioned
/// in the specification, but enforced by the conformance test suite
/// (https://www.w3.org/XML/Test/)
/// 100 digits should be enough to ensure that any artificial restrictions
/// (such as maximal string of u128 representation) does not applied
#[test]
fn unescape_long() {
    assert_eq!(
        escape::unescape("&#0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000048;"),
        Ok("0".into()),
    );
    assert_eq!(
        escape::unescape("&#x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030;"),
        Ok("0".into()),
    );
}

#[test]
fn unescape_with() {
    let custom_entities = |ent: &str| match ent {
        "foo" => Some("BAR"),
        _ => None,
    };

    let unchanged = escape::unescape_with("test", custom_entities);
    // assert_eq does not check that Cow is borrowed, but we explicitly use Cow
    // because it influences diff
    // TODO: use assert_matches! when stabilized and other features will bump MSRV
    assert_eq!(unchanged, Ok(Cow::Borrowed("test")));
    assert!(matches!(unchanged, Ok(Cow::Borrowed(_))));

    assert_eq!(
        escape::unescape_with("&lt;", custom_entities),
        Err(EscapeError::UnrecognizedEntity(1..3, "lt".into())),
    );
    assert_eq!(
        escape::unescape_with("&#x30;", custom_entities),
        Ok("0".into())
    );
    assert_eq!(
        escape::unescape_with("&#48;", custom_entities),
        Ok("0".into())
    );
    assert_eq!(
        escape::unescape_with("&foo;", custom_entities),
        Ok("BAR".into())
    );
    assert_eq!(
        escape::unescape_with("&fop;", custom_entities),
        Err(EscapeError::UnrecognizedEntity(1..4, "fop".into()))
    );
}

/// XML allows any number of leading zeroes. That is not explicitly mentioned
/// in the specification, but enforced by the conformance test suite
/// (https://www.w3.org/XML/Test/)
/// 100 digits should be enough to ensure that any artificial restrictions
/// (such as maximal string of u128 representation) does not applied
#[test]
fn unescape_with_long() {
    assert_eq!(
        escape::unescape_with("&#0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000048;", |_| None),
        Ok("0".into()),
    );
    assert_eq!(
        escape::unescape_with("&#x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030;", |_| None),
        Ok("0".into()),
    );
}
