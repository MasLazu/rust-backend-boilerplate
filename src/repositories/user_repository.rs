use super::error::{Error, Result};
use crate::models::user::User;
use sqlx::PgPool;

pub async fn get_user_by_id(db: &PgPool, id: i32) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, role, password FROM users WHERE id = $1",
        id
    )
    .fetch_one(db)
    .await;

    match user {
        Ok(u) => Ok(u),
        Err(sqlx::Error::RowNotFound) => Err(Error::RowNotFound),
        Err(_) => Err(Error::DatabaseError),
    }
}

pub async fn get_all_users(db: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, name, role, password FROM users")
        .fetch_all(db)
        .await;

    match users {
        Ok(u) => Ok(u),
        Err(sqlx::Error::RowNotFound) => {
            let u: Vec<User> = Vec::new();
            Ok(u)
        }
        Err(_) => Err(Error::DatabaseError),
    }
}

pub async fn insert_user(db: &PgPool, user: User) -> Result<User> {
    let user = sqlx::query_as!(
            User,
            "INSERT INTO users (name, role, password) VALUES ($1, $2, $3) RETURNING id, name, role, password",
            user.name,
            user.role.to_string(),
            user.password
        )
        .fetch_one(db)
        .await;

    match user {
        Ok(u) => Ok(u),
        Err(sqlx::Error::Database(e)) => {
            if e.is_check_violation() {
                Err(Error::UniqueConstraintViolation)
            } else {
                Err(Error::DatabaseError)
            }
        }
        Err(_) => Err(Error::DatabaseError),
    }
}

pub async fn update_user(db: &PgPool, user: User) -> Result<User> {
    let user = sqlx::query_as!(
            User,
            "UPDATE users SET name = $2, role = $3, password = $4 WHERE id = $1 RETURNING id, name, role, password",
            user.id,
            user.name,
            user.role.to_string(),
            user.password
        )
        .fetch_one(db)
        .await;

    match user {
        Ok(u) => Ok(u),
        Err(sqlx::Error::Database(e)) => {
            if e.is_check_violation() {
                Err(Error::UniqueConstraintViolation)
            } else {
                Err(Error::DatabaseError)
            }
        }
        Err(_) => Err(Error::DatabaseError),
    }
}

pub async fn delete_user(db: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(|_| Error::DatabaseError)?;

    Ok(())
}
