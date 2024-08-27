// Import necessary modules from the standard library and external crates
use axum::{extract::{Path, State}, routing::{delete, get, patch, post}, Json, Router};
use serde::{Deserialize, Serialize};
use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;
use dotenv::dotenv;
mod text_file;

/// Main function to set up and run the server
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    // Get the database URL from environment variables
    let url = std::env::var("DATABASE_URL")?;

    // Connect to the SQLite database
    let pool = SqlitePool::connect(&url).await.unwrap();

    // Set up the router with routes and middleware
    let app = Router::new()
        .route("/", get(list))
        .route("/note/:id", get(get_note))
        .route("/create", post(add_note))
        .route("/delete/:id", delete(delete_note))
        .route("/update/:id", patch(update_note))
        .route("/convert/:id", get(convert_to_file))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    // Bind the server to the specified address and port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Start the server
    Ok(axum::serve(listener, app).await.unwrap())
}

/// Represents a note item with an ID and content
#[derive(Debug, Serialize, Deserialize)]
struct NoteItem {
    pub id: i64,
    pub content: String,
}

/// Represents the data needed to upload a new note
#[derive(Serialize, Deserialize)]
struct NoteItemUpload {
    content: String
}

/// Represents a note item ID
#[derive(Serialize, Deserialize)]
struct NoteItemId {
    id: i64
}

use std::convert::Infallible;

/// Handler to list all notes
async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<NoteItem>>, Infallible> {
    // Fetch all notes from the database
    let notes = sqlx::query_as!(NoteItem, "SELECT id, content FROM notes ORDER BY id")
        .fetch_all(&pool)
        .await.unwrap();

    println!("Listing all notes: {:?}", notes);
    println!("===========================================");
    Ok(Json(notes))
}

/// Handler to add a new note
async fn add_note(State(pool): State<SqlitePool>, Json(note): Json<NoteItemUpload>) -> Json<NoteItem> {
    println!("Adding new note...");
    // Insert the new note into the database
    let nt  = sqlx::query!("INSERT INTO notes (content) VALUES (?)", note.content)
    .execute(&pool)
    .await
    .unwrap()
    .last_insert_rowid();

    println!("Created note with id: {:?}", nt);
    println!("===========================================");

    // Fetch and return the newly created note
    get_note(State(pool), Path(nt)).await.unwrap()
}

/// Handler to delete a note
async fn delete_note(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> String {
    // Delete the note from the database
    let note = sqlx::query!("DELETE FROM notes WHERE id = ?", id)
        .execute(&pool)
        .await
        .unwrap();
    
    println!("Deleted: {:?}", note);
    println!("===========================================");

    "Deleted succefully".to_string()
}

/// Handler to update a note
async fn update_note(State(pool): State<SqlitePool>, Json(note): Json<NoteItem>) -> Json<NoteItem> {
    // Update the note in the database
    sqlx::query!("UPDATE notes SET content = ? WHERE id = ?", note.content, note.id)
        .execute(&pool)
        .await
        .unwrap();

    // Fetch and return the updated note
    let updated_note = get_note(State(pool), Path(note.id)).await.unwrap();

    println!("Updated: {:?}", updated_note);
    println!("===========================================");
    
    updated_note
}

/// Handler to convert a note to a file
async fn convert_to_file(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> String {
    // Get the note from the database
    let note = get_note(State(pool), Path(id)).await.unwrap();
    let note = text_file::NoteItem {
        id: note.id,
        content: note.content.clone()
    };
    
    // Convert the note to a file
    let file = text_file::convert(note);

    println!("File: {:?}", file);
    println!("==========================================");

    "File created".to_string()
}

/// Handler to get a specific note by ID
async fn get_note(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<NoteItem>, String> {
    // Fetch the note from the database
    let note = sqlx::query_as!(NoteItem, "SELECT id, content FROM notes WHERE id = ?", id)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;

    println!("Note gotten: {:?}", note);
    println!("==========================================");
    Ok(Json(note))
}
