use std::io::Write;
use std::str::FromStr;
use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{AsExpression, sql_types::Text, FromSqlRow, deserialize, serialize};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{Output, ToSql};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Deserialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name=crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable, Debug)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type=Text)]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl FromSql<Text, Pg> for RoleCode{
    fn from_sql(pg_value: PgValue<'_>) -> deserialize::Result<Self> {
        match pg_value.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"editor" => Ok(RoleCode::Editor),
            b"viewer" => Ok(RoleCode::Viewer),
            _ => Ok(RoleCode::Viewer)
        }
    }
}

impl ToSql<Text, Pg> for RoleCode{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromStr for RoleCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err(())
        }
    }
}

impl ToString for RoleCode {
    fn to_string(&self) -> String {
        match self {
            RoleCode::Admin => "admin".to_string(),
            RoleCode::Editor => "editor".to_string(),
            RoleCode::Viewer => "viewer".to_string()
        }
    }
}