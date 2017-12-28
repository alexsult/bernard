#![recursion_limit = "128"]
#![feature(concat_idents)]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

// copy paste from:
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
    
    /*
    let endpoint = match &format!("{}", struct_name) as &str {
        "Artist" => "artist",
        "Area" => "area",
        "Release" => "release",
        _ => ""
    };
    */
    let endpoint = (format!("{}", struct_name) as String).to_lowercase();


    let field_result = quote::Ident::from(format!("{}s", endpoint));
    let field_offset = quote::Ident::from(format!("{}_offset", endpoint));
    let field_count = quote::Ident::from(format!("{}_count", endpoint));

    quote! {
		#[derive(Debug, Clone, Serialize, Deserialize)]
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
                       client: &super::MusicBrainz,
                       entity_id: &Uuid,
                       params: &mut HashMap<&str, &str>) -> Result<Self, Error> {

                let data = match client.get(&format!("{endpoint}/{id}", 
                                                            endpoint=#endpoint,
                                                            id=entity_id), 
                                                   params) {
                    Ok(x) => x,
                    Err(e) => return Err(Error::Get(e)) 
                };

                let data_struct: #struct_name =
                                  match serde_json::from_str(&data) {
                
                    Ok(x) => x,
                    Err(e) => return Err(Error::ParseJson(e))
                };

                Ok(data_struct)
            }

            fn search(&self, 
                      client: &super::MusicBrainz, 
                      params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error> {

                let data = match client.get(&format!("{endpoint}",
                                                     endpoint=#endpoint),
                                            params) {
                    Ok(x) => x,
                    Err(e) => return Err(Error::Get(e)) 
                };

                //let mut results: Vec<#struct_name> = Vec::new();

                let search_results: #struct_result_name = match serde_json::from_str(&data) {
                    Ok(x) => x,
                    Err(e) => return Err(Error::ParseJson(e))
                };

                Ok(search_results.#field_result )
            }

            fn browse(&self, 
                      client: &super::MusicBrainz, 
                      params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error> {

                let data = match client.get(&format!("{endpoint}",
                                                     endpoint=#endpoint),
                                            params) {
                    Ok(x) => x,
                    Err(e) => return Err(Error::Get(e)) 
                };

                //let mut results: Vec<#struct_name> = Vec::new();

                let search_results: #struct_browse_name = match serde_json::from_str(&data) {
                    Ok(x) => x,
                    Err(e) => return Err(Error::ParseJson(e))
                };

                Ok(search_results.#field_result )
            }
        }
    }
}
