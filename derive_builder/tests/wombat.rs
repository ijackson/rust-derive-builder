#[macro_use]
extern crate derive_builder;

#[derive(Builder)]
#[builder(derive(serde::Serialize))]
struct Lorem {
    #[builder_field_attrs(
        #[serde(rename="dolor")]
    )]
    ipsum: String,
}
fn main() {
    let mut show = LoremBuilder::default();
    show.ipsum("sit".into());
    assert_eq!(serde_json::to_string(&show).unwrap(), r#"{"dolor":"sitx"}"#);
}
