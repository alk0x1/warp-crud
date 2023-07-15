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


use crate::app::models::{User, Place};

pub trait Repository<T> {
  fn create(&self, entity: &T) -> Result<(), Error>;
  fn update(&self, entity: &T) -> Result<(), Error>;
  fn delete(&self, id: &str) -> Result<(), Error>;
  fn find_by_id(&self, id: &str) -> Result<Option<T>, Error>;
  fn find_all(&self) -> Result<Vec<T>, Error>;
}

pub struct UserRepository {
    // Implementation details for the user repository
}

impl Repository<User> for UserRepository {
  fn create(&self, user: &User) -> Result<(), Error> {
      // Implementation for creating a user
  }

  fn update(&self, user: &User) -> Result<(), Error> {
      // Implementation for updating a user
  }

  fn delete(&self, id: &str) -> Result<(), Error> {
      // Implementation for deleting a user
  }

  fn find_by_id(&self, id: &str) -> Result<Option<User>, Error> {
      // Implementation for finding a user by ID
  }

  fn find_all(&self) -> Result<Vec<User>, Error> {
      // Implementation for finding all users
  }
}

pub struct PlaceRepository {
  // Implementation details for the place repository
}

impl Repository<Place> for PlaceRepository {
  fn create(&self, place: &Place) -> Result<(), Error> {
      // Implementation for creating a place
  }

  fn update(&self, place: &Place) -> Result<(), Error> {
      // Implementation for updating a place
  }

  fn delete(&self, id: &str) -> Result<(), Error> {
      // Implementation for deleting a place
  }

  fn find_by_id(&self, id: &str) -> Result<Option<Place>, Error> {
      // Implementation for finding a place by ID
  }

  fn find_all(&self) -> Result<Vec<Place>, Error> {
      // Implementation for finding all places
  }
}