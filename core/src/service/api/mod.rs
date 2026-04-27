use crate::db::*;
use crate::models::*;
use crate::service::errors::ApiError;
use crate::service::user::UserService;

#[derive(Debug)]
pub struct ApiService {}

impl ApiService {
    pub async fn get_user(token: String) -> Result<UserService, Irror> {
        let mut res = DB
            .query(
                "SELECT user.* FROM api_token WHERE expires_at > time::now()   
                    AND crypto::sha512($tokenn) == `token`;",
            )
            .bind(("tokenn", token))
            .await?;
        let user: User = res
            .take::<Option<User>>(0)?
            .ok_or(Irror::Api(ApiError::NotFound))?;

        Ok(UserService {
            user_record_id: user.id.clone().ok_or(Irror::Api(ApiError::Unauthorized))?,
            user,
            current_base: None,
        })
    }
}
