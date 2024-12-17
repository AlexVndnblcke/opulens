struct Asset {
    id: Uuid,
    asset_type: AssetType,
    description: String,
}

enum AssetType {
    Stock,
    Bond,
    Crypto,
    Property,
    Cash,
}
