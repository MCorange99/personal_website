use rand::Rng;
use uuid::Uuid;
use sqlx::Row;
use crate::database::Database;
use futures::TryStreamExt;

use super::Permissions;

#[derive(sqlx::FromRow, Debug)]
pub struct Token {
    pub token: String,
    pub owner_id: Uuid,
    pub permissions: Permissions,
}

#[allow(dead_code)]
impl Token {
    pub async fn create_new(db: &mut Database, owner_id: Uuid, permissions: Permissions) -> anyhow::Result<Self> {

        let token = generate_token(32);

        sqlx::query(r#"
            INSERT INTO tokens ( token, owner_id, permissions )
            VALUES ( $1, $2, $3 )
        "#)
            .bind(&token)
            .bind(owner_id)
            .bind(permissions.bits())
            .execute(db.connection())
            .await?;

        log::debug!("Created token '{token}'");

        Ok(Self {
            token,
            owner_id,
            permissions,
        })
    }

    pub async fn get_by_token(db: &mut Database, token: String) -> anyhow::Result<Option<Self>>{
        let mut rows = sqlx::query("SELECT * FROM tokens WHERE token = $1")
            .bind(token)
            .fetch(db.connection());

        while let Some(row) = rows.try_next().await? {
            return Ok(Some(Self {
                token: row.try_get("token")?,
                owner_id: row.try_get("owner_id")?,
                permissions: Permissions::from_bits(row.try_get("permissions")?).unwrap(),
            }));
        }

        Ok(None)
    }

    pub async fn get_by_owner_id(db: &mut Database, owner_id: String) -> anyhow::Result<Option<Self>>{
        let mut rows = sqlx::query("SELECT * FROM tokens WHERE owner_id = $1")
            .bind(owner_id)
            .fetch(db.connection());

        while let Some(row) = rows.try_next().await? {
            return Ok(Some(Self {
                token: row.try_get("token")?,
                owner_id: row.try_get("owner_id")?,
                permissions: Permissions::from_bits(row.try_get("permissions")?).unwrap(),
            }));
        }

        Ok(None)
    }


    pub async fn save(&self, db: &mut Database) -> anyhow::Result<()> {
        let _ = sqlx::query(r#"
                UPDATE users
                SET token = $1
                SET owner_id = $2
                SET permissions = $5
            "#)
            .bind(&self.token)
            .bind(&self.owner_id)
            .bind(&self.permissions.bits())
            .execute(db.connection()).await?;

        Ok(())
    }

    pub async fn remove(&self, db: &mut Database) -> anyhow::Result<()> {
        sqlx::query(r#"
            DELETE FROM tokens
            WHERE token = $1
        "#)
            .bind(&self.token)
            .execute(db.connection())
            .await?;

        log::debug!("Deleted token '{}'", self.token);

        Ok(())
    }
}




const TOKEN_CHARSET: &'static [char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn generate_token(len: u8) -> String {
    let mut token = String::new();

    for _ in 0..len {
        let rand = rand::thread_rng().gen_range(0..TOKEN_CHARSET.len());
        token.push(TOKEN_CHARSET[rand])
    }

    token
}