pub mod place_service {
  use tokio_postgres::Client;
  use tokio_postgres::types::ToSql;
  use crate::entities::place_entity::Place;
  
  pub async fn create_place(place: &Place, conn: &mut Client) {
    let query = "INSERT INTO places (name, email) VALUES ($1, $2) RETURNING *";
    let stmt = conn.prepare(query).await?;
    let rows = conn.query(&stmt, &[&place.name, &place.email]).await?;
    let row = rows.get(0);
    let place: Place = row.try_into()?;
    
    Ok(place)
  }
  
  pub async fn get_places(conn: &mut Client) {
    let query = "SELECT * FROM places";
    let rows = conn.query(query, &[]).await?;
    let places: Vec<Place> = rows.iter().map(|row| row.try_into().unwrap()).collect();
  
    Ok(places)
  }
  
  pub async fn get_place_by_id(id: &str, conn: &mut Client) {
    let query = "SELECT * FROM places WHERE id = $1";
    let stmt = conn.prepare(query).await?;
    let rows = conn.query(&stmt, &[&id]).await?;
  
    if rows.is_empty() {
      Ok(None)
    } else {
      let row = rows.get(0);
      let place: Place = row.try_into()?;
      Ok(Some(place))
    }
  }
  
  pub async fn update_place(place: &Place, id: &str, conn: &mut Client) {
    let query = "UPDATE places SET name = $1, email = $2 WHERE id = $3";
    conn.execute(query, &[&place.name, &place.email, &id]).await?;
    
    Ok(())
  }
  
  pub async fn delete_place(id: &str, conn: &mut Client) {
    let query = "DELETE FROM places WHERE id = $1";
    conn.execute(query, &[&id]).await?;
  
    Ok(())
  }
}
