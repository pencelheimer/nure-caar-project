use crate::{
    error::{
        AppError, //
        ResourceError,
    },
    models::entities::{
        prelude::Reservoir, //
        reservoir,
    },
    views::reservoir::{
        CreateReservoirRequest, //
        UpdateReservoirRequest,
    },
};

use sea_orm::*;

pub struct Reservoirs;

impl Reservoirs {
    pub async fn find_all_by_user(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<reservoir::Model>, AppError> {
        let reservoirs = Reservoir::find()
            .filter(reservoir::Column::UserId.eq(user_id))
            .order_by_asc(reservoir::Column::Id)
            .all(db)
            .await?;

        Ok(reservoirs)
    }

    pub async fn find_by_id_and_user(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
    ) -> Result<Option<reservoir::Model>, AppError> {
        let reservoir = Reservoir::find_by_id(id)
            .filter(reservoir::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        Ok(reservoir)
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        data: CreateReservoirRequest,
    ) -> Result<reservoir::Model, AppError> {
        let active_model = reservoir::ActiveModel {
            user_id: Set(user_id),
            name: Set(data.name),
            description: Set(data.description),
            capacity: Set(data.capacity),
            location: Set(data.location),
            ..Default::default()
        };

        let res = active_model.insert(db).await?;
        Ok(res)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
        data: UpdateReservoirRequest,
    ) -> Result<reservoir::Model, AppError> {
        let model = Self::find_by_id_and_user(db, id, user_id)
            .await?
            .ok_or_else(|| ResourceError::NotFound {
                msg: "Reservoir not found".into(),
            })?;

        let mut active: reservoir::ActiveModel = model.into();

        if let Some(name) = data.name {
            active.name = Set(name);
        }
        if let Some(desc) = data.description {
            active.description = Set(Some(desc));
        }
        if let Some(cap) = data.capacity {
            active.capacity = Set(cap);
        }
        if let Some(loc) = data.location {
            active.location = Set(Some(loc));
        }

        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32, user_id: i32) -> Result<(), AppError> {
        let result = Reservoir::delete_many()
            .filter(reservoir::Column::Id.eq(id))
            .filter(reservoir::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        if result.rows_affected == 0 {
            return Err(ResourceError::NotFound {
                msg: "Reservoir not found".into(),
            })?;
        }

        Ok(())
    }
}
