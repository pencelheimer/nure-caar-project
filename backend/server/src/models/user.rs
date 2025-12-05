use crate::{
    error::{
        AppError, //
        AuthError,
    },
    models::entities::{
        device, //
        prelude::*,
        reservoir,
        sea_orm_active_enums::UserRole,
        user,
    },
    views::auth::UpdateProfileRequest,
};

use sea_orm::{prelude::*, *};

pub struct Users;

#[derive(Debug, FromQueryResult)]
pub struct UserWithStats {
    pub id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: UserRole,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub reservoirs_count: i64,
    pub devices_count: i64,
}

impl Users {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, AppError> {
        let user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?;

        Ok(user)
    }

    pub async fn exists_by_email(db: &DatabaseConnection, email: &str) -> Result<bool, AppError> {
        let count = User::find()
            .filter(user::Column::Email.eq(email))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    pub async fn create(
        db: &DatabaseConnection,
        email: String,
        hashed_password: String,
        first_name: Option<String>,
        last_name: Option<String>,
        role: Option<UserRole>,
    ) -> Result<user::Model, AppError> {
        let active_model = user::ActiveModel {
            email: Set(email),
            hashed_password: Set(hashed_password),
            first_name: Set(first_name),
            last_name: Set(last_name),
            role: Set(role.unwrap_or(UserRole::User)),
            ..Default::default()
        };

        let user = active_model.insert(db).await?;

        Ok(user)
    }

    // NOTE(pencelheimer): add pagination?
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<user::Model>, AppError> {
        let users = User::find()
            .order_by_desc(user::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(users)
    }

    pub async fn find_all_with_stats(
        db: &DatabaseConnection,
    ) -> Result<Vec<UserWithStats>, AppError> {
        let users = User::find()
            .select_only()
            .column(user::Column::Id)
            .column(user::Column::Email)
            .column(user::Column::FirstName)
            .column(user::Column::LastName)
            .column(user::Column::Role)
            .column(user::Column::CreatedAt)
            .column_as(
                sea_query::Expr::col((reservoir::Entity, reservoir::Column::Id)).count_distinct(),
                "reservoirs_count",
            )
            .column_as(
                sea_query::Expr::col((device::Entity, device::Column::Id)).count_distinct(),
                "devices_count",
            )
            .left_join(Reservoir)
            .left_join(Device)
            .group_by(user::Column::Id)
            .order_by_desc(user::Column::CreatedAt)
            .into_model::<UserWithStats>()
            .all(db)
            .await?;

        Ok(users)
    }

    pub async fn update_role(
        db: &DatabaseConnection,
        user_id: i32,
        new_role: UserRole,
    ) -> Result<user::Model, AppError> {
        let mut user: user::ActiveModel = User::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or(AuthError::UserNotFound)?
            .into();

        user.role = Set(new_role);

        let updated = user.update(db).await?;
        Ok(updated)
    }

    pub async fn count(db: &DatabaseConnection) -> Result<u64, AppError> {
        Ok(User::find().count(db).await?)
    }

    pub async fn update_profile(
        db: &DatabaseConnection,
        user_id: i32,
        data: UpdateProfileRequest,
    ) -> Result<user::Model, AppError> {
        let user = User::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        let mut active: user::ActiveModel = user.into();

        if let Some(first_name) = data.first_name {
            active.first_name = Set(Some(first_name));
        }
        if let Some(last_name) = data.last_name {
            active.last_name = Set(Some(last_name));
        }

        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn update_password(
        db: &DatabaseConnection,
        user_id: i32,
        new_hash: String,
    ) -> Result<(), AppError> {
        let user = user::ActiveModel {
            id: Set(user_id),
            hashed_password: Set(new_hash),
            ..Default::default()
        };

        user.update(db).await?;
        Ok(())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<user::Model, AppError> {
        User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AuthError::UserNotFound.into())
    }
}
