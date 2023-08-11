use crate::services::user_service::user_service;
use crate::entities::user_entity::User;

pub async fn create_user(user: &User) {
  user_service::create_user(user);
}
