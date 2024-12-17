struct Transaction {
    id: Uuid,
    holding_id: Uuid,
    timestamp: DateTime<Utc>,
    asset_id: Uuid,
    value: f64,
    transaction_type: TransactionType,
}

enum TransactionType {
    // Income Types
    RentIncome,      // Rental income from real estate
    Dividend,        // Dividends from securities
    Interest,        // Interest income from savings
    CapitalGain,     // Realized gain from asset liquidation

    // Cost Types
    MaintenanceCost, // Maintenance expenses for real estate
    Tax,             // Taxes on holdings or income
    Fees,            // Management or transaction fees
    CapitalLoss,     // Realized loss from asset liquidation

    // Neutral/Other Types
    Deposit,         // Deposits to a holding
    Withdrawal,      // Withdrawals from a holding
    Acquisition,     // Asset purchase (not inherently cost/income)
    Liquidation,     // Asset sale (not inherently cost/income)
}
