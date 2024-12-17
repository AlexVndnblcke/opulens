### Opulens Definition

#### Domain description

The following paragraphs will describe the domain logic and elements in a plain text. This is used to identify the key components of the project.

A user wants to get a concise view of their wealth. The wealth of user consists of different holdings. These holdings can be of several types a.o. security portfolio's, current accounts, savings accounts, real estate, pension savings.

In real life, a holding can of course have multiple shareholders. E.g. a shared portfolio between 2 spouses. Or an inherited property which is owned by the different children. This makes that a holding can have multiple owners, and each owner caan have a percentage of ownership which isn't necessarily equal.

It is possible that a single user has multiple of holdings of the same type, consider the scenario where a user has a savings account for themselves and one shared with a spouse. This can also be the scenario for security portfolio's, etc.

The user will require some overview of the total wealth they've accumulated through time. This means that each of the holding types will be represented by a set of transactions. For each holding type, the transaction will be distinct. For a securities portfolio, a transaction can be the acquisition or liquidation of shares, bonds, crypto, etc. For a savings account, the transactions will be deposits or withdrawals. For real estate, the purchase or disposal of properties.

Each transaction will need a timestamp information, the actual value of the asset at that time, a unique identification of the asset, a holding identifier.

Cash-in and cash-out will be different for each holding type. A real estate property can come with costs for maintenance and tax as well as income from rent. This information has no direct impact on the holding so won't be accumulated there. However, interests on a savings account are typically payed out on the savings account itself so are a new transaction in itself. Each holding type will have its own definition of how these cash operations are handled, but won't be part of the original product.

####
