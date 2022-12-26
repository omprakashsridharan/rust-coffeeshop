use sqlx::{
    Pool,
    Postgres,
    query_as, Result as SqlxResult,
    types::{Decimal, Uuid},
};

pub struct Product {
    id: Uuid,
    name: String,
    product_type: i32,
    price: Decimal,
    image: String,
}

pub async fn get_all(pool: Pool<Postgres>) -> SqlxResult<Vec<Product>> {
    let products = query_as!(Product, "select id, name, type as product_type, price, image from product.products")
        .fetch_all(&pool)
        .await?;
    Ok(products)
}

pub async fn get_by_names(pool: Pool<Postgres>, names: Vec<String>) -> SqlxResult<Vec<Product>> {
    let products = query_as!(Product, "select id, name, type as product_type, price, image from product.products where name = any($1)", names.as_slice())
        .fetch_all(&pool)
        .await?;
    Ok(products)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use sqlx::types::{Decimal, Uuid};

    use crate::actors::db::{get_all, get_by_names};

    #[sqlx::test(fixtures("get_all_products"))]
    async fn get_all_test(pool: PgPool) {
        let products_result = get_all(pool).await;
        assert!(products_result.is_ok());
        let products = products_result.unwrap();
        assert_eq!(1, products.len());
        let first_product = products.get(0).unwrap();
        assert_eq!(first_product.name, String::from("COFFEE"));
        assert_eq!(first_product.product_type, 1);
        assert_eq!(first_product.price, Decimal::from_str("19.99").unwrap());
    }

    #[sqlx::test(fixtures("get_products_by_names"))]
    async fn get_by_names_test(pool: PgPool) {
        let products_result = get_by_names(pool, vec![String::from("COFFEE"), String::from("CHIPS")]).await;
        assert!(products_result.is_ok());
        let products = products_result.unwrap();
        assert_eq!(2, products.len());
        let first_product = products.get(0).unwrap();
        assert_eq!(first_product.name, String::from("COFFEE"));
        assert_eq!(first_product.product_type, 1);
        assert_eq!(first_product.price, Decimal::from_str("19.99").unwrap());

        let second_product = products.get(1).unwrap();
        assert_eq!(second_product.name, String::from("CHIPS"));
        assert_eq!(second_product.product_type, 2);
        assert_eq!(second_product.price, Decimal::from_str("1.99").unwrap());
    }
}