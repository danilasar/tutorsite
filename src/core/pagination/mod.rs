use crate::core::models::DbError;

enum ItemsOrdering {
    Ascending,
    Descending
}

struct PaginationSettings {
    per_page: i32,
    ordering: ItemsOrdering
}

struct PaginationInfo {
    pages_count: i32,
    items_count: i32,
    current_page: Option<i32>,
    settings: PaginationSettings
}

enum PaginationError {
    PageRequired, // требуется номер страницы, а передан None
    PageNotFound, // некорректный номер страницы
    Database(DbError) // ошибка базы данных
}

trait GetPage {
    async fn get_pages_count(settings:PaginationSettings) -> Result<i32, PaginationError>;
    async fn get_paginated_items(info:PaginationInfo) -> Result<Vec<Self>, PaginationError>;
}