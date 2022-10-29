# Diesel Trivial Type Conversion Example

A demonstration of implementing a custom Rust type as trivial conversion to
an existing database-supported type such as `Text`.

## Configuration

For testing, a database is necessary:

### Postgres

Create the test database, for example:

```sql
CREATE USER example LOGIN PASSWORD 'example';
CREATE DATABASE example OWNER=example;
```

These credentials are configured in the `.env` file:

```
DATABASE_URL=postgres://example:example@localhost/example
```

Once prepared, run the migrations:

```shell
diesel migration run
```
