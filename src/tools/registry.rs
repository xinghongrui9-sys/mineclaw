//! 本地工具注册表
//!
//! 管理所有本地工具，提供工具查找和调用功能。

use super::{LocalTool, ToolContext};
use crate::error::{Error, Result};
use crate::models::Tool;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

// ==================== LocalToolRegistry ====================

/// 本地工具注册表
pub struct LocalToolRegistry {
    tools: HashMap<String, Arc<dyn LocalTool>>,
}

impl LocalToolRegistry {
    /// 创建一个新的本地工具注册表
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// 注册一个工具
    pub fn register(&mut self, tool: Arc<dyn LocalTool>) {
        let name = tool.name().to_string();
        debug!(tool_name = %name, "Registering local tool");
        self.tools.insert(name, tool);
    }

    /// 获取所有工具列表
    pub fn list_tools(&self) -> Vec<Tool> {
        self.tools
            .values()
            .map(|t| Tool {
                name: t.name().to_string(),
                description: t.description().to_string(),
                input_schema: t.input_schema(),
            })
            .collect()
    }

    /// 检查工具是否存在
    pub fn has_tool(&self, tool_name: &str) -> bool {
        self.tools.contains_key(tool_name)
    }

    /// 获取工具定义
    pub fn get_tool(&self, tool_name: &str) -> Option<Tool> {
        self.tools.get(tool_name).map(|t| Tool {
            name: t.name().to_string(),
            description: t.description().to_string(),
            input_schema: t.input_schema(),
        })
    }

    /// 调用工具
    pub async fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
        context: ToolContext,
    ) -> Result<Value> {
        let tool = self
            .tools
            .get(tool_name)
            .ok_or_else(|| Error::LocalToolNotFound(tool_name.to_string()))?;

        tool.call(arguments, context)
            .await
            .map_err(|e| Error::LocalToolExecution {
                tool: tool_name.to_string(),
                message: e.to_string(),
            })
    }

    /// 清空注册表
    pub fn clear(&mut self) {
        self.tools.clear();
    }
}

impl Default for LocalToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::models::Session;
    use async_trait::async_trait;
    use serde_json::json;

    struct TestTool;

    #[async_trait]
    impl LocalTool for TestTool {
        fn name(&self) -> &str {
            "test_tool"
        }

        fn description(&self) -> &str {
            "A test tool"
        }

        fn input_schema(&self) -> Value {
            json!({
                "type": "object",
                "properties": {
                    "input": { "type": "string" }
                }
            })
        }

        async fn call(&self, arguments: Value, _context: ToolContext) -> Result<Value> {
            let input = arguments["input"].as_str().unwrap_or("");
            Ok(json!({ "output": format!("Hello, {}!", input) }))
        }
    }

    #[tokio::test]
    async fn test_registry_new() {
        let registry = LocalToolRegistry::new();
        assert!(registry.tools.is_empty());
    }

    #[tokio::test]
    async fn test_register_tool() {
        let mut registry = LocalToolRegistry::new();
        registry.register(Arc::new(TestTool));

        assert!(registry.has_tool("test_tool"));
    }

    #[tokio::test]
    async fn test_list_tools() {
        let mut registry = LocalToolRegistry::new();
        registry.register(Arc::new(TestTool));

        let tools = registry.list_tools();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "test_tool");
    }

    #[tokio::test]
    async fn test_call_tool() {
        let mut registry = LocalToolRegistry::new();
        registry.register(Arc::new(TestTool));

        let context = ToolContext::new(Session::new(), Arc::new(Config::default()));

        let result = registry
            .call_tool("test_tool", json!({"input": "World"}), context)
            .await
            .unwrap();

        assert_eq!(result["output"], "Hello, World!");
    }

    #[tokio::test]
    async fn test_call_nonexistent_tool() {
        let registry = LocalToolRegistry::new();

        let context = ToolContext::new(Session::new(), Arc::new(Config::default()));

        let result = registry.call_tool("nonexistent", json!({}), context).await;

        assert!(matches!(result, Err(Error::LocalToolNotFound(_))));
    }
}
