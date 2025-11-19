use anyhow::Result;
use askama::Template;
use axum::{
    Router,
    extract::{Form, State, Query},
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
};
use std::collections::HashMap;
use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Client, Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Movie {
    pub id: String,
    pub title: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    movies: Vec<Movie>,
    message: String,
}

#[derive(Deserialize)]
struct AddMovieForm {
    title: String,
}

#[derive(Deserialize)]
struct DeleteMovieForm {
    title: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool,
}

pub async fn get_all_movies(client: &Client) -> Result<Vec<Movie>> {
    let stmt = client
        .prepare("SELECT id::text, title FROM movie ORDER BY title")
        .await?;
    let rows = client.query(&stmt, &[]).await?;

    let movies = rows
        .iter()
        .map(|row| Movie {
            id: row.get(0),
            title: row.get(1),
        })
        .collect();

    Ok(movies)
}

async fn index(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, StatusCode> {
    let client = state
        .db_pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let movies = get_all_movies(&client)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let template = IndexTemplate {
        movies,
        message: params.get("message").cloned().unwrap_or_default(),
    };
    let html = template
        .render()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(html))
}

async fn insert_movie(
    State(state): State<AppState>,
    Form(form): Form<AddMovieForm>,
) -> Result<Redirect, StatusCode> {
    let client = state
        .db_pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Check if movie already exists
    let check_stmt = client
        .prepare("SELECT title FROM movie WHERE title = $1")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let existing = client.query_one(&check_stmt, &[&form.title]).await;
    
    if existing.is_ok() {
        Ok(Redirect::to(&format!("/?message='{}' is already in your watchlist", form.title)))
    } else {
        let stmt = client
            .prepare("INSERT INTO movie (title) VALUES ($1)")
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        client
            .execute(&stmt, &[&form.title])
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Redirect::to("/"))
    }
}

async fn delete_movie(
    State(state): State<AppState>,
    Form(form): Form<DeleteMovieForm>,
) -> Result<Redirect, StatusCode> {
    let client = state
        .db_pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let stmt = client
        .prepare("DELETE FROM movie WHERE title = $1")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    client
        .execute(&stmt, &[&form.title])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut cfg = Config::new();
    cfg.dbname = Some("stardust".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);
    cfg.user = Some("postgres".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    let state = AppState { db_pool: pool };

    let app = Router::new()
        .route("/", get(index))
        .route("/add", post(insert_movie))
        .route("/delete", post(delete_movie))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Stardust server running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
