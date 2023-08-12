use rusqlite::{Connection, Result};

fn main() {
    /**************************************************
     *  SQLite database example
     *
     * For windows users:
     *  - https://gist.github.com/zeljic/d8b542788b225b1bcb5fce169ee28c55
     *  - https://github.com/rusqlite/rusqlite#user-content-notes-on-building-rusqlite-and-libsqlite3-sys
     *
     ***************************************************/

    let db_conn = create_database();

    create_table(&db_conn);

    insert_pet(&db_conn, "Leonidas");

    let pet = get_pet(&db_conn, 1);

    println!("Pet: {}", pet.unwrap());
}

fn create_database() -> Connection {
    let conn_result = Connection::open("./pets-database.db3");

    match conn_result {
        Ok(_) => {
            println!("SQLite database connected");
            return conn_result.unwrap();
        }
        Err(err) => panic!("Error connecting SQLite: {}", err),
    };
}

fn create_table(conn: &Connection) {
    let sql_query = "
    CREATE TABLE IF NOT EXISTS pets (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )";

    let query_result = conn.execute(sql_query, []);

    match query_result {
        Ok(_) => {
            println!("SQLite table created succesfully");
        }
        Err(err) => panic!("Error creating SQLite table: {}", err),
    };
}

fn insert_pet(conn: &Connection, name: &str) {
    let sql_query = "
    INSERT INTO pets (name) values (?1)
    ";

    let query_result = conn.execute(sql_query, &[name]);

    match query_result {
        Ok(_) => {
            println!("Pet created succesfully");
        }
        Err(err) => panic!("Error creating pet: {}", err),
    };
}

fn get_pet(conn: &Connection, id: i32) -> Result<String> {
    let mut statement = conn.prepare("SELECT (name) FROM pets WHERE id = ?1")?;

    let pets_iter = statement.query_map([id], |row| {
        let pet_name: String = row.get(0)?;
        Ok(pet_name)
    })?;

    for pet in pets_iter {
        let pet = Ok(pet.unwrap());
        return pet;
    }

    Ok(String::from(""))
}
