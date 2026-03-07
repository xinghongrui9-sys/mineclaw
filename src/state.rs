use std::sync::Arc;
use tokio::sync::Mutex;

use crate::checkpoint::CheckpointManager;
use crate::config::Config;
use crate::llm::LlmProvider;
use crate::mcp::{McpServerManager, ToolExecutor};
use crate::models::SessionRepository;
use crate::tool_coordinator::ToolCoordinator;
use crate::tools::LocalToolRegistry;
use agentfs::AgentFS;

#[derive(Clone)]
pub struct AppState {
    pub session_repo: SessionRepository,
    pub llm_provider: Arc<dyn LlmProvider>,
    pub mcp_server_manager: Arc<Mutex<McpServerManager>>,
    pub tool_executor: ToolExecutor,
    pub tool_coordinator: Arc<ToolCoordinator>,
    pub local_tool_registry: Arc<LocalToolRegistry>,
    pub config: Arc<Config>,
    pub agent_fs: Arc<AgentFS>,
    pub checkpoint_manager: Arc<CheckpointManager>,
}

impl AppState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session_repo: SessionRepository,
        llm_provider: Arc<dyn LlmProvider>,
        mcp_server_manager: Arc<Mutex<McpServerManager>>,
        tool_executor: ToolExecutor,
        tool_coordinator: ToolCoordinator,
        local_tool_registry: Arc<LocalToolRegistry>,
        config: Arc<Config>,
        agent_fs: Arc<AgentFS>,
        checkpoint_manager: Arc<CheckpointManager>,
    ) -> Self {
        Self {
            session_repo,
            llm_provider,
            mcp_server_manager,
            tool_executor,
            tool_coordinator: Arc::new(tool_coordinator),
            local_tool_registry,
            config,
            agent_fs,
            checkpoint_manager,
        }
    }
}
