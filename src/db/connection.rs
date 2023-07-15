use tokio_postgres::{NoTls, Error};

pub async fn establish_connection() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=rustwarpcrud dbname=warp_crud",
        NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
pub async fn perform_query(client: &tokio_postgres::Client) -> Result<(), Error> {
    let rows = client
        .query("SELECT * FROM mytable", &[])
        .await?;

    for row in rows {
        let id: i32 = row.get("id");
        let name: &str = row.get("name");
        println!("id: {}, name: {}", id, name);
    }

    Ok(())
}
