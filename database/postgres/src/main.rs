use postgres::row::Row;
use postgres::types::FromSql;
use serde_json::{Map, Value};

fn main() {}

fn _rows_to_json(rows: &Vec<Row>) -> Value {
    let mut rows_json = Vec::with_capacity(rows.len());

    for row in rows {
        let mut row_json = Map::new();
        for col in row.columns() {
            let key = col.name();
            let ty = col.type_();

            let value = if i32::accepts(ty) {
                Value::from(row.get::<&str, i32>(key))
            } else if String::accepts(ty) {
                Value::from(row.get::<&str, String>(key))
            } else if Vec::<u8>::accepts(ty) {
                Value::from(row.get::<&str, Vec<u8>>(key))
            } else {
                Value::Null
            };

            row_json.insert(key.to_string(), Value::from(value));
        }
        rows_json.push(row_json);
    }

    return Value::from(rows_json);
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use postgres::{Client, NoTls};
    use serde_json::json;
    use std::env;

    #[test]
    fn json() -> Result<(), postgres::error::Error> {
        dotenv().ok();

        let host = env::var("PGHOST").expect("PGHOST must be set");
        let port = env::var("PGPORT").expect("PGPORT must be set");
        let user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let pwd = env::var("POSTGRES_PASSWD").expect("POSTGRES_PASSWD must be set");
        let db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

        let database_url = format!(
            "host={} port={} user={} password={} dbname={} connect_timeout=10 keepalives=0",
            host, port, user, pwd, db
        );
        let mut conn = Client::connect(&database_url, NoTls)?;

        conn.execute("DROP TABLE IF EXISTS person", &[])?;
        conn.execute(
            "CREATE TABLE person (
	                    id              SERIAL PRIMARY KEY,
	                    name            VARCHAR NOT NULL,
	                    data            BYTEA
	                  )",
            &[],
        )?;

        let data = vec![vec![0u8, 1u8, 2u8], vec![]];
        let p = json!([
            {
                "id": 1,
                "name": "a",
                "data": data[0]
            },
            {
                "id": 2,
                "name": "b",
                "data": data[1]
            }
        ]);

        let stmt = conn.prepare("INSERT INTO person (name, data) VALUES ($1, $2)")?;
        conn.execute(&stmt, &[&p[0]["name"].as_str().unwrap(), &data[0]])?;
        conn.execute(&stmt, &[&p[1]["name"].as_str().unwrap(), &data[1]])?;

        let j = _rows_to_json(&conn.query("SELECT id, name, data FROM person", &[])?);

        println!("p = {:?}", p.to_string());
        println!("j = {:?}", j.to_string());
        assert_eq!(p, j);

        conn.execute("drop TABLE person", &[])?;

        Ok(())
    }
}
