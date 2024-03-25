use uuid::Uuid;
use sqlx::Row;
use crate::database::Database;
use futures::TryStreamExt;

#[derive(sqlx::FromRow)]
pub struct Users {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub pw_hash: String,
    pub permissions: i64,
}

#[allow(dead_code)]
impl Users {
    pub async fn create_new(db: &mut Database, email: String, username: String, password: String) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let hash = bcrypt::hash(password, 15)?;

        sqlx::query(r#"
            INSERT INTO users ( id, email, username, pw_hash, permissions )
            VALUES ( $1, $2, $3, $4, 0 )
            RETURNING id
        "#)
            .bind(id)
            .bind(email)
            .bind(username)
            .bind(hash)
            .execute(db.connection())
            .await?;

        log::debug!("Created user with id '{id}'");

        Ok(id)
    }

    pub async fn get_by_id(db: &mut Database, id: Uuid) -> anyhow::Result<Option<Self>>{
        let mut rows = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch(db.connection());

        while let Some(row) = rows.try_next().await? {
            return Ok(Some(Self {
                id: row.try_get("id")?,
                email: row.try_get("email")?,
                username: row.try_get("username")?,
                pw_hash: row.try_get("pw_hash")?,
                permissions: row.try_get("permissions")?,
            }));
        }

        Ok(None)
    }

    pub async fn get_by_email(db: &mut Database, email: String) -> anyhow::Result<Option<Self>>{
        let mut rows = sqlx::query("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch(db.connection());

        while let Some(row) = rows.try_next().await? {
            return Ok(Some(Self {
                id: row.try_get("id")?,
                email: row.try_get("email")?,
                username: row.try_get("username")?,
                pw_hash: row.try_get("pw_hash")?,
                permissions: row.try_get("permissions")?,
            }));
        }

        Ok(None)
    }

    pub async fn get_by_username(db: &mut Database, username: String) -> anyhow::Result<Option<Self>>{
        let mut rows = sqlx::query("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch(db.connection());

        while let Some(row) = rows.try_next().await? {
            return Ok(Some(Self {
                id: row.try_get("id")?,
                email: row.try_get("email")?,
                username: row.try_get("username")?,
                pw_hash: row.try_get("pw_hash")?,
                permissions: row.try_get("permissions")?,
            }));
        }

        Ok(None)
    }

    pub async fn check_login(db: &mut Database, email: String, password: String) -> anyhow::Result<Option<Self>>{
        let Some(user) = Self::get_by_email(db, email).await? else {return Ok(None)};

        if bcrypt::verify(password, &user.pw_hash)? {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, db: &mut Database) -> anyhow::Result<()> {
        let _ = sqlx::query(r#"
                UPDATE users
                SET id = $1
                SET email = $2
                SET username = $3
                SET pw_hash = $4
                SET permissions = $5
            "#)
            .bind(&self.id)
            .bind(&self.email)
            .bind(&self.username)
            .bind(&self.pw_hash)
            .bind(&self.permissions)
            .execute(db.connection()).await?;

        Ok(())
    }

    pub async fn remove_user(&self, db: &mut Database) -> anyhow::Result<()> {
        sqlx::query(r#"
            DELETE FROM users
            WHERE id = $1
        "#)
            .bind(self.id)
            .execute(db.connection())
            .await?;

        log::debug!("Deleted user with id '{}'", self.id);

        Ok(())
    }
}