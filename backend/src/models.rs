use chrono::{TimeZone, Utc};
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct User {
    pub name: String,
    pub password_hash: String,
}

#[derive(Serialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub author: String,
    pub tags: Vec<String>,
    pub time_required: String,
    pub summary: String,
    pub description: String,
    pub image_key: String,
    pub ingredients: Vec<Ingredient>,
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub amount: String,
    pub typ: String,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub author: String,
    pub comment: String,

    #[serde(with = "ts_milliseconds")]
    pub posted: chrono::DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Session {
    pub session_token: String,
    pub username: String,
}

pub async fn get_user(pool: &SqlitePool, name: &str) -> anyhow::Result<Option<User>> {
    let result = sqlx::query("SELECT * FROM users WHERE name = ?")
        .bind(name)
        .map(|row| {
            User {
                name: row.get("name"),
                password_hash: row.get("password_hash"),
            }
        })
        .fetch_optional(pool).await?;

    Ok(result)
}

pub async fn create_user(pool: &SqlitePool, user: &User) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO users (name, password_hash) VALUES (?, ?)")
        .bind(&user.name)
        .bind(&user.password_hash)
        .execute(pool).await?;

    Ok(())
}

pub async fn insert_recipe(pool: &SqlitePool, recipe: &Recipe) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO recipes (id, name, author, timeRequired, summary, description, imageLocation) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&recipe.id)
        .bind(&recipe.name)
        .bind(&recipe.author)
        .bind(&recipe.time_required)
        .bind(&recipe.summary)
        .bind(&recipe.description)
        .bind(&recipe.image_key)
        .execute(pool).await?;

    for ingredient in &recipe.ingredients {
        sqlx::query("INSERT INTO ingredients (amount, type, recipeId) VALUES (?, ?, ?)")
            .bind(&ingredient.amount)
            .bind(&ingredient.typ)
            .bind(&recipe.id)
            .execute(pool).await?;
    }

    for tag in &recipe.tags {
        sqlx::query("INSERT INTO tags (tag, recipeId) VALUES (?, ?)")
            .bind(tag)
            .bind(&recipe.id)
            .execute(pool).await?;
    }

    for comment in &recipe.comments {
        sqlx::query("INSERT INTO comments (author, comment, posted, recipeId) VALUES (?, ?, ?, ?, ?)")
            .bind(&comment.author)
            .bind(&comment.comment)
            .bind(comment.posted.timestamp_millis())
            .bind(&recipe.id)
            .execute(pool).await?;
    }

    Ok(())
}

pub async fn get_all_recipes(pool: &SqlitePool) -> anyhow::Result<Vec<Recipe>> {
    let recipe_rows = sqlx::query("SELECT * FROM recipes")
        .fetch_all(pool).await?;

    let mut result = vec![];

    for row in recipe_rows {
        let id: String = row.get("id");
        let name: String = row.get("name");
        let author: String = row.get("author");
        let time_required: String = row.get("timeRequired");
        let summary: String = row.get("summary");
        let description: String = row.get("description");
        let image_location: String = row.get("imageLocation");

        let comments = sqlx::query("SELECT * FROM comments WHERE recipeId = ?")
            .bind(&id)
            .map(|r| Comment {
                author: r.get("author"),
                comment: r.get("comment"),
                posted: Utc.timestamp_millis_opt(r.get("posted")).unwrap(),
            })
            .fetch_all(pool).await?;

        let ingredients = sqlx::query("SELECT * FROM ingredients WHERE recipeId = ?")
            .bind(&id)
            .map(|r| Ingredient {
                amount: r.get("amount"),
                typ: r.get("type"),
            })
            .fetch_all(pool).await?;

        let tags: Vec<String> = sqlx::query("SELECT * FROM tags WHERE recipeId = ?")
            .bind(&id)
            .map(|r| r.get("tag"))
            .fetch_all(pool).await?;

        result.push(Recipe {
            id,
            name,
            author,
            tags,
            time_required,
            summary,
            description,
            image_key: image_location,
            ingredients,
            comments,
        });
    }

    Ok(result)
}

pub async fn add_session(pool: &SqlitePool, user: &User) -> anyhow::Result<Session> {
    let gen_uuid = Uuid::new_v4();

    let session = Session {
        session_token: gen_uuid.to_string(),
        username: user.name.clone(),
    };

    sqlx::query("INSERT INTO sessions (session_token, username) VALUES (?, ?)")
        .bind(&session.session_token)
        .bind(&session.username)
        .execute(pool).await?;

    Ok(session)
}

pub async fn get_user_by_session(pool: &SqlitePool, session_token: impl AsRef<str>) -> anyhow::Result<Option<User>> {
    let Some(username): Option<String> = sqlx::query("SELECT * FROM sessions WHERE session_token = ?")
        .bind(session_token.as_ref())
        .map(|r| r.get("username"))
        .fetch_optional(pool).await? else { return Ok(None); };

    get_user(pool, &username).await
}

pub async fn delete_session(pool: &SqlitePool, session_token: impl AsRef<str>) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM sessions WHERE session_token = ?")
        .bind(session_token.as_ref())
        .execute(pool).await?;

    Ok(())
}

pub async fn bookmark(pool: &SqlitePool, user: String, recipe: String, bookmark: bool) -> anyhow::Result<()> {
    let is_bookmarked = sqlx::query("SELECT * FROM bookmarks WHERE user = ? AND recipeId = ?")
        .bind(&user)
        .bind(&recipe)
        .fetch_optional(pool).await?.is_some();

    if is_bookmarked && !bookmark {
        sqlx::query("DELETE FROM bookmarks WHERE user = ? AND recipeId = ?")
            .bind(&user)
            .bind(&recipe)
            .execute(pool).await?;
    } else if !is_bookmarked && bookmark {
        sqlx::query("INSERT INTO bookmarks (user, recipeId) VALUES (?, ?)")
            .bind(&user)
            .bind(&recipe)
            .execute(pool).await?;
    }


    Ok(())
}

pub async fn get_all_bookmarks(pool: &SqlitePool, user: String) -> anyhow::Result<Vec<String>> {
    let bookmarks: Vec<String> = sqlx::query("SELECT * FROM bookmarks WHERE user = ?")
        .bind(user)
        .map(|r| r.get("recipeId"))
        .fetch_all(pool).await?;

    Ok(bookmarks)
}


pub async fn add_comment(pool: &SqlitePool, recipe_id: String, comment: String, author: String, timestamp: i64) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO comments (author, comment, posted, recipeId) VALUES (?, ?, ?, ?)")
        .bind(author)
        .bind(comment)
        .bind(timestamp)
        .bind(recipe_id)
        .execute(pool).await?;

    Ok(())
}
