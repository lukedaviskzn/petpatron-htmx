
pub struct Dog {
    pub id: i64,
    pub name: String,
    pub bio: String,
    pub breed: String,
    pub image: String,
    pub account: i64,
    pub birth_date: Option<chrono::NaiveDate>,
}

impl Dog {
    pub async fn from_row(row: libsql::Row) -> libsql::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            bio: row.get(2)?,
            breed: row.get(3)?,
            image: row.get(4)?,
            account: row.get(5)?,
            birth_date: chrono::NaiveDate::parse_from_str(&row.get::<String>(6)?, "%Y-%m-%d").ok(),
        })
    }

    pub async fn fetch_all(conn: &libsql::Connection) -> libsql::Result<Vec<Self>> {
        let mut rows = conn.query("SELECT id, name, bio, breed, image, account, birth_date FROM dogs", ()).await.unwrap();
        let mut dogs = Vec::new();
        while let Some(row) = rows.next().await? {
            dogs.push(Self::from_row(row).await?);
        }
        Ok(dogs)
    }

    pub async fn fetch(conn: &libsql::Connection, id: i64) -> libsql::Result<Option<Self>> {
        let mut rows = conn.query("SELECT id, name, bio, breed, image, account, birth_date FROM dogs WHERE id = ?", libsql::params!(id)).await.unwrap();
        let dog;
        if let Some(row) = rows.next().await? {
            dog = Some(Self::from_row(row).await?);
        } else {
            dog = None;
        }
        Ok(dog)
    }
}