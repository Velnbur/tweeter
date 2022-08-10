use mobc_postgres::tokio_postgres::NoTls;
use mobc_postgres::PgConnectionManager;

pub type Pool = mobc::Pool<PgConnectionManager<NoTls>>;
pub type Connection = mobc::Connection<PgConnectionManager<NoTls>>;
