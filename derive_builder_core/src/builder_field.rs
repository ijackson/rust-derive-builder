use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn;
use syn::spanned::Spanned;

/// Field for the builder struct, implementing `quote::ToTokens`.
///
/// # Examples
///
/// Will expand to something like the following (depending on settings):
///
/// ```rust,ignore
/// # extern crate proc_macro2;
/// # #[macro_use]
/// # extern crate quote;
/// # #[macro_use]
/// # extern crate syn;
/// # #[macro_use]
/// # extern crate derive_builder_core;
/// # use derive_builder_core::{BuilderField, BuilderPattern};
/// # fn main() {
/// #    let attrs = vec![parse_quote!(#[some_attr])];
/// #    let mut field = default_builder_field!();
/// #    field.attrs = attrs.as_slice();
/// #
/// #    assert_eq!(quote!(#field).to_string(), quote!(
/// #[some_attr] pub foo: ::derive_builder::export::core::option::Option<String>,
/// #    ).to_string());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct BuilderField<'a> {
    /// Name of the target field.
    pub field_ident: &'a syn::Ident,
    /// Type of the target field.
    ///
    /// The corresonding builder field will be `Option<field_type>`.
    pub field_type: &'a syn::Type,
    /// Whether the builder implements a setter for this field.
    ///
    /// Note: We will fallback to `PhantomData` if the setter is disabled
    ///       to hack around issues with unused generic type parameters - at
    ///       least for now.
    pub field_enabled: bool,
    /// Visibility of this builder field, e.g. `syn::Visibility::Public`.
    pub field_visibility: syn::Visibility,
    /// Attributes passed through, to be attached to this builder field.
    pub pass_attrs: &'a [syn::Attribute],
    /// Extra attributes explicitly requested to be attached to the builder field.
    pub nested_attrs: &'a [syn::Meta],
}

impl<'a> ToTokens for BuilderField<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.field_enabled {
            let vis = &self.field_visibility;
            let ident = self.field_ident;
            let ty = self.field_type;
            let pass_attrs = self.pass_attrs;

            // The input tokens do not contain the #[ ] parts of an attribute, so we must provide them.
            // And we want to give these extra tokens a span derived from the input.
            let nested_attrs = self
                .nested_attrs
                .iter()
                .map(|meta| quote_spanned!(meta.span() => #[#meta]));
            let q = quote!(
                #(#pass_attrs)* #(#nested_attrs)*
                #vis #ident: ::derive_builder::export::core::option::Option<#ty>,
            );
            eprintln!("FIELD {}", q);
            dbg!(&q);
            tokens.append_all(q);
        } else {
            let ident = self.field_ident;
            let ty = self.field_type;
            let pass_attrs = self.pass_attrs;

            tokens.append_all(quote!(
                #(#pass_attrs)* #ident: ::derive_builder::export::core::marker::PhantomData<#ty>,
            ));
        }
    }
}

impl<'a> BuilderField<'a> {
    /// Emits a struct field initializer that initializes the field to `Default::default`.
    pub fn default_initializer_tokens(&self) -> TokenStream {
        let ident = self.field_ident;
        quote! { #ident : ::derive_builder::export::core::default::Default::default(), }
    }
}

/// Helper macro for unit tests. This is _only_ public in order to be accessible
/// from doc-tests too.
#[doc(hidden)]
#[macro_export]
macro_rules! default_builder_field {
    () => {{
        BuilderField {
            field_ident: &syn::Ident::new("foo", ::proc_macro2::Span::call_site()),
            field_type: &parse_quote!(String),
            field_enabled: true,
            field_visibility: parse_quote!(pub),
            pass_attrs: &[parse_quote!(#[some_attr])],
            nested_attrs: &[],
        }
    }};
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn setter_enabled() {
        let field = default_builder_field!();

        assert_eq!(
            quote!(#field).to_string(),
            quote!(
                #[some_attr] pub foo: ::derive_builder::export::core::option::Option<String>,
            )
            .to_string()
        );
    }

    #[test]
    fn setter_disabled() {
        let mut field = default_builder_field!();
        field.field_enabled = false;

        assert_eq!(
            quote!(#field).to_string(),
            quote!(
                #[some_attr]
                foo: ::derive_builder::export::core::marker::PhantomData<String>,
            )
            .to_string()
        );
    }

    #[test]
    fn private_field() {
        let private = syn::Visibility::Inherited;
        let mut field = default_builder_field!();
        field.field_visibility = private;

        assert_eq!(
            quote!(#field).to_string(),
            quote!(
                #[some_attr]
                foo: ::derive_builder::export::core::option::Option<String>,
            )
            .to_string()
        );
    }
}
