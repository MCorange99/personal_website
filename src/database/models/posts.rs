use serde_json::Value;
use uuid::Uuid;
use sqlx::Row;
use crate::database::Database;
use futures::TryStreamExt;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub descr: String,
    pub img_url: String,
    pub origin_url: String,
    pub original_request: String,
    pub posted_on: i64,
}

#[allow(dead_code)]
impl Post {
    pub async fn create_new(db: &mut Database, title: String, descr: String, img_url: String, origin_url: String, orignal_request: Value) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let posted_on = chrono::Utc::now().timestamp_millis();


        sqlx::query(r#"
            INSERT INTO posts ( id, title, descr, img_url, origin_url, original_request, posted_on )
            VALUES ( $1, $2, $3, $4, 0 )
            RETURNING id
        "#)
            .bind(id)
            .bind(title)
            .bind(descr)
            .bind(img_url)
            .bind(origin_url)
            .bind(orignal_request)
            .bind(posted_on)
            .execute(db.connection())
            .await?;

        log::debug!("Created post with id '{id}'");

        Ok(id)
    }

    pub async fn get_by_id(db: &mut Database, id: Uuid) -> anyhow::Result<Option<Self>>{
        let mut rows = sqlx::query("SELECT * FROM posts WHERE id = $1")
            .bind(id)
            .fetch(db.connection());

        while let Some(row) = rows.try_next().await? {
            return Ok(Some(Self {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                descr: row.try_get("descr")?,
                img_url: row.try_get("img_url")?,
                origin_url: row.try_get("origin_url")?,
                original_request: row.try_get("original_request")?,
                posted_on: row.try_get("posted_on")?,
            }));
        }

        Ok(None)
    }

    pub async fn get_last_n(db: &mut Database, n: usize) -> anyhow::Result<Vec<Post>>{
        let mut rows = sqlx::query("SELECT * FROM posts")
            .fetch(db.connection());

        let mut posts = Vec::new();

        while let Some(row) = rows.try_next().await? {
            let post = Self {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                descr: row.try_get("descr")?,
                img_url: row.try_get("img_url")?,
                origin_url: row.try_get("origin_url")?,
                original_request: row.try_get("original_request")?,
                posted_on: row.try_get("posted_on")?,
            };
            posts.push(post);
        }

        posts.sort_unstable_by(|a, b| {
            b.posted_on.cmp(&a.posted_on)
        });

        let posts = if posts.len() < n {
            posts.to_vec()
        } else {
            posts[..n].to_vec()
        };


        Ok(posts)
    }

    pub async fn save(&self, db: &mut Database) -> anyhow::Result<()> {
        let _ = sqlx::query(r#"
                UPDATE posts
                SET id = $1
                SET title = $2
                SET descr = $3
                SET img_url = $4
                SET origin_url = $5
                SET original_request = $6
                SET posted_on = $7

            "#)
            .bind(&self.id)
            .bind(&self.title)
            .bind(&self.descr)
            .bind(&self.img_url)
            .bind(&self.origin_url)
            .bind(&self.original_request)
            .bind(&self.posted_on)
            .execute(db.connection()).await?;

        Ok(())
    }

    pub async fn remove(&self, db: &mut Database) -> anyhow::Result<()> {
        sqlx::query(r#"
            DELETE FROM posts
            WHERE id = $1
        "#)
            .bind(self.id)
            .execute(db.connection())
            .await?;

        log::debug!("Deleted post with id '{}'", self.id);

        Ok(())
    }
}