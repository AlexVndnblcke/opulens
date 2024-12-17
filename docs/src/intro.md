# Introduction

The Opulens project provides a means to get aggregated information about someones wealth.

In its initial stage it will focus on securities portfolios. After which additional types of wealth can be added. But as this is most important to its creator, this will be the first scope.

Opulens will provide an interface to model a person their different portfolio's from different platforms. All this portfolio information will be bundled in a single wealth overview. A portfolio itself consists of multiple securities which are in fact a snapshot over all transactions. This means that internally transactions will be stored and the portfolio state can be calculated based on those transactions.

For all the other parts, Opulens will provide API interfaces to perform the CRUD operations on parts of the portfolio.

## Future

As a first extension, mutli-user support is scheduled:

Opulens will be a server application which is offered in a Saas way, meaning that a single server will be able to handle multiple users. User handling will be performed by an external authentication platform (e.g. https://github.com/casbin/casbin-rs).

In the future it is the intention to have the following features available:

- dashboarding
- portfolio uploads from different systems
- integrations with brokers (if possible)
