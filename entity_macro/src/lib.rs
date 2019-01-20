#![recursion_limit = "256"]
extern crate proc_macro;
use crate::proc_macro::TokenStream;
use inflector::Inflector;
use proc_macro2;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;

#[proc_macro_derive(Entity)]
pub fn entity(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_entity_macro(&ast)
}

fn impl_entity_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let struct_result_name = Ident::new(&format!("{}SearchResult", struct_name), Span::call_site());
    let struct_browse_name = Ident::new(&format!("{}BrowseResult", struct_name), Span::call_site());

    let endpoint = (format!("{}", struct_name) as String).to_snake_case();

    let api_endpoint = (format!("{}", struct_name) as String).to_kebab_case();

    let field_result = Ident::new(
        &{
            if endpoint.chars().nth(endpoint.len() - 1).unwrap() == 's'
                || endpoint.chars().nth(endpoint.len() - 1).unwrap() == 'x'
            {
                format!("{}es", endpoint)
            } else {
                format!("{}s", endpoint)
            }
        },
        Span::call_site(),
    );

    let field_offset = Ident::new(&format!("{}_offset", endpoint), Span::call_site());
    let field_count = Ident::new(&format!("{}_count", endpoint), Span::call_site());

    let gen = quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct #struct_result_name {
            pub created: String,
            pub count: i32,
            pub offset: i32,
            pub #field_result: Vec<#struct_name>
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct #struct_browse_name {
            pub #field_offset: i32,
            pub #field_count: i32,
            pub #field_result: Vec<#struct_name>
        }

        impl Entity for #struct_name {
            fn lookup(bernard_request: &mut super::super::Bernard) ->
                Box<Future<Item = #struct_name, Error = hyper::Error>> {

                bernard_request.build_lookup_uri(#api_endpoint);

                let body = bernard_request.get().and_then(|res| {
                        res.into_body().concat2()
                  });

                let data_struct = body.and_then(move |body| {
                    let res: #struct_name = serde_json::from_slice(&body).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            e
                        )
                    }).unwrap();
                    futures::future::ok(res)
                });

                Box::new(data_struct)
            }

            fn search(bernard_request: &mut super::super::Bernard) ->
                Box<Future<Item = Vec<#struct_name>, Error = hyper::Error>> {

                bernard_request.build_search_uri(#api_endpoint);

                let body = bernard_request.get().and_then(|res| {
                    res.into_body().concat2()
                });

                let data_struct = body.and_then(move |body| {
                    let res: #struct_result_name = serde_json::from_slice(&body).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            e
                        )
                    }).unwrap();
                    futures::future::ok(res.#field_result)
                });

                Box::new(data_struct)
            }

            fn browse(bernard_request: &mut super::super::Bernard) ->
                Box<Future<Item = Vec<#struct_name>, Error = hyper::Error>> {

                bernard_request.build_search_uri(#api_endpoint);

                let body = bernard_request.get().and_then(|res| {
                    res.into_body().concat2()
                });

                let data_struct = body.and_then(move |body| {
                    let res: #struct_browse_name = serde_json::from_slice(&body).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            e
                        )
                    }).unwrap();
                    futures::future::ok(res.#field_result)
                });

                Box::new(data_struct)
            }
        }
    };

    gen.into()
}
