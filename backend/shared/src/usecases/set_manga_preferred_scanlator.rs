use crate::{
    database::Database,
    model::{MangaId, MangaState},
};
use anyhow::Result;

pub async fn set_manga_preferred_scanlator(
    db: &Database,
    manga_id: MangaId,
    preferred_scanlator: Option<String>,
) -> Result<()> {
    let manga_state = db.find_manga_state(&manga_id).await.unwrap_or_default();

    let updated_manga_state = MangaState {
        preferred_scanlator,
        ..manga_state
    };

    db.upsert_manga_state(&manga_id, updated_manga_state).await;

    Ok(())
}
