use crate::{
    error::{
        AppError, //
        AuthError,
        ResourceError,
    },
    models::entities::{
        device, //
        prelude::Device,
        sea_orm_active_enums::DeviceStatus,
    },
    views::device::{
        CreateDeviceRequest, //
        UpdateDeviceRequest,
    },
};

use sea_orm::*;
use uuid::Uuid;

pub struct Devices;

impl Devices {
    pub async fn find_all_by_user(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<device::Model>, AppError> {
        let devices = Device::find()
            .filter(device::Column::UserId.eq(user_id))
            .order_by_asc(device::Column::Id)
            .all(db)
            .await?;
        Ok(devices)
    }

    pub async fn find_by_id_and_user(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
    ) -> Result<Option<device::Model>, AppError> {
        let device = Device::find_by_id(id)
            .filter(device::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        Ok(device)
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        data: CreateDeviceRequest,
    ) -> Result<device::Model, AppError> {
        let api_key = Uuid::new_v4().to_string();

        let active_model = device::ActiveModel {
            user_id: Set(user_id),
            name: Set(data.name),
            reservoir_id: Set(data.reservoir_id),
            api_key: Set(api_key),
            status: Set(DeviceStatus::Offline),
            ..Default::default()
        };

        let res = active_model.insert(db).await?;
        Ok(res)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
        data: UpdateDeviceRequest,
    ) -> Result<device::Model, AppError> {
        let model = Self::find_by_id_and_user(db, id, user_id)
            .await?
            .ok_or_else(|| ResourceError::NotFound {
                msg: "Device not found".into(),
            })?;

        if model.user_id != user_id {
            return Err(AuthError::PermissionDenied)?;
        }

        let mut active: device::ActiveModel = model.into();

        if let Some(name) = data.name {
            active.name = Set(name);
        }

        match data.reservoir_id {
            Some(Some(reservoir_id)) => {
                active.reservoir_id = Set(Some(reservoir_id));
            }
            Some(None) => {
                active.reservoir_id = Set(None);
            }
            None => {}
        }

        if let Some(status) = data.status {
            active.status = Set(status.into());
        }

        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32, user_id: i32) -> Result<(), AppError> {
        let result = Device::delete_many()
            .filter(device::Column::Id.eq(id))
            .filter(device::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        if result.rows_affected == 0 {
            return Err(ResourceError::NotFound {
                msg: "Device not found".into(),
            }
            .into());
        }

        Ok(())
    }

    pub async fn rotate_api_key(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
    ) -> Result<String, AppError> {
        let device = Self::find_by_id_and_user(db, id, user_id)
            .await?
            .ok_or_else(|| ResourceError::NotFound {
                msg: "Device not found".into(),
            })?;

        let new_key = Uuid::new_v4().to_string();

        let mut active: device::ActiveModel = device.into();
        active.api_key = Set(new_key.clone());

        active.update(db).await?;

        Ok(new_key)
    }
}
