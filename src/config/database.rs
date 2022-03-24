use rocket_sync_db_pools::{database, postgres};

#[database("url_linker")]
pub struct DbConnection(postgres::Client);
