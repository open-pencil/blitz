//! A stylesheet `@import` whose URL resolves against no base, such as a relative one in a
//! document without a URL, is refused rather than panicking, and the rest of the stylesheet
//! still applies.

use blitz_dom::DocumentConfig;
use blitz_html::HtmlDocument;
use style::properties::generated::longhands::visibility::computed_value::T as Visibility;

const HTML: &str = r#"<!doctype html>
<html>
  <head><style>@import url("missing.css"); #styled { visibility: hidden }</style></head>
  <body><p id="styled">text</p></body>
</html>"#;

#[test]
fn relative_import_without_base_url_is_refused() {
    let mut doc = HtmlDocument::from_html(HTML, DocumentConfig::default());
    doc.resolve(0.0);
    assert!(!doc.has_pending_critical_resources());
    let node = doc
        .get_node(doc.get_element_by_id("styled").unwrap())
        .unwrap();
    assert_eq!(
        node.primary_styles()
            .unwrap()
            .get_inherited_box()
            .slow_clone_visibility(),
        Visibility::Hidden
    );
}
