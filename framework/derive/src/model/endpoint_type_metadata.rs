/// Used to generate the EndpointType in the ABI.
#[derive(Debug, Clone)]
pub enum EndpointTypeMetadata {
    Init,
    Upgrade,
    Endpoint,
    PromisesCallback,
}

impl EndpointTypeMetadata {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            EndpointTypeMetadata::Init => {
                quote! { drt_sc::abi::EndpointTypeAbi::Init }
            },
            EndpointTypeMetadata::Upgrade => {
                quote! { drt_sc::abi::EndpointTypeAbi::Upgrade }
            },
            EndpointTypeMetadata::Endpoint => {
                quote! { drt_sc::abi::EndpointTypeAbi::Endpoint }
            },
            EndpointTypeMetadata::PromisesCallback => {
                quote! { drt_sc::abi::EndpointTypeAbi::PromisesCallback }
            },
        }
    }
}
