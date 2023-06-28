// Functional Repository Implementation
pub fn find_user_by_id(conn: &PgConnection, user_id: i32) -> Result<Option<User>, DbError> {
  let query = users::table.filter(users::id.eq(user_id));
  let result = conn.query_one(&query)?;

  // Map the result to your domain model (User) and return
  let user = map_to_user(&result);
  Ok(user)
}

pub fn create_user(conn: &Client, user: User) -> Result<User, DbError> {
  let values = diesel::insert_into(users::table)
      .values(&user)
      .get_result(&conn)?;

  // Map the result to your domain model (User) and return
  let created_user = map_to_user(&values);
  Ok(created_user)
}

// Helper function to map database results to your domain model
fn map_to_user(row: &PgRow) -> User {
  // Map the row fields to User struct and return
  // ...
}