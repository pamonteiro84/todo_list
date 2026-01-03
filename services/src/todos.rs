use anyhow::{Error, Result};
use chrono::{DateTime, Utc};
use entities::todos::{self, Entity as TodosEntity};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
    TransactionTrait,
};
use uuid::Uuid;

pub struct TodosService<'a, C: ConnectionTrait + TransactionTrait> {
    database: &'a C,
}

impl<'a, C> TodosService<'a, C>
where
    C: ConnectionTrait + TransactionTrait,
{
    pub fn new(database: &'a C) -> Self {
        Self { database }
    }

    pub async fn create_todo(
        &self,
        user_id: Uuid,
        title: String,
        due_date: Option<DateTime<Utc>>,
    ) -> Result<todos::Model, Error> {
        if title.trim().is_empty() {
            return Err(Error::msg("Title cannot be empty"));
        }

        let db_transaction = self
            .database
            .begin()
            .await
            .map_err(|e| Error::msg(format!("Failed to begin transaction: {}", e)))?;

        let new_todo = todos::ActiveModel {
            user_id: Set(user_id),
            title: Set(title),
            status: Set(false),
            due_date: Set(due_date.map(|dt| dt.naive_utc())),
            ..Default::default()
        };

        let created_todo = new_todo
            .insert(&db_transaction)
            .await
            .map_err(|e| Error::msg(format!("Failed to create todo: {}", e)))?;

        db_transaction
            .commit()
            .await
            .map_err(|e| Error::msg(format!("Failed to commit transaction: {}", e)))?;

        Ok(created_todo)
    }

    pub async fn list_todos(&self) -> Result<Vec<todos::Model>, Error> {
        TodosEntity::find()
            .all(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to list todos: {}", e)))
    }

    pub async fn get_todo(&self, todo_id: i32) -> Result<Option<todos::Model>, Error> {
        TodosEntity::find_by_id(todo_id)
            .one(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to get todo {}: {}", todo_id, e)))
    }

    pub async fn get_todos_by_user(&self, user_id: Uuid) -> Result<Vec<todos::Model>, Error> {
        let mut condition = Condition::all();
        condition = condition.add(todos::Column::UserId.eq(user_id));

        TodosEntity::find()
            .filter(condition)
            .all(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to get todos for user {}: {}", user_id, e)))
    }

    pub async fn delete_todo(self, todo_id: i32) -> Result<(), Error> {
        let todo: Option<todos::Model> = TodosEntity::find_by_id(todo_id)
            .one(self.database)
            .await
            .map_err(|e| Error::msg(format!("Failed to find todo {}: {}", todo_id, e)))?;

        if let Some(todo) = todo {
            let active_model: todos::ActiveModel = todo.into();
            active_model
                .delete(self.database)
                .await
                .map_err(|e| Error::msg(format!("Failed to delete todo {}: {}", todo_id, e)))?;
            Ok(())
        } else {
            Err(Error::msg(format!("Todo with id {} not found", todo_id)))
        }
    }
}
