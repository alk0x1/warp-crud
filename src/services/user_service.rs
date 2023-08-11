pub mod user_service {
  use tokio_postgres::Client;
  use tokio_postgres::types::ToSql;
  use crate::entities::user_entity::User;
  use crate::db;


  pub async fn create_user(user: &User, conn: &mut Client) {
    let client =  db::establish_connection();
    
    let query = "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *";
    let stmt = conn.prepare(query).await?;
    let rows = conn.query(&stmt, &[&user.name, &user.email]).await?;
    let row = rows.get(0);
    let user: User = row.try_into()?;
    
    Ok(user)
  }
  
  pub async fn get_users(conn: &mut Client) {
    let query = "SELECT * FROM users";
    let rows = conn.query(query, &[]).await?;
    let users: Vec<User> = rows.iter().map(|row| row.try_into().unwrap()).collect();
  
    Ok(users)
  }
  
  pub async fn get_user_by_id(id: &str, conn: &mut Client) {
    let query = "SELECT * FROM users WHERE id = $1";
    let stmt = conn.prepare(query).await?;
    let rows = conn.query(&stmt, &[&id]).await?;
  
    if rows.is_empty() {
      Ok(None)
    } else {
      let row = rows.get(0);
      let user: User = row.try_into()?;
      Ok(Some(user))
    }
  }
  
  pub async fn update_user(user: &User, id: &str, conn: &mut Client) {
    let query = "UPDATE users SET name = $1, email = $2 WHERE id = $3";
    conn.execute(query, &[&user.name, &user.email, &id]).await?;
    
    Ok(())
  }
  
  pub async fn delete_user(id: &str, conn: &mut Client) {
    let query = "DELETE FROM users WHERE id = $1";
    conn.execute(query, &[&id]).await?;
  
    Ok(())
  }

}