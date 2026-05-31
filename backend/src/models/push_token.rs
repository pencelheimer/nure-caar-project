use crate::{
    error::AppError,
    models::entities::{prelude::*, push_token},
};

use sea_orm::*;

pub struct PushTokens;

impl PushTokens {
    pub async fn upsert(
        db: &DatabaseConnection,
        user_id: i32,
        token: String,
        platform: String,
    ) -> Result<(), AppError> {
        let existing = PushToken::find()
            .filter(push_token::Column::UserId.eq(user_id))
            .filter(push_token::Column::Token.eq(&token))
            .one(db)
            .await?;

        if existing.is_none() {
            push_token::ActiveModel {
                user_id: Set(user_id),
                token: Set(token),
                platform: Set(platform),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }

        Ok(())
    }

    pub async fn find_tokens_by_user(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<String>, AppError> {
        let tokens = PushToken::find()
            .filter(push_token::Column::UserId.eq(user_id))
            .all(db)
            .await?
            .into_iter()
            .map(|t| t.token)
            .collect();

        Ok(tokens)
    }

    pub async fn delete(
        db: &DatabaseConnection,
        user_id: i32,
        token: &str,
    ) -> Result<(), AppError> {
        PushToken::delete_many()
            .filter(push_token::Column::UserId.eq(user_id))
            .filter(push_token::Column::Token.eq(token))
            .exec(db)
            .await?;

        Ok(())
    }
}
