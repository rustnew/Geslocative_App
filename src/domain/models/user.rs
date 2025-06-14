use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Role {
    Admin,
    User,
    Guest,
}

impl Role {
    pub fn from_i32(value: i32) -> Option<Role> {
        match value {
            1 => Some(Role::Admin),
            2 => Some(Role::User),
            3 => Some(Role::Guest),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Role::Admin => 1,
            Role::User => 2,
            Role::Guest => 3,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Users {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub password: String,
    pub telephone: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateUser {
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub password: String,
    pub telephone: String,
    pub role: i32,
    pub created_user: Uuid,
}

impl Users {
    pub fn new(id: Uuid, create_user: CreateUser) -> Self {
        let now = Utc::now();
        Users {
            id,
            nom: create_user.nom,
            prenom: create_user.prenom,
            email: create_user.email,
            password: create_user.password,
            telephone: create_user.telephone,
            role: Role::from_i32(create_user.role).unwrap_or(Role::Guest),
            created_at: now,
            updated_at: now,
        }
    }
}