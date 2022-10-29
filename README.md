# Diesel Trivial Type Conversion Example

A demonstration of implementing a custom Rust type as trivial conversion to
an existing database-supported type such as `Text`.

## Requirements

In order to convert from an arbitrary Rust type to a SQL type, there are a
number of things that must be implemented. An example is used here to
illustrate the requirements in more specific terms.

For a Rust type `Example` which converts to the Rust type `String`
and the SQL type `Text`:

- [ ] `Example` declaration with:
  - [ ] [`#[derive(AsExpression)`](https://docs.diesel.rs/2.0.x/diesel/expression/derive.AsExpression.html)
  - [ ] [`#[derive(FromSqlRow)`](https://docs.diesel.rs/2.0.x/diesel/deserialize/derive.FromSqlRow.html)
- [ ] An error type for conversion issues that implements:
  - [ ] [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
  - [ ] [`std::error::Error`](https://doc.rust-lang.org/std/fmt/struct.Error.html)
- [ ] Appropriate [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) implementations for:
  - [ ] At least one of:
    - [ ] `impl<'a> TryFrom<&'a Example> for &'a String`
    - [ ] `impl TryFrom<Example> for String`
  - [ ] `impl TryFrom<String> for Example`
- [ ] An implementation for the target database type(s) of:
  - [ ] [`impl ToSql for Example`](https://docs.diesel.rs/2.0.x/diesel/serialize/trait.ToSql.html) that uses `TryFrom<&Example>` or `TryFrom<Example>`
  - [ ] [`impl FromSql for Example`](https://docs.diesel.rs/2.0.x/diesel/deserialize/trait.FromSql.html) that uses `TryFrom<String>`

A working example is implemented in [`src/label.rs`](src/label.rs) that can be
used as a template for other implementations.

### Postgres Example

An example `ToSql` implementation that wraps around the `String` version to
convert to `Text` looks like:

```rust
impl diesel::serialize::ToSql<Text, Pg> for Label {
    fn to_sql(&self, out: &mut diesel::serialize::Output<'_, '_, Pg>) -> diesel::serialize::Result {
        <String as diesel::serialize::ToSql<Text, Pg>>::to_sql(self.try_into()?, &mut out.reborrow())
    }
}
```

An example `FromSql` implementation that wraps around the `String` version to
handle conversion from `Text` looks like:

```rust
impl diesel::deserialize::FromSql<Text, Pg> for Label {
    fn from_sql<'a>(bytes : diesel::backend::RawValue<'a,Pg>) -> diesel::deserialize::Result<Self> {
        <String as diesel::deserialize::FromSql<Text, Pg>>::from_sql(bytes)
            .and_then(|s| Self::try_from(s).map_err(|e| e.into()))
    }
}
```

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
