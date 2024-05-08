use scylla::transport::session::Session;
use std::error::Error;

pub async fn create_keyspace(session: &Session, keyspace_name: &str) -> Result<(), Box<dyn Error>> {
    let query_param = format!(
        "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{ \
            'class': 'NetworkTopologyStrategy', \
            'replication_factor': 1 \
        }}",
        keyspace_name
    );
    // Create a new keyspace
    session.query(query_param, &[]).await?;
    if let Err(e) = Ok(()) {
        println!("Failed to create keyspace: {:?}", e);
        return e;
    } else {
        println!("created keyspace named {keyspace_name}");
        Ok(())
    }
}

pub async fn create_networks_table(
    session: &Session,
    keyspace_name: &str,
) -> Result<(), Box<dyn Error>> {
    // Use the keyspace
    let keyspace_query = format!("USE {}", keyspace_name);
    session.query(keyspace_query, &[]).await?;
    // create the table withen the keyspace
    session
        .query(
            "CREATE TABLE IF NOT EXISTS networks (
            id UUID PRIMARY KEY,
            name text,
            chain_id text,
            unit text,
            env text,
            explorer text,
            unbonding_period_days int,
            created_at timestamp,
            updated_at timestamp
        )",
            &[],
        )
        .await?;

    if let Err(e) = Ok(()) {
        println!("Failed to create networks table: {:?}", e);
        return e;
    } else {
        println!("created networks table at keyspace {keyspace_name}");
        Ok(())
    }
}

pub async fn create_contracts_table(
    session: &Session,
    keyspace_name: &str,
) -> Result<(), Box<dyn Error>> {
    // Use the keyspace
    let keyspace_query = format!("USE {}", keyspace_name);
    session.query(keyspace_query, &[]).await?;
    // create the table withen the keyspace
    session
        .query(
            "CREATE TABLE IF NOT EXISTS contracts (
            id UUID PRIMARY KEY,
            creator text,
            contract_name text,
            contract_address text,
            code_id int,
            admin text,
            metadata text,
            network_id UUID ,
            created_at timestamp,
            updated_at timestamp
        )",
            &[],
        )
        .await?;

    if let Err(e) = Ok(()) {
        println!("Failed to create contracts table: {:?}", e);
        return e;
    } else {
        println!("created contracts table at keyspace {keyspace_name}");
        Ok(())
    }
}
