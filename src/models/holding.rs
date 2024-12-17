struct Holding {
    id: Uuid,
    holding_type: HoldingType,
    name: String,
    created_at: DateTime<Utc>,
}

enum HoldingType {
    SecurityPortfolio,
    CurrentAccount,
    SavingsAccount,
    RealEstate,
    PensionSavings,
}
