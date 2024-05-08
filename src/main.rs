use scylla::transport::session::Session;
use scylla::SessionBuilder;
use std::error::Error;
extern crate dotenv;

// use scylla_practice::db_ops::data_definition::create::create_keyspace;
// use scylla_practice::db_ops::data_definition::create::create_networks_table;
// use scylla_practice::db_ops::data_definition::create::create_contracts_table;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    println!("Using Scylla URI: {}", uri);

    let session: Session = SessionBuilder::new().known_node(uri).build().await?;

    // create new keyspace if not existed
    // create_keyspace(&session, "keyspace_fn_test_1").await?;

    // create networks table if not existed
    // create_networks_table(&session, "keyspace_fn_test_1").await?;

    // create contracts table if not existed
    // create_contracts_table(&session, "keyspace_fn_test_1").await?;

    Ok(())
}
