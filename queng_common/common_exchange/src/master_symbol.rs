#[derive(Debug)]
pub struct MasterSymbolRow {
    pub data: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MasterSymbol {
    pub master_symbol: String,
    pub asset_class: String,
    pub base_asset: String,
    pub quote_asset: String,
}
