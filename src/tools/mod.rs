//! 本地工具模块
//!
//! 提供本地工具的 trait 定义和基础结构。

use crate::config::Config;
use crate::error::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

pub mod filesystem;
pub mod registry;

pub use registry::LocalToolRegistry;

// ==================== ToolContext ====================

/// 工具执行上下文
#[derive(Debug, Clone)]
pub struct ToolContext {
    /// 会话 ID
    pub session_id: String,
    /// 配置
    pub config: Arc<Config>,
}

// ==================== LocalTool ====================

/// 本地工具 trait
#[async_trait]
pub trait LocalTool: Send + Sync {
    /// 获取工具名称
    fn name(&self) -> &str;

    /// 获取工具描述
    fn description(&self) -> &str;

    /// 获取工具输入 schema
    fn input_schema(&self) -> Value;

    /// 调用工具
    async fn call(&self, arguments: Value, context: ToolContext) -> Result<Value>;
}
