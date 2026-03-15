# SQLX UnitOfWork Demo
Demo for a UnitOfWork wrapping calls to multiple postgres repos in a single transaction

The concept was taken from this brilliant [article.](https://medium.com/@patrickkoss/the-unit-of-work-pattern-in-rust-2bd620f6d517) This repo implements a worked example.

## Architecture

Implementig a somewhat simplified hexagonal architecture (e.g. no ports, no inbound adapters).

## Setup

### Environment

The system checks for a `.env` file in the project root folder to find certain configuration variables. These values are needed for the dataase layer including the docker setup of the database. After cloning the project, you need to set one up:

```bash
#.env 
# variables are mandatory, values given are a dev default
DB_TYPE=postgres
DB_HOST=localhost
DB_USER=postgres
DB_PASSWORD=postgres
DB_NAME=process
```
### Database Setup

```bash
# from project root folder
docker compose --env-file .env -f ./docker/compose.yaml up
```
Assuming a postgres server is now running on localhost, observe that an empty db has been started: 

```bash
psql -h localhost -p 5432 -U postgres
postgres=# \d
Did not find any relations.
postgres=# 
```
### Database Migration

To setup the tables in the postgres database, we use sqlx-cli:
```bash
cargo install sqlx-cli
...
export DATABASE_URL=postgres://postgres:postgres@localhost/process;  
sqlx migrate run
```
this executes the pre-created migration in `src/migrations'.

Check the structure with psql:
```bash
postgres=:# \c process
You are now connected to database "process" as user "postgres".
process=# \dt
              List of relations
 Schema |       Name       | Type  |  Owner   
--------+------------------+-------+----------
 public | _sqlx_migrations | table | postgres
 public | outbox           | table | postgres
 public | resources        | table | postgres
(3 rows)

```

## Run
```bash
# Build and run
cargo run 
```

Then use psql to observe the entries made in the outbox and resources tables.

```bash
process=# select * from resources;
...

process=# select * from outbox;
```
## License

MIT

# Discussion

To visualize the Rust design in PlantUml (which does not lend itself towards that language), I am adapting the notation a bit: 

![Notation](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/onouv/sql-unit-of-work-sprint/main/doc/legend.class.puml)

A "complex service" is one that needs to bundle multiple repository accesses under a single transaction as discussed in the [article](https://medium.com/@patrickkoss/the-unit-of-work-pattern-in-rust-2bd620f6d517), and frequently encountered when a service needs to do a business domain transaction, but also publish a message about this on a messaging system (e.g. Kafka) to let other services know about the fact. For this, often the outbox pattern is used. We are just emulating the initial step of this here, namely writing to an outbox table. In reality, this would asynchronously be evaluated by a CDC adapter and the appropriate messages would be published.  

In essence, this is just an example of how you must run operations in a common transaction. You don't want to post when the business transaction has failed and is rolling back, or all hell breaks loose in the overall system and customers become angry. So all goes in the same transaction.  

This is the responsibility of the [UnitOfWork](./src/application/uow.rs). This struct handles an sqlx  connection `Pool` and the associaed sqlx `Transaction` when running the callback provided by the client code.

![Using the UnitOfWork](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/onouv/sql-unit-of-work-sprint/main/doc/overview_complex_service.class.puml)

To set all this up, everything is created in the `main` function. It creates a `UnitOfWork`. This is passed as a `clone()` to the application services constructor, together with the required repositories. The `UnitOfWork` only contains the `PgPool` which is `Clone`. The repos are `Clone` for even less money. 

![Creation and Injection](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/onouv/sql-unit-of-work-sprint/main/doc/setup_complex_service.class.puml)

The service constructor also does a bunch of `Arc` and `Rc` magic - just read the [article](https://medium.com/@patrickkoss/the-unit-of-work-pattern-in-rust-2bd620f6d517), and goes to town for its business work.  

The different things a domain service needs to do now are delegated into specialized units of work, like `CreateResourceUoW`, which keeps the domain service nice and tidy.