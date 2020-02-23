mod psqlx_fn;
use postgres::{Client, NoTls};

fn main() {
    let mut client = Client::connect("host=127.0.0.1 user=lipeng dbname=baseinfo_test", NoTls).unwrap();

    table_names(&mut client);
    //SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'psqlx_fn::sqlx_lib_fn();
    println!("Hello, world!");
}

fn table_names(client: &mut postgres::Client) {
    println!("Hello, world!");

    if let Ok(rows) = client.query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'", &[]) {
        for row in &rows {
            let v: String = row.get(0);
            println!("table name: {}", v);
        }
    } 
}

fn postgresql_lib_fn() {
    let mut client = Client::connect("host=127.0.0.1 user=lipeng dbname=baseinfo_test", NoTls).unwrap();
    println!("Hello, world!");

    if let Ok(rows) = client.query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'", &[]) {
        for row in &rows {
            let v: String = row.get(0);
            println!("row id: {}", v);
        }
        for (i, row) in rows.iter().enumerate() {
            let v: i64 = row.get(0);
            println!("index: {}, row id: {}", i, v);
        }
    }

    if let Ok(rows) = client.query("select * from dish order by id", &[]) {
        for row in &rows {
            let v: i64 = row.get(0);
            println!("row id: {}", v);
        }
        for (i, row) in rows.iter().enumerate() {
            let v: i64 = row.get(0);
            println!("index: {}, row id: {}", i, v);
        }
    }
}


