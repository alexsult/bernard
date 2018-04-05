#![recursion_limit = "256"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate inflector;
extern crate percent_encoding;
extern crate regex;

use proc_macro::TokenStream;
use inflector::Inflector;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

// copied pasted from:
// https://doc.rust-lang.org/beta/book/procedural-macros.html
#[proc_macro_derive(Entity)]
pub fn entity(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_entity(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_entity(ast: &syn::MacroInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let struct_result_name = quote::Ident::from(format!("{}SearchResult", struct_name));
    let struct_browse_name = quote::Ident::from(format!("{}BrowseResult", struct_name));

    let endpoint = (format!("{}", struct_name) as String).to_snake_case();
    let api_endpoint = (format!("{}", struct_name) as String).to_kebab_case();

    let mut field_result = quote::Ident::new("");

    if endpoint.chars().nth(endpoint.len() - 1).unwrap() == 's' ||
        endpoint.chars().nth(endpoint.len() - 1).unwrap() == 'x' {
        field_result = quote::Ident::from(format!("{}es", endpoint));
    }
    else {
        field_result = quote::Ident::from(format!("{}s", endpoint));
    }
    let field_offset = quote::Ident::from(format!("{}_offset", endpoint));
    let field_count = quote::Ident::from(format!("{}_count", endpoint));

    quote! {
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
            fn lookup(&self,
                       client: &super::super::Bernard,
                       entity_id: &Uuid,
                       params: &mut HashMap<&str, &str>
                      ) -> Box<Future<Item = Self, Error = hyper::Error>> {

                let uri = &format!("{endpoint}/{id}",
                                   endpoint=#api_endpoint,
                                   id=entity_id);

                let body = client.get(uri, params).and_then(|res| {
                    res.body().concat2()
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

            fn search(&self,
                      client: &super::super::Bernard,
                      params: &mut HashMap<&str, &str>
                      ) -> Box<Future<Item = Vec<Self>, Error = hyper::Error>> {

                let body = client.get(#api_endpoint,
                                      params).and_then(|res| {
                                            res.body().concat2()
                                      });

                let search_results = body.and_then(move |body| {
                    let res: #struct_result_name = serde_json::from_slice(&body).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            e
                        )
                    }).unwrap();
                    futures::future::ok(res.#field_result)
                });

                Box::new(search_results)
            }

            fn browse(&self,
                      client: &super::super::Bernard,
                      params: &mut HashMap<&str, &str>
                      ) -> Box<Future<Item = Vec<Self>, Error = hyper::Error>> {

                let body = client.get(#api_endpoint,
                                      params).and_then(|res| {
                                            res.body().concat2()
                                      });

                let search_results = body.and_then(move |body| {
                    let res: #struct_browse_name = serde_json::from_slice(&body).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            e
                        )
                    }).unwrap();
                    futures::future::ok(res.#field_result)
                });

                Box::new(search_results)
            }
        }
    }
}
