use crate::common::error::AppError;
use crate::domain::models::Product;
use crate::domain::repos::ProductRepoTrait;
use std::sync::Mutex;

pub struct MockProductRepo {
    store: Mutex<Vec<Product>>,
}

impl MockProductRepo {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MockProductRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductRepoTrait for MockProductRepo {
    fn get_price(&self, upc: String) -> Result<i32, AppError> {
        let guard = self.store.lock().unwrap();
        let p = guard
            .iter()
            .find(|p| p.upc == upc)
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", upc)))?;
        Ok(p.price)
    }

    fn get_by_upc(&self, upc: String) -> Result<Option<Product>, AppError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.upc == upc)
            .cloned())
    }

    fn create(&self, p: &Product) -> Result<(), AppError> {
        self.store.lock().unwrap().push(p.clone());
        Ok(())
    }

    fn update_by_upc(&self, p: &Product) -> Result<(), AppError> {
        let mut v = self.store.lock().unwrap();
        if let Some(elem) = v.iter_mut().find(|e| e.upc == p.upc) {
            *elem = p.clone();
        }
        Ok(())
    }

    fn update_by_upc_with_tx(
        &self,
        p: &crate::domain::models::Product,
        _tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError> {
        self.update_by_upc(p)
    }

    fn list(&self) -> Result<Vec<Product>, AppError> {
        Ok(self.store.lock().unwrap().clone())
    }

    fn search(
        &self,
        desc_like: Option<String>,
        category: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(Product, i64)>, AppError> {
        let guard = self.store.lock().unwrap();
        let mut products: Vec<Product> = guard
            .iter()
            .filter(|p| {
                let desc_match = desc_like
                    .as_ref()
                    .map(|s| p.desc.contains(s))
                    .unwrap_or(true);
                let cat_match = category.as_ref().map(|c| &p.category == c).unwrap_or(true);
                desc_match && cat_match
            })
            .cloned()
            .collect();

        products.sort_by(|a, b| b.added.cmp(&a.added));

        let start = offset as usize;
        let end = std::cmp::min(start + limit as usize, products.len());
        let sliced = if start < products.len() {
            &products[start..end]
        } else {
            &[]
        };

        let v = sliced.iter().cloned().map(|p| (p, 0_i64)).collect();

        Ok(v)
    }

    fn count(&self, desc_like: Option<String>, category: Option<String>) -> Result<u32, AppError> {
        let guard = self.store.lock().unwrap();
        let count = guard
            .iter()
            .filter(|p| {
                let desc_match = desc_like
                    .as_ref()
                    .map(|s| p.desc.contains(s))
                    .unwrap_or(true);
                let cat_match = category.as_ref().map(|c| &p.category == c).unwrap_or(true);
                desc_match && cat_match
            })
            .count();
        Ok(count as u32)
    }
}
