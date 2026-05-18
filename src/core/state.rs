use crate::core::error::AppError;
use crate::models::{dotfile::Dotfile, profile::Profile};
use crate::models::{dotfile::DotfileId, profile::ProfileId};
use rusqlite::Connection;
use std::{cell::RefCell, rc::Rc};

pub struct Database {
    pub conn: Rc<RefCell<Connection>>,
}

pub struct SqlProfileRepo {
    pub conn: Rc<RefCell<Connection>>,
}

#[allow(dead_code)]
pub struct SqlDotfileRepo {
    pub conn: Rc<RefCell<Connection>>,
}

#[allow(dead_code)]
impl Database {
    fn profiles(&self) -> SqlProfileRepo {
        SqlProfileRepo {
            conn: self.conn.clone(),
        }
    }
    fn dotfiles(&self) -> SqlDotfileRepo {
        SqlDotfileRepo {
            conn: self.conn.clone(),
        }
    }
}

#[allow(dead_code)]
pub trait ProfileRepo {
    fn get(&self, profile_id: ProfileId) -> Result<Profile, AppError>;
    fn add(&self, profile: &Profile) -> Result<(), AppError>;
    fn dotfiles(&self, profile_id: ProfileId) -> Result<Vec<Dotfile>, AppError>;
}

pub trait DotfileRepo {
    fn get(&self, dotfile_id: DotfileId) -> Result<Dotfile, AppError>;
}

pub struct ProfileService<T> {
    pub repo: T,
}

#[allow(dead_code)]
impl<T: ProfileRepo> ProfileService<T> {
    fn get_profile(&self, profile_id: ProfileId) -> Result<Profile, AppError> {
        self.repo.get(profile_id)
    }
    fn add_profile(&self, profile: &Profile) -> Result<(), AppError> {
        self.repo.add(profile)
    }
}

#[allow(dead_code)]
impl ProfileRepo for SqlProfileRepo {
    fn get(&self, profile_id: ProfileId) -> Result<Profile, AppError> {
        let conn = self.conn.borrow();
        let mut stmt = conn
            .prepare("SELECT id, name FROM profiles WHERE profile_id = ?1")
            .unwrap();
        let res = stmt
            .query_one([profile_id.0], |row| {
                Ok(Profile {
                    id: ProfileId(row.get(0).unwrap()),
                    name: row.get(1).unwrap(),
                })
            })
            .unwrap();
        Ok(res)
    }
    fn add(&self, profile: &Profile) -> Result<(), AppError> {
        let conn = self.conn.borrow();
        conn.execute(
            "INSERT INTO profiles (profile_id) VALUES (?1)",
            (profile.id.0,),
        )
        .unwrap();
        Ok(())
    }
    fn dotfiles(&self, _profile_id: ProfileId) -> Result<Vec<Dotfile>, AppError> {
        Ok(Vec::new())
    }
}

pub struct DotfileService<T> {
    pub repo: T,
}
#[allow(dead_code)]
impl<T: DotfileRepo> DotfileService<T> {
    fn get_dotfile(&self, dotfile_id: DotfileId) -> Result<Dotfile, AppError> {
        self.repo.get(dotfile_id)
    }
}

#[cfg(test)]
mod tests {}
