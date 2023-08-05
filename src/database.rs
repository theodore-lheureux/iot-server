use sqlx::{Pool, Sqlite};

pub mod requests;
pub mod responses;

use responses::UserResponse;

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn create_user(
    pool: &Pool<Sqlite>,
    username: &str,
    password: &str,
) -> DBResult<UserResponse> {
    let user = sqlx::query_as::<_, UserResponse>(
        r#"
        INSERT INTO users (username, password)
        VALUES (?, ?)
        RETURNING id, username, password
        "#,
    )
    .bind(username)
    .bind(password)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user(pool: &Pool<Sqlite>, username: &str) -> DBResult<UserResponse> {
    let user = sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, username, password
        FROM users
        WHERE username = ?
        "#,
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_username(pool: &Pool<Sqlite>, username: &str) -> DBResult<UserResponse> {
    let user = sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, username, password
        FROM users
        WHERE username = ?
        "#,
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    Ok(user)
}
