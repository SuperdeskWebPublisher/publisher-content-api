# Publisher Content API

Superdesk Publisher Content (read only) API powered by GraphQL. It allows to fetch resources from the Superdesk Publisher.

### Features:

- GraphQL support.
- Integrated GraphiQL to explore GraphQL API.
- Fetch Superdesk Publisher articles.
- Pagination is designed to work with the [Relay Cursor Connections](https://facebook.github.io/relay/graphql/connections.htm) spec.

This project is using the following libraries:

- [Rocket](https://rocket.rs) (web server)
- [Diesel](http://diesel.rs) (database)
- [Juniper](https://github.com/graphql-rust/juniper) (graphql)
- [juniper-from-schema](https://github.com/davidpdrsn/juniper-from-schema) (graphql code generation)
- [juniper-eager-loading](https://github.com/davidpdrsn/juniper-eager-loading) (eager loading to avoid N+1 query bugs)


## Installation

1. Open `.env` file and set the value of `DATABASE_URL` to the URL of the Publisher's database (postgres).
2. Run `cargo run`
3. Then go to http://localhost:8000/graphiql

## Configuration

In `.env` file:

- `PUBLISHER_CDN_URL` - CDN URL for images to properly generate URLs when the images are hosted on, for example AWS.
