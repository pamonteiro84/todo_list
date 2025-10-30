use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251029_000001_create_todos_table::Todos;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Name,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
     async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Users::UpdatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users-todos_user_id")
                            .from(Users::Table, Users::Id)
                            .to(Todos::Table, Todos::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await

            

    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
       
       todo!()
    }
}
