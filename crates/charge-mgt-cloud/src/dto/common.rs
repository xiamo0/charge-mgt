use axum::extract::Query;
use serde::{Deserialize, Serialize};

/// 通用分页请求参数。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct PageQuery {
    /// 页码，从 1 开始；默认 1。
    #[serde(default = "default_page")]
    pub page: u64,
    /// 每页数量；默认 20，最大 100。
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    20
}

impl PageQuery {
    pub fn normalize(mut self) -> Self {
        if self.page == 0 {
            self.page = 1;
        }
        if self.page_size == 0 || self.page_size > 100 {
            self.page_size = 20;
        }
        self
    }

    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1)) * self.page_size
    }
}

/// 通用分页结果。
#[derive(Debug, Clone, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

impl<T> PageResult<T> {
    pub fn empty(page: &PageQuery) -> Self {
        Self {
            items: Vec::new(),
            total: 0,
            page: page.page,
            page_size: page.page_size,
        }
    }
}

/// 通用响应外壳。
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "ok".to_owned(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: String) -> ApiResponse<()> {
        ApiResponse {
            code,
            message,
            data: None,
        }
    }
}

/// 从 `axum::extract::Query<PageQuery>` 中规范化分页参数。
pub fn normalized(q: Query<PageQuery>) -> PageQuery {
    q.0.normalize()
}
