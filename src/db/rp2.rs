// data_access.rs

use tokio_postgres::{NoTls, Error};

pub struct UserRepository {
    // Database connection pool or client
    client: tokio_postgres::Client,
}

impl UserRepository {
    pub async fn new() -> Result<Self, Error> {
        // Establish a connection to the database
        let (client, connection) = tokio_postgres::connect("host=localhost user=myuser dbname=mydb", NoTls).await?;
        
        // Spawn a task to process the connection in the background
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }

    pub async fn find_user_by_id(&self, user_id: i32) -> Result<User, Error> {
        // Perform a query using the client
        let rows = self.client.query("SELECT * FROM users WHERE id = $1", &[&user_id]).await?;

        // Process the query results and map them to a domain entity (User)
        if let Some(row) = rows.get(0) {
            let id: i32 = row.get("id");
            let name: String = row.get("name");
            let email: Option<String> = row.get("email");

            let user = User { id, name, email };

            Ok(user)
        } else {
            // User not found
            Err(Error::from("User not found"))
        }
    }

    // Other data access methods...
}