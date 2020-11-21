use crate::database::DbPool;
use crate::models::user::User;
use crate::models::uuid_serializer;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::types::Uuid;
use sqlx::{Done, FromRow};
use time::PrimitiveDateTime;

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub content: String,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Post {
    #[serde(with = "uuid_serializer")]
    pub id: Uuid,
    #[serde(with = "uuid_serializer")]
    pub user_id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

// implementation of Actix Responder for Post struct so we can return Post from action handler
impl Responder for Post {
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

// Implementation for Post struct, functions for read/write/update and delete post from database
impl Post {
    pub async fn find_all(pool: &DbPool) -> Result<Vec<Post>> {
        let posts = sqlx::query_as!(
            Post,
            "
                SELECT id, user_id, title, slug, excerpt, content, created_at, updated_at
                    FROM posts
                ORDER BY created_at
            "
        )
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    pub async fn find_by_id(id: Uuid, pool: &DbPool) -> Result<Post> {
        let post = sqlx::query_as!(
            Post,
            "
                SELECT * FROM posts WHERE id = $1
            ",
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(post)
    }

    pub async fn create(post: PostRequest, pool: &DbPool, logged_user: User) -> Result<Post> {
        let slug = slugify(post.title.clone());
        // take the first 55 words as the excerpt.
        let excerpt: String = post
            .content
            .split(' ')
            .take(55)
            .collect::<Vec<&str>>()
            .join(" ");
        let mut tx = pool.begin().await?;
        let post = sqlx::query_as!(
            Post,
            "INSERT INTO posts (user_id, title, slug, excerpt, content) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            logged_user.id,
            post.title,
            slug,
            excerpt,
            post.content,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(post)
    }

    pub async fn update(id: Uuid, post: PostRequest, pool: &DbPool) -> Result<Post> {
        let mut tx = pool.begin().await.unwrap();
        // we won't update the slug automatically in case others have linked to it
        // maybe in the future we'll have an option to explicitly change the slug
        // take the first 55 words as the excerpt.
        let excerpt: String = post
            .content
            .split(' ')
            .take(55)
            .collect::<Vec<&str>>()
            .join(" ");
        let post = sqlx::query_as!(
            Post,
            "
                UPDATE posts SET title = $1, content = $2, excerpt = $3 WHERE id = $4 RETURNING *
            ",
            post.title,
            post.content,
            excerpt,
            id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await.unwrap();
        Ok(post)
    }

    pub async fn delete(id: Uuid, pool: &DbPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted.rows_affected())
    }
}
