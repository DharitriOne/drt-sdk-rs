/// Contains metdata from the `#[payable(...)]` attribute.
/// Only endpoints and the constructor can be marked payable.
#[derive(Clone, Debug)]
pub enum MethodPayableMetadata {
    NotPayable,
    Rewa,
    SingleDctToken(String),
    AnyToken,
}

impl MethodPayableMetadata {
    pub fn is_payable(&self) -> bool {
        !matches!(self, MethodPayableMetadata::NotPayable)
    }

    pub fn no_dct(&self) -> bool {
        matches!(
            self,
            MethodPayableMetadata::NotPayable | MethodPayableMetadata::Rewa
        )
    }

    pub fn abi_strings(&self) -> Vec<String> {
        match self {
            MethodPayableMetadata::NotPayable => Vec::new(),
            MethodPayableMetadata::Rewa => vec!["REWA".to_string()],
            MethodPayableMetadata::SingleDctToken(s) => vec![s.clone()],
            MethodPayableMetadata::AnyToken => vec!["*".to_string()],
        }
    }
}
