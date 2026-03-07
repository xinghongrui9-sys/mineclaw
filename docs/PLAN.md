# MineClaw 项目计划

## 项目概述

MineClaw 是一个基于 CLAW 定义的 AI 智能体工具，旨在提供一个随时随地可用的 coding agent。它通过终端调用静态分析工具来检查问题，实现 checkpoint 机制让它可以自己回退，使用 Ollama 本地模型作为 subagent 帮助查询陌生库用法，并且有总结和存储经验的能力。

## 已完成阶段

### Phase 1: 基础消息流转 ✅
- [详细文档](./PHASE1.md)
- 项目基础架构
- 配置管理
- 核心数据模型
- LLM 客户端
- Web API 层
- 请求/响应日志

### Phase 2: MCP 集成 ✅
- [详细文档](./PHASE2.md)
- 数据模型和配置扩展
- MCP 协议和基础客户端
- 工具调用功能
- 扩展 LLM 支持工具调用
- 集成工具调用循环
- SSE 流式模式
- API 扩展和管理功能

### Phase 3: 本地工具与 Checkpoint 集成 🔄
- [详细文档](./PHASE3_inprogress.md)
- API Key 加密存储
- 终端工具
- 文件工具集
- Checkpoint 集成（与文件工具深度集成）
- 配置文件动态修改

## 未来计划

### Phase 4: Jupyter Notebook 支持
- [详细文档](./PHASE4.md) (待创建)
- Jupyter Notebook 文件解析
- Notebook 单元格编辑和执行
- Notebook 状态管理
- 代码单元格输出捕获和显示

### Phase 5: 工作区规则自动提取
- [详细文档](./PHASE5.md) (待创建)
- 工作区 Markdown 文件扫描
- 规则提取和解析
- 规则优先级管理
- 自动规则更新机制

### Phase 6: 工作流系统
- [详细文档](./PHASE6.md) (待创建)
- 工作流定义语言
- 工作流执行引擎
- 工作流版本控制
- 工作流模板库

### Phase 7: Subagent 系统
- [详细文档](./PHASE7.md) (待创建)
- Ollama 本地模型集成
- Subagent 任务分配
- Subagent 协作机制
- 子任务结果聚合

### Phase 8: 经验学习系统
- [详细文档](./PHASE8.md) (待创建)
- 任务完成后的经验总结
- 经验库存储和检索
- 经验相似度匹配
- 经验应用建议

### Phase 9: 状态监控与远程显示
- [详细文档](./PHASE9.md) (待创建)
- Agent 状态追踪
- 任务进度报告
- 移动端 API 适配
- 状态可视化

### Phase 10: Telegram Bot 集成
- [详细文档](./PHASE10.md) (待创建)
- Telegram Bot API 集成
- 消息接收和处理
- 任务启动和控制
- 结果推送和通知

### Phase 11: 沙箱安全系统
- [详细文档](./PHASE11.md) (待创建)
- 文件系统沙箱
- 进程隔离
- 网络访问控制
- 资源使用限制

### Phase 12: OpenViking 集成（上下文数据库）
- [详细文档](./PHASE12.md) (待创建)
- OpenViking 上下文数据库集成
- 文件系统范式的上下文管理
- L0/L1/L2 分层上下文加载（降低 Token 消耗）
- 目录递归检索策略
- 可视化检索轨迹
- 自动会话管理和记忆自迭代
- OpenClaw 记忆插件兼容

### Phase 13: EvoMap 集成（AI 自我进化基础设施）
- [详细文档](./PHASE13.md) (待创建)
- GEP (Genome Evolution Protocol) 集成
- Gene（可复用策略模板）和 Capsule（经验证的修复方案）管理
- GDI (Global Desirability Index) 评分系统
- Agent 之间的能力共享和继承
- 跨生态系统兼容（OpenClaw、Manus、HappyCapy、Cursor 等）
- AI 驱动的质量审查机制

### Phase 14: Flutter 前端开发
- [详细文档](./PHASE14.md) (待创建)
- 日常任务管理界面
- 代码编辑器集成（ACP）
- 状态追踪仪表板
- 自我反馈工作流界面

## 技术栈

### 后端
- Rust (主要语言)
- Axum (Web 框架)
- Tokio (异步运行时)
- agentfs (Checkpoint 管理)
- agentsql (数据持久化)
- OpenViking (上下文数据库)
- EvoMap (AI 自我进化协议)

### 前端
- Flutter (跨平台 UI)

### AI/ML
- OpenAI API 兼容接口
- Ollama (本地模型)
- OpenViking (上下文管理)
- EvoMap (能力进化)

## 核心特性路线图

- [x] 基础对话 API
- [x] 多轮对话
- [x] MCP 工具调用
- [x] SSE 流式输出
- [ ] 文件读写工具集
- [ ] Checkpoint 机制
- [ ] API Key 加密
- [ ] 终端工具
- [ ] 配置动态修改
- [ ] Jupyter Notebook 支持
- [ ] 工作区规则自动提取
- [ ] 工作流系统
- [ ] Subagent 系统
- [ ] 经验学习系统
- [ ] 状态监控 API
- [ ] Telegram Bot
- [ ] 沙箱安全
- [ ] OpenViking 集成（上下文数据库）
- [ ] EvoMap 集成（AI 自我进化）
- [ ] Flutter 前端

## 开发原则

1. **Baby Steps™**: 每次只做一个小的、有意义的改变
2. **The Process is the Product**: 过程和结果同样重要
3. **Incremental Validation**: 每一步都要验证
4. **Documentation First**: 先写文档再实现

## 相关项目

- **OpenViking**: AI 智能体的上下文数据库，采用文件系统范式统一管理记忆、资源和技能
- **EvoMap**: AI 自我进化基础设施，通过 GEP 协议实现 agent 间能力的共享、验证和继承

## 项目地址

[https://github.com/Cryptocho/mineclaw](https://github.com/Cryptocho/mineclaw)
