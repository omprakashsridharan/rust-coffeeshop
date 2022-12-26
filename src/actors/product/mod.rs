use actix::prelude::*;
use sqlx::{Pool, Postgres};
use sqlx::Result as SqlxResult;

mod db;

pub struct ProductActor {
    db_pool: Pool<Postgres>,
}

impl Actor for ProductActor { type Context = Context<Self>; }

#[derive(Message)]
#[rtype(result = "SqlxResult<Vec<db::Product>>")]
pub struct GetAllProductsMessage;

impl Handler<GetAllProductsMessage> for ProductActor {
    type Result = ResponseFuture<SqlxResult<Vec<db::Product>>>;

    fn handle(&mut self, _msg: GetAllProductsMessage, _ctx: &mut Self::Context) -> Self::Result {
        let pool = self.db_pool.clone();
        Box::pin(async move {
            db::get_all(pool).await
        })
    }
}

#[derive(Message)]
#[rtype(result = "SqlxResult<Vec<db::Product>>")]
pub struct GetProductsByNamesMessage {
    names: Vec<String>,
}

impl Handler<GetProductsByNamesMessage> for ProductActor {
    type Result = ResponseFuture<SqlxResult<Vec<db::Product>>>;

    fn handle(&mut self, msg: GetProductsByNamesMessage, _ctx: &mut Self::Context) -> Self::Result {
        let pool = self.db_pool.clone();
        Box::pin(async move {
            db::get_by_names(pool, msg.names).await
        })
    }
}