use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginatedData<T> {
    pub current_page: u64,
    pub total_pages: u64,
    pub absolute_first_page: u64, // Will be 0 or 1, depending on the context
    pub items: Vec<T>,            // The items for the current page
}
