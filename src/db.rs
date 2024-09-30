use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::{File, ReceiveFileDetails, SentFileDetails, SharedLink, User};

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;

    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, sqlx::Error>;

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        name: T,
    ) -> Result<User, sqlx::Error>;

    async fn update_user_password(
        &self,
        user_id: Uuid,
        password: String,
    ) -> Result<User, sqlx::Error>;

    async fn save_user_key(&self, user_id: Uuid, public_key: String) -> Result<(), sqlx::Error>;

    async fn search_by_email(&self, user_id: Uuid, query: String)
        -> Result<Vec<User>, sqlx::Error>;

    async fn save_encrypted_file(
        &self,
        user_id: Uuid,
        file_name: String,
        file_size: i64,
        recipient_user_id: Uuid,
        password: String,
        expiration_date: DateTime<Utc>,
        encrypted_aes_key: Vec<u8>,
        encrypted_file: Vec<u8>,
        iv: Vec<u8>,
    ) -> Result<(), sqlx::Error>;

    async fn get_shared(
        &self,
        shared_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<SharedLink>, sqlx::Error>;

    async fn get_file(
        &self,
        file_id: Uuid,
    ) -> Result<Option<File>, sqlx::Error>;

    async fn get_sent_files(
        &self,
        user_id: Uuid,
        page: u32,
        limit: usize
    ) -> Result<(Vec<SentFileDetails>, i64), sqlx::Error>;

    async fn get_receive_files(
        &self,
        user_id: Uuid,
        page: u32,
        limit: usize
    ) -> Result<(Vec<ReceiveFileDetails>, i64), sqlx::Error>;

    async fn delete_expired_files(
        &self
    ) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl UserExt for DBClient {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        let mut user: Option<User> = None;

        if let Some(user_id) = user_id {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, public_key, created_at, updated_at FROM users WHERE id = $1"#,
                user_id
            ).fetch_optional(&self.pool).await?;
        } else if let Some(name) = name {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, public_key, created_at, updated_at FROM users WHERE name = $1"#,
                name
            ).fetch_optional(&self.pool).await?;
        } else if let Some(email) = email {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, public_key, created_at, updated_at FROM users WHERE email = $1"#,
                email
            ).fetch_optional(&self.pool).await?;
        }

        Ok(user)
    }

    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password) 
            VALUES ($1, $2, $3) 
            RETURNING id, name, email, password, public_key, created_at, updated_at
            "#,
            name.into(),
            email.into(),
            password.into()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        new_name: T,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET name = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, public_key, created_at, updated_at
            "#,
            new_name.into(),
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_user_password(
        &self,
        user_id: Uuid,
        new_password: String,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, public_key, created_at, updated_at
            "#,
            new_password,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn save_user_key(&self, user_id: Uuid, public_key: String) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET public_key = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, public_key, created_at, updated_at
            "#,
            public_key,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(())
    }
    async fn search_by_email(
        &self,
        user_id: Uuid,
        query: String,
    ) -> Result<Vec<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, password, public_key, created_at, updated_at
            FROM users
            WHERE email LIKE $1
            AND public_key IS NOT NULL
            AND id != $2
            "#,
            query,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user)
    }
    async fn save_encrypted_file(
        &self,
        user_id: Uuid,
        file_name: String,
        file_size: i64,
        recipient_user_ud: Uuid,
        password: String,
        expiration_date: DateTime<Utc>,
        encrypted_aes_key: Vec<u8>,
        encrypted_file: Vec<u8>,
        iv: Vec<u8>,
    ) -> Result<(), sqlx::Error> {
        // Insert into the files table and get the file_id
        let file_id: Uuid = sqlx::query_scalar!(
            r#"
            INSERT INTO files (user_id, file_name, file_size, encrypted_aes_key, encrypted_file, iv, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING id
            "#,
            user_id,
            file_name,
            file_size,
            encrypted_aes_key,
            encrypted_file,
            iv
        )
        .fetch_one(&self.pool)
        .await?;

        // Insert into the shared_links table using the returned file_id
        sqlx::query!(
            r#"
            INSERT INTO shared_links (file_id, recipient_user_id, password, expiration_date, created_at)
            VALUES ($1, $2, $3, $4, NOW())
            "#,
            file_id,
            recipient_user_ud,
            password,
            expiration_date
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_shared(
        &self,
        shared_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<SharedLink>, sqlx::Error> {
        let shared_link = sqlx::query_as!(
            SharedLink,
            r#"
            SELECT id, file_id, recipient_user_id, password, expiration_date, created_at
            FROM shared_links
            WHERE id = $1
            AND recipient_user_id = $2
            AND expiration_date > NOW()
            "#,
            shared_id,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(shared_link)
    }

    async fn get_file(
        &self,
        file_id: Uuid,
    ) -> Result<Option<File>, sqlx::Error> {
        let file = sqlx::query_as!(
            File,
            r#"
            SELECT id, user_id, file_name, file_size, encrypted_aes_key, encrypted_file, iv, created_at
            FROM files
            WHERE id = $1
            "#,
            file_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(file)
    }
    async fn get_sent_files(
        &self,
        user_id: Uuid,
        page: u32,
        limit: usize
    ) -> Result<(Vec<SentFileDetails>, i64), sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let files = sqlx::query_as!(
            SentFileDetails,
            r#"
                SELECT
                    f.id AS file_id,
                    f.file_name,
                    u.email AS recipient_email,
                    sl.expiration_date,
                    sl.created_at
                FROM 
                    shared_links sl
                JOIN 
                    files f ON sl.file_id = f.id
                JOIN 
                    users u ON sl.recipient_user_id = u.id
                WHERE 
                    f.user_id = $1
                ORDER BY 
                    sl.created_at DESC 
                LIMIT $2 
                OFFSET $3
            "#,
            user_id,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await?;

        let count_row = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*)
                FROM shared_links sl
                JOIN files f ON sl.file_id = f.id
                WHERE f.user_id = $1
            "#,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        let total_count = count_row.unwrap_or(0);

        Ok((files, total_count))
    }

    async fn get_receive_files(
        &self,
        user_id: Uuid,
        page: u32,
        limit: usize
    ) -> Result<(Vec<ReceiveFileDetails>, i64), sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let files = sqlx::query_as!(
            ReceiveFileDetails,
            r#"
                SELECT
                    sl.id AS file_id,
                    f.file_name,
                    u.email AS sender_email,
                    sl.expiration_date,
                    sl.created_at
                FROM 
                    shared_links sl
                JOIN 
                    files f ON sl.file_id = f.id
                JOIN 
                    users u ON f.user_id = u.id
                WHERE 
                    sl.recipient_user_id = $1
                ORDER BY 
                    sl.created_at DESC 
                LIMIT $2 
                OFFSET $3
            "#,
            user_id,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await?;

        let count_row = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*)
                FROM shared_links sl
                JOIN files f ON sl.file_id = f.id
                WHERE sl.recipient_user_id = $1
            "#,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        let total_count = count_row.unwrap_or(0);

        Ok((files, total_count))
    }

    async fn delete_expired_files(
        &self
    ) -> Result<(), sqlx::Error> {
        
        let expired_shared_links: Vec<Uuid> = sqlx::query_scalar!(
            r#"
            SELECT sl.id
            FROM shared_links sl
            WHERE sl.expiration_date < NOW()
            "#,
        ).
        fetch_all(&self.pool)
        .await?;

        if expired_shared_links.is_empty() {
            println!("No expired files or shared links to delete.");
            return Ok(());
        }

        let expired_file_ids: Vec<Uuid> = sqlx::query_scalar!(
            r#"
            SELECT f.id
            FROM files f
            WHERE f.id IN (
                SELECT sl.file_id
                FROM shared_links sl
                WHERE sl.expiration_date < NOW()
            )
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM shared_links
            WHERE id = ANY($1)
            "#,
            &expired_shared_links[..] // Pass the list of expired shared link IDs
        )
        .execute(&self.pool)
        .await?;

        // Delete the expired files
        sqlx::query!(
            r#"
            DELETE FROM files
            WHERE id = ANY($1)
            "#,
            &expired_file_ids[..] // Pass the list of expired file IDs
        )
        .execute(&self.pool)
        .await?;

        println!("Successfully deleted expired files and their shared links.");

        Ok(())

    }
}
