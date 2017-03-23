# Trying Diesel

## Database

### Setting up PostgreSQL

```
docker run --name postgres -p 5432:5432 -d postgres
```

The default user for accessing the local PostgreSQL server is postgres with a blank password.

### Create User

```
psql -h localhost -U postgres -c "create user diesel createdb password 'diesel'"
```

## Diesel

### Installing Diesel command tool

```
# Compiling diesel_cli requires
brew install mysql
brew install postgresql

cargo install diesel_cli
```

### Creating Database

```
diesel setup
```

This will create our database (if it didn't already exist), and create an empty migrations directory that we can use to manage our schema. If the migrations directory exists, each migration script will be run.

Or creat database manually

```
psql -h localhost -U postgres -c "create database diesel with owner diesel"
```

### Create migrations

```
diesel migration generate $migration_name
```

### Apply migrations

```
diesel migration run
```

## Where is more examples

https://github.com/diesel-rs/diesel/tree/master/diesel_tests

## Where is the file ".env"

Since it is in a cargo workspace, so .env is in workspace's root.