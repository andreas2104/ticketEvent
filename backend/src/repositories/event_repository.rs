use crate::db::DbPool;
use crate::domain::event::{Event, CreateEvent};
use sqlx::{self};

pub struct EventRepository {
    pool: DbPool,
}

impl EventRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    pub async fn create(&self, event: CreateEvent) -> Result<i64, sqlx::Error> {
        let result = sqlx::query("INSERT INTO events (name, date, location, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)")
            .bind(&event.name)
            .bind(&event.date)
            .bind(&event.location)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn find_all(&self) -> Result<Vec<Event>, sqlx::Error> {
        let events = sqlx::query_as::<_, Event>("SELECT * FROM events")
            .fetch_all(&self.pool)
            .await?;

        Ok(events)
    }
}
