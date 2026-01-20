use serde::Deserialize;

use crate::models::ids::UserId;
use crate::models::session::Session;
use crate::models::user::*;
use crate::services::session::get_session_with_token;
use crate::services::Error;
use crate::services::DB;

const USER: &str = "user";

// alr alr, so there is a lil issue, there is only an admin and an user, one that has some obvoious
// permissions like reading its own data and anoter that can do pretty much everything, but what if
// i added permissions, yea, this could make everything just way better and easier,
// EDIT: its totally not easier wtf did i just write 2 weeks ago
//
// MRAOW I HATE MY LIFE UWU

// like hey, the admin dont need that MANY info , right ?
#[derive(Deserialize)]
pub struct PublicUser {
    id: UserId,
    username: Username,
    role: UserRole,
    is_deleted: bool,
}

pub async fn create_user(new_user: InsertUser) -> Result<Option<User>, Error> {
    DB.create(USER)
        .content(User::from_insert(new_user))
        .await?
        .ok_or(Error::NotFound)
}

pub async fn get_user_by_session_token(session_token: &String) -> Result<Option<User>, Error> {
    let session: Session = get_session_with_token(session_token.to_string()).await?;
    if session.is_expired() {
        return Err(Error::Forbidden);
    }
    let user_id = session.user.0.id.to_string();
    let user: User = DB.select((USER, user_id)).await?.ok_or(Error::NotFound)?;
    if user.is_deleted {
        return Err(Error::NotFound); // hihihi, how will they know =3
    };
    Ok(Some(user))
}

/// only admin can update a user
pub async fn update_user(
    user_updated: UserPatch,
    user_id: UserId,
    session_token: String,
) -> Result<Option<User>, Error> {
    let user = get_user_by_session_token(&session_token)
        .await?
        .ok_or(Error::NotFound)?;
    match user.role {
        UserRole::Admin => {
            let patch: AdminUserPatch = user_updated.clone().into();
            DB.update((USER, user_id.0.id.to_string()))
                .content(patch)
                .await
                .map_err(|_| Error::Db)
        }
        UserRole::User => {
            if user.id == Some(user_id.clone()) {
                let patch: SelfUserPatch = user_updated.into();
                DB.update((USER, user_id.0.id.to_string()))
                    .content(patch)
                    .await
                    .map_err(|_| Error::Db)
            } else {
                Err(Error::Forbidden)
            }
        }
    }
}

/// heck, only an admin can delete a user too XD
pub async fn delete_user(user: UserId, session_token: String) -> Result<Option<User>, Error> {
    let owner = get_user_by_session_token(&session_token)
        .await?
        .ok_or(Error::NotFound)?;
    match owner.role {
        UserRole::User => Err(Error::Forbidden),
        UserRole::Admin => DB
            .update((USER, user.0.id.to_string()))
            .content(UserPatch {
                is_deleted: Some(true),
                last_name: None,
                first_name: None,
                username: None,
            })
            .await?
            .ok_or(Error::NotFound)?,
    }
}

/// only admin can do that :p
pub async fn list_all_users(session_token: String) -> Result<Vec<PublicUser>, Error> {
    let user = get_user_by_session_token(&session_token)
        .await?
        .ok_or(Error::Forbidden)?;
    match user.role {
        UserRole::Admin => Ok(DB.select(USER).await?),
        UserRole::User => Err(Error::Forbidden),
    }
}
