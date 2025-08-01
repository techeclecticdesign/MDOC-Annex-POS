use crate::common::error::AppError;
use crate::domain::models::PriceAdjustment;
use crate::domain::repos::PriceAdjustmentRepoTrait;
use std::sync::Mutex;

pub struct MockPriceAdjustmentRepo {
    store: Mutex<Vec<PriceAdjustment>>,
}

impl MockPriceAdjustmentRepo {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockPriceAdjustmentRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl PriceAdjustmentRepoTrait for MockPriceAdjustmentRepo {
    fn create(&self, adj: &PriceAdjustment) -> Result<(), AppError> {
        self.store.lock().unwrap().push(adj.clone());
        Ok(())
    }

    fn create_with_tx(
        &self,
        adj: &PriceAdjustment,
        _tx: &rusqlite::Transaction<'_>,
    ) -> Result<i32, AppError> {
        self.create(adj).map(|_| adj.id)
    }

    fn get_by_id(&self, id: i32) -> Result<Option<PriceAdjustment>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.id == id)
            .cloned())
    }

    fn list_for_product(&self, upc: String) -> Result<Vec<PriceAdjustment>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.upc == upc)
            .cloned()
            .collect())
    }

    fn list_for_operator(&self, op: i32) -> Result<Vec<PriceAdjustment>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.operator_mdoc == op)
            .cloned()
            .collect())
    }

    fn list_for_today(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn list(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn search(
        &self,
        limit: i32,
        offset: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(PriceAdjustment, String, String)>, AppError> {
        let guard = self.store.lock().unwrap();

        let mut adjustments: Vec<(PriceAdjustment, String, String)> = guard
            .iter()
            .filter(|a| {
                // Date filter
                let date_match = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .is_none_or(|parsed| a.created_at.is_some_and(|dt| dt.date() == parsed));

                // Search filter
                let search_match = search.as_ref().is_none_or(|s| {
                    let s = s.as_str();
                    a.upc.contains(s) || a.operator_mdoc.to_string().contains(s)
                });

                date_match && search_match
            })
            .cloned()
            .map(|a| (a, String::new(), String::new()))
            .collect();

        // Sort newest first
        adjustments.sort_by(|(a, _, _), (b, _, _)| b.created_at.cmp(&a.created_at));

        let start = offset as usize;
        let end = (start + limit as usize).min(adjustments.len());
        Ok(adjustments.get(start..end).unwrap_or(&[]).to_vec())
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError> {
        let guard = self.store.lock().unwrap();

        let count = guard
            .iter()
            .filter(|a| {
                let date_match = date
                    .as_ref()
                    .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                    .is_none_or(|parsed| a.created_at.is_some_and(|dt| dt.date() == parsed));

                let search_match = search.as_ref().is_none_or(|s| {
                    let s = s.as_str();
                    a.upc.contains(s) || a.operator_mdoc.to_string().contains(s)
                });

                date_match && search_match
            })
            .count();

        Ok(count as i32)
    }
}
