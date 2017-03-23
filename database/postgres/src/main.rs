extern crate postgres;
#[macro_use] extern crate serde_json;
extern crate dotenv;

use postgres::{Connection, TlsMode};
use postgres::rows::{Rows};
use postgres::types::{FromSql};

use serde_json::{Value, Map};

fn main() {
}

fn rows_to_json(rows: &Rows) -> Value {
	let mut rows_json = Vec::with_capacity(rows.len());
	for _ in 0..rows.len() {
		rows_json.push(Map::new());
	}

	for i in 0..rows.columns().len() {
		column_to_json(rows, i, &mut rows_json);
	}

	return Value::from(rows_json);
}

fn column_to_json(rows: &Rows, column_pos: usize, rows_json: &mut Vec<Map<String, Value>>) {
	if let Some(column) = rows.columns().get(column_pos) {
		let t = column.type_();
		if <i32 as FromSql>::accepts(t) {
			_column_to_json::<i32>(rows, column.name(), column_pos, rows_json);
		} else if <String as FromSql>::accepts(t) {
			_column_to_json::<String>(rows, column.name(), column_pos, rows_json);
		} else if <Vec<u8> as FromSql>::accepts(t) {
			_column_to_json::<Vec<u8>>(rows, column.name(), column_pos, rows_json);
		} 
	}
}

fn _column_to_json<T: FromSql>(rows: &Rows, column_name: &str, 
	column_pos: usize, rows_json: &mut Vec<Map<String, Value>>) 
	where serde_json::Value: std::convert::From<T> {

	for (j, row) in rows.iter().enumerate() {
		if let Some(row_json) = rows_json.get_mut(j) {
			row_json.insert(
				column_name.to_string(), 
				Value::from(row.get::<usize, T>(column_pos))
			);
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;

    #[test]
    fn json() {
    	dotenv().ok();
    	let database_url = env::var("POSTGRES_URL")
        	.expect("POSTGRES_URL must be set");
        let conn = Connection::connect(database_url, TlsMode::None).unwrap();
        conn.execute("drop TABLE person", &[]).unwrap_or(0);
	    conn.execute("CREATE TABLE person (
	                    id              SERIAL PRIMARY KEY,
	                    name            VARCHAR NOT NULL,
	                    data            BYTEA
	                  )", &[]).unwrap();
	    
	    let data = vec![
	    	vec![0u8,1u8,2u8],
	    	vec![]
	    ];
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

	    let stmt = conn.prepare("INSERT INTO person (name, data) VALUES ($1, $2)").unwrap();
	    stmt.execute(&[&p[0]["name"].as_str().unwrap(), &data[0]]).unwrap();
	    stmt.execute(&[&p[1]["name"].as_str().unwrap(), &data[1]]).unwrap();

	    let j = rows_to_json(&conn.query("SELECT id, name, data FROM person", &[]).unwrap());
	    
	    println!("p = {:?}", p.to_string());
	    println!("j = {:?}", j.to_string());
	    assert_eq!(p, j);

	    conn.execute("drop TABLE person", &[]).unwrap();
    }
}
//[cfg(test)]