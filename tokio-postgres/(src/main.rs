use tokio_postgres::{NoTls, Error};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Load environment variables (e.g., from a .env file)
    // This is where the secret retrieved during attestation would be used.
    dotenv().ok(); 

    // 2. Construct the connection string from secure environment variables
    let host = env::var("PG_HOST").unwrap_or_else(|_| "localhost".to_string());
    let user = env::var("PG_USER").expect("PG_USER must be set");
    let password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
    let dbname = env::var("PG_DBNAME").expect("PG_DBNAME must be set");

    let connection_string = format!(
        "host={} user={} password={} dbname={}", 
        host, user, password, dbname
    );

    println!("Attempting to connect to PostgreSQL at host: {}", host);

    // 3. Establish the connection
    let (client, connection) = 
        tokio_postgres::connect(&connection_string, NoTls).await?;

    // The connection object performs the actual I/O, so it needs to be run in the background.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    println!("✅ Database connection established successfully!");

    // 4. Test the connection by running a simple query
    let rows = client
        .query("SELECT $1::TEXT", &[&"Hello, DB Connection!"])
        .await?;

    let value: &str = rows[0].get(0);
    println!("Database Test Query Result: {}", value);
    
    // 5. Example: Querying data
    let test_query = "SELECT COUNT(*) FROM pg_database WHERE datname = $1";
    let count_rows = client.query(test_query, &[&dbname]).await?;
    let db_exists: i64 = count_rows[0].get(0);

    if db_exists > 0 {
        println!("Database '{}' successfully queried (It exists).", dbname);
    } else {
        println!("Database '{}' could not be found.", dbname);
    }


    Ok(())
      }
