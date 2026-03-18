use crate::core::{
    db::{error::Error, DB},
    models::{
        base::Base,
        ids::{BaseId, UserId},
        permissions::BasePermissions,
    },
};

#[derive(Debug, Clone)]
pub struct BaseService {
    pub base: Base,
    pub user: UserId,
    base_record_id: BaseId,
}

impl BaseService {
    pub fn id(&self) -> &BaseId {
        &self.base_record_id
    }

    pub async fn new(base_id: BaseId, user: UserId) -> Result<Self, Error> {
        let mut res = DB
            .query(
                "
BEGIN TRANSACTION;

LET $base_record = (SELECT * FROM $base WHERE is_deleted = false)[0];

LET $accessible_base = (
    SELECT * FROM $base WHERE 
        is_deleted = false AND (
            owner = $user OR 
            fn::can(
                (SELECT VALUE perms FROM can_access_base WHERE in = $user AND out = $this.id)[0], 
                2
            )
        )
)[0];

IF $accessible_base == NONE {
    THROW 'Permission Denied: User ' + <string>$user + ' cannot access ' + <string>$base;
};

RETURN $accessible_base;

COMMIT TRANSACTION;
            ",
            )
            .bind(("base", base_id.clone()))
            .bind(("user", user.clone()))
            .await?;
        let base: Base = res.take::<Option<Base>>(7)?.ok_or(Error::Database(
            super::errors::DatabaseError::QueryFailed(format!("{}:{}", file!(), line!())),
        ))?;
        Ok(Self {
            base,
            base_record_id: base_id,
            user,
        })
    }

    pub async fn invite_user(&self, user: UserId, perms: BasePermissions) -> Result<(), Error> {
        let res = DB.query("
            BEGIN TRANSACTION;

-- View (1 << 1) = 2 + ManageInvitations (1 << 8) = 256

LET $is_owner = (SELECT VALUE owner FROM $target_base)[0] == $inviter_id;
LET $inviter_perms = (SELECT VALUE perms FROM can_access_base WHERE in = $inviter_id AND out = $target_base)[0] OR 0;

IF !$is_owner AND !fn::can($inviter_perms, 258) {
    THROW 'Unauthorized: You need [View] and [ManageInvitations] to invite others.';
};

RELATE $invited_id->can_access_base->$target_base 
    SET perms = $requested_perms;

COMMIT TRANSACTION;
            ").bind(("inviter_id",self.user.clone())).bind(("invited_id",user)).bind(("target_base",self.base_record_id.clone())).bind(("perms",perms)).await?;
        res.check()?;
        Ok(())
    }
}
