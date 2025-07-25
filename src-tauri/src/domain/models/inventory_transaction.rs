use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InventoryTransaction {
    pub id: Option<i32>, // auto‑assigned PK
    pub upc: String,
    pub quantity_change: i32,
    pub operator_mdoc: i32,
    pub customer_mdoc: Option<i32>,
    pub ref_order_id: Option<i32>,
    pub reference: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
