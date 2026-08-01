//! 通用 DTO：`PageQuery` / `PageResult<T>` / `ApiResponse<T>`。

use axum::extract::Query;
use serde::{Deserialize, Serialize};

/// 通用分页请求参数。
///
/// 直接挂在 axum `Query` 提取器上时，`#[serde(default = "...")]` 会让用户
/// 不传 `page` / `page_size` 也合法。`normalize()` 会进一步把 0 / 越界值
/// 修正到默认值（见其文档）。
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
    /// 修正非法分页参数：`page == 0 → 1`，`page_size == 0 || > 100 → 20`。
    /// 防御性 normalize，防止用户传 0 或超大值触发 SQL `OFFSET` 错误或
    /// 性能问题。
    pub fn normalize(mut self) -> Self {
        if self.page == 0 {
            self.page = 1;
        }
        if self.page_size == 0 || self.page_size > 100 {
            self.page_size = 20;
        }
        self
    }

    /// SQL `OFFSET` 值，等价于 `(page - 1) * page_size`，使用 `saturating_sub`
    /// 避免 `page == 0` 时下溢（虽然 `normalize` 已经挡住，但此处做兜底）。
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
    /// 构造空分页结果（用于零查询的边界场景）。
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
///
/// 所有 HTTP handler 返回值都用 `ApiResponse<T>` 包裹，约定：
/// * `code == 0` + `data: Some(T)` 表示成功
/// * `code != 0` + `data: None` 表示业务错误（HTTP 状态码独立）
///
/// 序列化时 `data: None` 会跳过（见 `skip_serializing_if`），错误体更紧凑。
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 构造成功响应：`code = 0`、消息固定 "ok"。
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "ok".to_owned(),
            data: Some(data),
        }
    }

    /// 构造错误响应（`T` 强制为 `()`，无法携带数据）。
    pub fn error(code: i32, message: String) -> ApiResponse<()> {
        ApiResponse {
            code,
            message,
            data: None,
        }
    }
}

/// 从 `axum::extract::Query<PageQuery>` 中规范化分页参数。
///
/// service 层调这个而非直接调 `q.normalize()`，让 query string 路径一致。
pub fn normalized(q: Query<PageQuery>) -> PageQuery {
    q.0.normalize()
}
