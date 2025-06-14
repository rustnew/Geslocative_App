use async_trait::async_trait;
use serde::Deserialize;
use sqlx::PgPool;
use crate::domain::models::user::{Users, CreateUser, Role};
use crate::ports::input::user::UserRepository;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Structure temporaire pour mapper les résultats SQL
#[derive(sqlx::FromRow, Deserialize, Debug)]
struct UserRow {
    id: Uuid,
    nom: String,
    prenom: String,
    email: String,
    password: String,
    telephone: String,
    role: i32, // Lisez comme i32 directement
    created_at: Option<DateTime<Utc>>, // Changé en Option pour gérer les valeurs nullables
    updated_at: Option<DateTime<Utc>>, // Changé en Option pour gérer les valeurs nullables
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Users>, String> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, nom, prenom, email, password, telephone, role, created_at, updated_at
            FROM Users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(user_row.map(|u| Users {
            id: u.id,
            nom: u.nom,
            prenom: u.prenom,
            email: u.email,
            password: u.password,
            telephone: u.telephone,
            role: Role::from_i32(u.role).unwrap_or(Role::Guest),
            created_at: u.created_at.unwrap_or_else(|| Utc::now()), // Valeur par défaut si null
            updated_at: u.updated_at.unwrap_or_else(|| Utc::now()), // Valeur par défaut si null
        }))
    }

    async fn create(&self, create_user: CreateUser) -> Result<Users, String> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"
            INSERT INTO Users (id, nom, prenom, email, password, telephone, role)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, nom, prenom, email, password, telephone, role, created_at, updated_at
            "#,
            Uuid::new_v4(),
            &create_user.nom,
            &create_user.prenom,
            &create_user.email,
            &create_user.password,
            &create_user.telephone,
            create_user.role, // Conversion explicite du Role en i32
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Users {
            id: user_row.id,
            nom: user_row.nom,
            prenom: user_row.prenom,
            email: user_row.email,
            password: user_row.password,
            telephone: user_row.telephone,
            role: Role::from_i32(user_row.role).unwrap_or(Role::Guest),
            created_at: user_row.created_at.unwrap_or_else(|| Utc::now()),
            updated_at: user_row.updated_at.unwrap_or_else(|| Utc::now()),
        })
    }

    async fn update(&self, user: Users) -> Result<Users, String> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"
            UPDATE Users
            SET nom = $1, prenom = $2, email = $3, password = $4, telephone = $5, role = $6, updated_at = CURRENT_TIMESTAMP
            WHERE id = $7
            RETURNING id, nom, prenom, email, password, telephone, role, created_at, updated_at
            "#,
            &user.nom,
            &user.prenom,
            &user.email,
            &user.password,
            &user.telephone,
            user.role.to_i32(), // Conversion explicite du Role en i32
            &user.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Users {
            id: user_row.id,
            nom: user_row.nom,
            prenom: user_row.prenom,
            email: user_row.email,
            password: user_row.password,
            telephone: user_row.telephone,
            role: Role::from_i32(user_row.role).unwrap_or(Role::Guest),
            created_at: user_row.created_at.unwrap_or_else(|| Utc::now()),
            updated_at: user_row.updated_at.unwrap_or_else(|| Utc::now()),
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        sqlx::query!("DELETE FROM Users WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}