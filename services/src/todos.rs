use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::entities::todos::Model as TodoModel;

/// Serviço de Todos: injeta tanto o TodosRepo quanto (opcionalmente) UsersRepo para validações.
pub struct TodosService {
    todos_repo: Arc<dyn TodosRepo>,
    users_repo: Arc<dyn UsersRepo>,
}

impl TodosService {
    pub fn new(todos_repo: Arc<dyn TodosRepo>, users_repo: Arc<dyn UsersRepo>) -> Self {
        Self { todos_repo, users_repo }
    }

    /// Cria um todo: valida título e verifica existência do user.
    pub async fn create_todo(
        &self,
        user_id: Uuid,
        title: String,
        due_date: Option<DateTime<Utc>>,
    ) -> Result<TodoModel, AppError> {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("title cannot be empty".into()));
        }

        // Checar existência do user (regras de negócio: retorna BadRequest se user inexistente)
        let user_exists = self.users_repo.find_by_id(user_id).await.map_err(AppError::Internal)?;
        if user_exists.is_none() {
            return Err(AppError::BadRequest("user not found".into()));
        }

        let todo = self
            .todos_repo
            .create(user_id, title, due_date)
            .await
            .map_err(AppError::Internal)?;

        Ok(todo)
    }

    pub async fn get_todo(&self, id: i32) -> Result<TodoModel, AppError> {
        match self.todos_repo.find_by_id(id).await.map_err(AppError::Internal)? {
            Some(t) => Ok(t),
            None => Err(AppError::NotFound),
        }
    }

    pub async fn list_todos(
        &self,
        user_id: Option<Uuid>,
        status: Option<bool>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<TodoModel>, AppError> {
        let list = self
            .todos_repo
            .list(user_id, status, limit, offset)
            .await
            .map_err(AppError::Internal)?;
        Ok(list)
    }

    pub async fn update_todo(
        &self,
        id: i32,
        title: Option<String>,
        status: Option<bool>,
        due_date: Option<Option<DateTime<Utc>>>,
    ) -> Result<TodoModel, AppError> {
        match self
            .todos_repo
            .update(id, title, status, due_date)
            .await
            .map_err(AppError::Internal)?
        {
            Some(t) => Ok(t),
            None => Err(AppError::NotFound),
        }
    }

    pub async fn delete_todo(&self, id: i32) -> Result<(), AppError> {
        let ok = self.todos_repo.delete(id).await.map_err(AppError::Internal)?;
        if ok {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}
