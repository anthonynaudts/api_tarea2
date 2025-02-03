use mysql::prelude::FromRow;
use mysql::Row;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl FromRow for Usuario {
    fn from_row(row: Row) -> mysql::Result<Self> {
        Ok(Usuario {
            id: row.get("id").unwrap(),
            username: row.get("username").unwrap(),
            password: row.get("password").unwrap(),
        })
    }
}
