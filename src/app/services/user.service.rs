use tokio_postgres::Client;

pub fn create_user(user: &User, conn: &mut Client) {
  let query = "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *";
  let stmt = conn.prepare(query).await?;
  let rows = conn.query(&stmt, &[&user.name, &user.email]).await?;
  let row = rows.get(0);
  let user: User = row.try_into()?;
  
  Ok(user)
}

pub fn get_user() {
  let query  = "SELECT * FROM users WHERE";
}

pub fn get_user_by_id(id: &str, conn: &mut Client) {

}

pub fn update_user(id: &str, conn: &mut Client) {
  
}

pub fn delete_user(id: &str, conn: &mut Client) {
  
}