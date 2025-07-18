use crate::common::error::AppError;
use crate::domain::models::Product;

pub trait ProductRepoTrait: Send + Sync {
    fn get_by_upc(&self, upc: String) -> Result<Option<Product>, AppError>;
    fn get_price(&self, upc: String) -> Result<i32, AppError>;
    fn create(&self, product: &Product) -> Result<(), AppError>;
    fn update_by_upc(&self, product: &Product) -> Result<(), AppError>;
    fn update_by_upc_with_tx(
        &self,
        product: &Product,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError>;
    fn list(&self) -> Result<Vec<Product>, AppError>;
    fn search(
        &self,
        desc_like: Option<String>,
        category: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(Product, i64)>, AppError>;
    fn count(&self, desc_like: Option<String>, category: Option<String>) -> Result<u32, AppError>;
}
