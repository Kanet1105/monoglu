mod influx;
mod redis;

use std::{ops::Deref, sync::Arc};

pub struct Database<T>(Arc<T>);

impl<T> Database<T> {
    pub fn new(db_client: T) -> Self {
        Self(Arc::new(db_client))
    }
}

impl<T> Clone for Database<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Deref for Database<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0    
    }
}

pub enum DatabaseBuilder {
    InfluxDB,
    Redis,
}

// TODO(): Create a database instance from db client.
// impl DatabaseBuilder {
//     pub fn init(self) -> Database {
//         Self::InfluxDB => {
//             unimplemented!();
//         }
//         Self::Redis => {
//             unimplemented!();
//         },
//     }
// }
