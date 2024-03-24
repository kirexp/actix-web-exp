extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};



#[proc_macro_derive(ApiResponseResponder)]
pub fn my_responder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    use shared::ApiResponse;
    use serde::{Deserialize, Serialize};
    let name = &input.ident;
    let gen = quote! {
        impl actix_web::Responder for #name {
                type Body = BoxBody;
                fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
                let wrapped_response = ApiResponse {
                    data: self,
                    is_success: true, // Set based on your logic
                };

                let body = serde_json::to_string(&wrapped_response).unwrap();

                // Create response and set content type
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(body)
            }
        }
    };

    gen.into()
}