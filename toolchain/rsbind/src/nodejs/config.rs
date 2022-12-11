#[derive(Clone, Deserialize, Debug)]
pub struct NodeJS {
    pub contract_name: Option<String>,
    pub imp_name: Option<String>,
}

impl Default for NodeJS {
    fn default() -> Self {
        NodeJS {
            contract_name: None,
            imp_name: None,
        }
    }
}
