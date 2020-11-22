use crate::database::DbPool;
use crate::models::uuid_serializer;
use actix_identity::Identity;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::types::Uuid;
use sqlx::{Done, FromRow};
use time::PrimitiveDateTime;

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub github_id: Option<i64>,
    pub github_token: Option<String>,
    pub role: Role,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(sqlx::Type, PartialEq)]
#[repr(i16)]
pub enum Role {
    /// access to everything
    SuperAdmin = 1,
    Admin = 2,
    /// Someone who can publish and edit posts including the posts of other users.
    Editor = 3,
    /// Someone who can publish and edit their own posts.
    Author = 4,
    /// Someone who can write and edit their own posts but cannot publish them.
    Contributor = 5,
    /// Someone who can only manage their profile and comment.
    Subscriber = 6,
    /// Someone who does not have an account
    Guest = 7,
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let slug = match self {
            Role::SuperAdmin => "super-admin",
            Role::Admin => "admin",
            Role::Editor => "editor",
            Role::Author => "author",
            Role::Contributor => "contributor",
            Role::Subscriber => "subscriber",
            Role::Guest => "guest",
        };
        serializer.serialize_str(slug)
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let slug = String::deserialize(deserializer)?;
        let role = match slug.as_str() {
            "super-admin" => Role::SuperAdmin,
            "admin" => Role::Admin,
            "editor" => Role::Editor,
            "author" => Role::Author,
            "contributor" => Role::Contributor,
            "subscriber" => Role::Subscriber,
            _ => Role::Guest,
        };
        Ok(role)
    }
}

// this struct will be used to represent database record
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    #[serde(with = "uuid_serializer")]
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub github_id: Option<i64>,
    #[serde(skip_serializing)]
    pub github_token: Option<String>,
    pub role: Role,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub trait ToUser {
    fn user(&self) -> Option<User>;
}

impl ToUser for Identity {
    fn user(&self) -> Option<User> {
        match self.identity() {
            None => None,
            Some(identity) => match serde_json::from_str::<User>(&*identity) {
                Ok(u) => Some(u),
                Err(_) => None,
            },
        }
    }
}

// implementation of Actix Responder for User struct so we can return User from action handler
impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

// Implementation for User struct, functions for read/write/update and delete user from database
impl User {
    pub async fn find_all(pool: &DbPool) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password, name, avatar_url,
                gravatar_id, github_id, github_token, role as "role: Role",
                created_at, updated_at
                FROM users
                ORDER BY created_at
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_id(id: Uuid, pool: &DbPool) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password, name, avatar_url,
                gravatar_id, github_id, github_token, role as "role: Role",
                created_at, updated_at
                FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_github_id(id: i64, pool: &DbPool) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password, name, avatar_url,
                gravatar_id, github_id, github_token, role as "role: Role",
                created_at, updated_at
                FROM users WHERE github_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn create(user: UserRequest, pool: &DbPool) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO users (username, email, password, name, avatar_url, gravatar_id, github_id, github_token, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, username, email, password, name, avatar_url,
                gravatar_id, github_id, github_token, role as "role: Role",
                created_at, updated_at
            "#,
            user.username,
            user.email,
            user.password,
            user.name,
            user.avatar_url,
            user.gravatar_id,
            user.github_id,
            user.github_token,
            user.role as i16,
            user.created_at,
            user.updated_at,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update(id: Uuid, user: UserRequest, pool: &DbPool) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                UPDATE users SET username = $1, email = $2, password = $3, name = $4, avatar_url = $5,
                gravatar_id = $6, github_token = $7, role = $8, updated_at = now()
                WHERE id = $9
                RETURNING id, username, email, password, name, avatar_url,
                gravatar_id, github_id, github_token, role as "role: Role",
                created_at, updated_at
            "#,
            user.username,
            user.email,
            user.password,
            user.name,
            user.avatar_url,
            user.gravatar_id,
            user.github_token,
            user.role as i16,
            id,
        )
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn delete(id: Uuid, pool: &DbPool) -> Result<u64> {
        let deleted = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(deleted.rows_affected())
    }
}
