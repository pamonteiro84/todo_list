use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251029_000001_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Todos {
    Table,
    Id,
    UserId,
    Title,
    Status,
    DueDate,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(pk_auto(Todos::Id))
                    .col(ColumnDef::new(Todos::UserId).uuid().not_null())
                    .col(ColumnDef::new(Todos::Title).string().not_null())
                    .col(ColumnDef::new(Todos::Status).boolean().not_null().default(false))
                    .col(ColumnDef::new(Todos::DueDate).date_time().null())
                    .col(ColumnDef::new(Todos::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Todos::UpdatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-todos-user_id")
                            .from(Todos::Table, Todos::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {

        todo!();
    }
}


// egwbg2b4b