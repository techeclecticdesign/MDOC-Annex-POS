use crate::common::error::AppError;
use crate::domain::models::PriceAdjustment;

pub trait PriceAdjustmentRepoTrait: Send + Sync {
    fn create(&self, adj: &PriceAdjustment) -> Result<(), AppError>;
    fn create_with_tx(
        &self,
        adj: &PriceAdjustment,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<i32, AppError>;
    fn get_by_id(&self, id: i32) -> Result<Option<PriceAdjustment>, AppError>;
    fn list_for_product(&self, upc: String) -> Result<Vec<PriceAdjustment>, AppError>;
    fn list_for_operator(&self, operator_mdoc: i32) -> Result<Vec<PriceAdjustment>, AppError>;
    fn list_for_today(&self) -> Result<Vec<PriceAdjustment>, AppError>;
    fn list(&self) -> Result<Vec<PriceAdjustment>, AppError>;
    fn search(
        &self,
        limit: i32,
        offset: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(PriceAdjustment, String, String)>, AppError>;
    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError>;
}
