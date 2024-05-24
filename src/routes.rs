use rocket::serde::json::Json;
use rocket::State;
use crate::models::Note;
use sqlx::PgPool;
use crate::errors::ErrorResponse;

#[get("/notes")]
pub async fn get_notes(pool: &State<PgPool>) -> Result<Json<Vec<Note>>, ErrorResponse> {
    let notes = sqlx::query_as::<_, Note>("SELECT id, content FROM notes")
        .fetch_all(pool.inner())
        .await?;

    Ok(Json(notes))
}

#[post("/notes", data = "<note>")]
pub async fn create_note(pool: &State<PgPool>, note: Json<Note>) -> Result<Json<Note>, ErrorResponse> {
    let result = sqlx::query!(
        "INSERT INTO notes (content) VALUES ($1) RETURNING id, content",
        note.content
    )
    .fetch_one(pool.inner())
    .await?;

    Ok(Json(Note {
        id: result.id,
        content: result.content,
    }))
}
