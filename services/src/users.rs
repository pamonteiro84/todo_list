use anyhow::{Error, Result};
use entities::users::{self, Entity as UsersEntity};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
    TransactionTrait,
};
use uuid::Uuid;

pub struct UsersService<'a, C: ConnectionTrait + TransactionTrait> {
    database: &'a C,
}

impl<'a, C> UsersService<'a, C>
where
    C: ConnectionTrait + TransactionTrait,
{
    pub fn new(database: &'a C) -> Self {
        Self { database }
    }

    pub async fn register_user(
        &self,
        _user_id: Uuid,
        name: String,
        email: String,
        password: String,
    ) -> Result<users::Model, Error> {
        let db_transaction = self
            .database
            .begin()
            .await
            .map_err(|e| Error::msg(format!("Failed to begin transaction: {}", e)))?;

        let existing_user = UsersEntity::find()
            .filter(users::Column::Email.eq(email.clone()))
            .one(&db_transaction)
            .await?;

        if existing_user.is_some() {
            return Err(Error::msg("Email already registered"));
        }

        // TODO: Hash da senha antes de guardar (use bcrypt ou argon2)
        // let password_hash = hash_password(&password)?;

        let new_user = users::ActiveModel {
            name: Set(name),
            email: Set(email),
            password: Set(password), // TODO: usar password_hash
            ..Default::default()
        };

        let created_user = new_user
            .insert(&db_transaction)
            .await
            .map_err(|e| Error::msg(format!("Failed to create user: {}", e)))?;

        db_transaction
            .commit()
            .await
            .map_err(|e| Error::msg(format!("Failed to commit transaction: {}", e)))?;

        Ok(created_user)
    }

    pub async fn list_users(&self) -> Result<Vec<users::Model>, Error> {
        UsersEntity::find()
            .all(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to list todos: {}", e)))
    }

    pub async fn get_user_by_name(&self, name: String) -> Result<Option<users::Model>, Error> {
        let mut condition = Condition::all();
        condition = condition.add(users::Column::Name.eq(name.clone()));

        UsersEntity::find()
            .filter(condition)
            .one(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to get user by name {}: {}", name, e)))
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<Option<users::Model>, Error> {
        let mut condition = Condition::all();
        condition = condition.add(users::Column::Email.eq(email.clone()));

        UsersEntity::find()
            .filter(condition)
            .one(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to get todos for email {}: {}", email, e)))
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<users::Model, Error> {
        let db_transaction = self
            .database
            .begin()
            .await
            .map_err(|e| Error::msg(format!("Failed to begin transaction: {}", e)))?;

        let user = UsersEntity::find_by_id(user_id)
            .one(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to find user: {}", e)))?
            .ok_or_else(|| Error::msg("User not found"))?;

        let mut user_active: users::ActiveModel = user.into();

        if let Some(n) = name {
            if n.trim().is_empty() {
                return Err(Error::msg("Name cannot be empty"));
            }

            user_active.name = Set(n);
        }

        if let Some(e) = email {
            if e.trim().is_empty() {
                return Err(Error::msg("Email cannot be empty"));
            }

            user_active.email = Set(e);
        }

        if let Some(p) = password {
            if p.trim().is_empty() {
                return Err(Error::msg("Password ecannot be empty"));
            }

            user_active.password = Set(p);
        }

        user_active.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated_user = user_active
            .update(&db_transaction)
            .await
            .map_err(|e| Error::msg(format!("Failed to update user: {}", e)))?;

        db_transaction
            .commit()
            .await
            .map_err(|e| Error::msg(format!("Failed to commit transaction {}", e)))?;

        Ok(updated_user)
    }

    pub async fn delete_user(self, user_id: Uuid) -> Result<(), Error> {
        let user: Option<users::Model> = UsersEntity::find_by_id(user_id)
            .one(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to find todo {}: {}", user_id, e)))?;

        if let Some(user) = user {
            let active_model: users::ActiveModel = user.into();
            active_model
                .delete(self.database)
                .await
                .map_err(|e| Error::msg(format!("Failed to delete todo {}: {}", user_id, e)))?;
            Ok(())
        } else {
            Err(Error::msg(format!("Todo with id {} not found", user_id)))
        }
    }
}
