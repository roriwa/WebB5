use chrono::{TimeZone, Utc};
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

pub struct User {
    pub name: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub author: String,
    pub tags: Vec<String>,
    pub stars: i8,
    pub time_required: String,
    pub summary: String,
    pub description: String,
    pub image_url: String,
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
    sqlx::query("INSERT INTO recipes (id, name, author, stars, timeRequired, summary, description, imageLocation) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&recipe.id)
        .bind(&recipe.name)
        .bind(&recipe.author)
        .bind(&recipe.stars)
        .bind(&recipe.time_required)
        .bind(&recipe.summary)
        .bind(&recipe.description)
        .bind(&recipe.image_url)
        .execute(pool).await?;

    for ingredient in &recipe.ingredients {
        sqlx::query("INSERT INTO ingredients (amount, type, recipeId) VALUES (?, ?, ?")
            .bind(&ingredient.amount)
            .bind(&ingredient.typ)
            .bind(&recipe.id)
            .execute(pool).await?;
    }

    for tag in &recipe.tags {
        sqlx::query("INSERT INTO tags (tag, recipeId) VALUES (?, ?)")
            .bind(&tag)
            .bind(&recipe.id)
            .execute(pool).await?;
    }

    for comment in &recipe.comments {
        sqlx::query("INSERT INTO comments (author, comment, posted, recipeId) VALUES (?, ?, ?, ?, ?)")
            .bind(&comment.author)
            .bind(&comment.comment)
            .bind(&comment.posted.timestamp_millis())
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
        let name: String = row.get("stars");
        let author: String = row.get("author");
        let stars: i8 = row.get("stars");
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
            stars,
            time_required,
            summary,
            description,
            image_url: image_location,
            ingredients,
            comments,
        });
    }

    Ok(result)
}
