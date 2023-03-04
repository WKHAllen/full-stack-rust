use crate::{new_id, DB};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quote {
    pub id: String,
    pub quote: String,
}

impl Quote {
    pub async fn create(db: &DB, quote: &str) -> Self {
        let id = new_id();

        sqlx::query!("INSERT INTO quotes VALUES (?, ?);", id, quote)
            .execute(&db.pool)
            .await
            .unwrap();

        Self::get(&db, &id).await.unwrap()
    }

    pub async fn get(db: &DB, id: &str) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM quotes WHERE id = ?;", id)
            .fetch_optional(&db.pool)
            .await
            .unwrap()
    }

    pub async fn list(db: &DB) -> Vec<Quote> {
        sqlx::query_as!(Self, "SELECT * FROM quotes;")
            .fetch_all(&db.pool)
            .await
            .unwrap()
    }

    pub async fn set(&mut self, db: &DB, new_quote: &str) {
        self.quote = new_quote.to_owned();

        sqlx::query!(
            "UPDATE quotes SET quote = ? WHERE id = ?;",
            self.quote,
            self.id
        )
        .execute(&db.pool)
        .await
        .unwrap();
    }

    pub async fn delete(self, db: &DB) {
        sqlx::query!("DELETE FROM quotes WHERE id = ?;", self.id)
            .execute(&db.pool)
            .await
            .unwrap();
    }
}
