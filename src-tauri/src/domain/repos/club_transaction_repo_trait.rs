use crate::common::error::AppError;
use crate::domain::models::ClubTransaction;

pub trait ClubTransactionRepoTrait: Send + Sync {
    fn list(&self) -> Result<Vec<ClubTransaction>, AppError>;
    fn get_by_id(&self, id: i32) -> Result<Option<ClubTransaction>, AppError>;
    fn create(&self, tx: &ClubTransaction) -> Result<(), AppError>;
    fn search(
        &self,
        limit: i32,
        offset: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(ClubTransaction, Option<String>)>, AppError>;
    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError>;
    fn get_account_total(&self) -> Result<i32, AppError>;
}
