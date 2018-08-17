use std::io;
use users::switch::{set_current_gid, set_current_uid};
use users::{get_group_by_name, get_user_by_name, gid_t, uid_t};

#[derive(Debug)]
pub struct Identity {
    uid: uid_t,
    gid: gid_t,
}

pub fn set_current(identity: &Identity) -> io::Result<()> {
    set_current_gid(identity.gid)?;
    set_current_uid(identity.uid)?;

    Ok(())
}

impl Identity {
    pub fn load(username: &str, group: &str) -> Option<Self> {
        let user = get_user_by_name(username)?;
        let group = get_group_by_name(group)?;

        Some(Self {
            uid: user.uid(),
            gid: group.gid(),
        })
    }

    pub(super) fn uid(&self) -> uid_t {
        self.uid
    }

    pub(super) fn gid(&self) -> gid_t {
        self.gid
    }
}
