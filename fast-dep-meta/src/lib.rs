use pyo3::prelude::*;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Fields, Meta, Path};
use quote::quote;

const PY_NAME_ATTRIBUTE: &str = "PyName";

#[proc_macro_derive(AstFromPy, attributes(PyName))]
pub fn ast_from_py_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let field_parsers = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                fields.named.iter().map(|field| {
                    let mut name: String = field.ident.as_ref().unwrap().to_string();
                    for attr in &field.attrs {
                        if attr.path().is_ident(PY_NAME_ATTRIBUTE) {
                            match &attr.meta {
                                Meta::Path(path) => panic!("Path"),
                                Meta::List(list) => {
                                    let value: Meta::NameValue = list.parse_args().unwrap();
                                    panic!("{}", value.as_str());
                                }
                                Meta::NameValue(named) => panic!("Named")
                            }
                            // if let Meta::Path(path) = &attr.meta {
                            //     panic!("{}", path.get_ident().unwrap().to_string());
                            //     //name = path.get_ident().unwrap().to_string();
                            // }
                            // if let Meta::List(list) = 
                            // panic!("yo");

                        }
                    }
                    quote! {
                        print!("{}", #name);
                    }
                })
            },
            _ => unimplemented!("Only named fields are supported"),
        },
        _ => unimplemented!("Only structs are supported."),
    };

    let ident = ast.ident;

    let gen = quote! {
        impl <'a> FromPyObject<'a> for #ident {
            fn extract(ob: &'a PyAny) -> PyResult<Self> {
                #(#field_parsers)*
                Ok(#ident{ name: "yo".to_string() })
            }
        }
    };

    let stream: TokenStream = gen.into();

    panic!("{}", stream.to_string())
    //stream
    // let gen = quote! {
    //     impl <'a> FromPyObject<'a> for #name {
    //         fn extract(ob: &'a PyAny) -> PyResult<Self> {
    //             let name = ob.get_type().name()?;
    //             print!("\n{}\n", name);
    //             print!("{}\n", ob.dir());

    //             Ok(#name{ name: name.to_string() })

    //         }
    //     }
    // };

    // gen.into()
}