use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres::{Client, Error};
use postgres_openssl::MakeTlsConnector;
use dotenv::dotenv;

#[derive(Debug, serde::Deserialize)]
struct Record {
    first_name: String,
    last_name: String,
    age: i32,
    sex: String
}

pub fn query_db() -> Result<(), Error> {
    dotenv().ok();

    let connection = std::env::var("POSTGRES_CONNECTION").expect("POSTGREST_CONNECTION must be set.");
    let cert_loc = std::env::var("CERT_LOCATION").expect("CERT_LOCATION must be set.");

    let mut builder = SslConnector::builder(SslMethod::tls()).expect("unable to create sslconnector builder");
    builder.set_ca_file(&cert_loc).expect("unable to load ca.cert");
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(&connection, connector)?;

    for row in client.query("SELECT * FROM people", &[])? {
        let person = Record {
            first_name: row.get(0),
            last_name: row.get(1),
            age: row.get(2),
            sex: row.get(3),
        };
        println!("First Name: {}, Last Name: {}, Age: {}, Sex: {}", person.first_name, person.last_name, person.age, person.sex);
    }

    Ok(())
}