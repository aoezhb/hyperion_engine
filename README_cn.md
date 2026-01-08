# Hyperion Engine (海伯利安引擎)

> **面向去中心化物理基础设施网络 (DePIN) 的自主计算层。**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/aoezhb/hyperion_engine)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Powered%20by-Rust-orange)](https://www.rust-lang.org/)

## 🌌 概述

**Hyperion Engine** 是一款工业级、高性能的节点客户端，旨在为下一代去中心化网络提供动力。该引擎采用 **Rust** 语言构建，确保了极高的内存安全性和并发性能。它是 DePIN 硬件的“大脑”与“双手”，负责在复杂的网络环境中调度计算、存储和网络资源，并提供可靠的加密完整性。

Hyperion 专为 7x24 小时自主运行而设计，即使在恶劣的网络环境下也能保持稳定，是去中心化物理网络的底层基石。

## 🏗 技术架构

Hyperion 采用了精密的 4 层模块化架构，确保了系统的可扩展性和关注点隔离。

```mermaid
graph TD
    subgraph "Hyperion Node Client"
        A[网络与通信层] <--> B(核心编排层);
        B --> C(硬件抽象层 HAL);
        C --> D(执行与验证层);
    end
```

- **Layer 1: 网络与通信**: 基于 Libp2p 的加密网格网络。
- **Layer 2: 核心编排**: 异步任务调度与全局状态管理。
- **Layer 3: 硬件抽象 (HAL)**: 跨平台的资源探测（支持 GPU/NPU/CPU）。
- **Layer 4: 执行与验证**: 
    - 可插拔的运行时（支持 AI 推理、渲染、ZK 证明）。
    - **PoPW (物理工作量证明)**: 内置 TEE (可信执行环境) 与 ZK-SNARK 接口，确保计算的可验证性。

## 🚀 核心特性

- **🚀 原生 Rust 性能**: 零成本抽象与内存安全，无垃圾回收延迟。
- **🛡️ 物理工作量证明 (PoPW)**: 通过加密证明确保硬件真实性及任务完成度。
- **🔌 模块化执行引擎**: 无需重构核心即可动态更换计算引擎（如用于 AI 的 Candle 或通用计算的 Wasmtime）。
- **🌐 自主 P2P 发现**: 鲁棒的对等点发现与基于 DHT 的路由机制。

## 🛠 快速上手

### 环境要求

- Rust (最新稳定版)
- CMake (用于编译原生依赖)
- NVIDIA 驱动 (可选，用于 GPU 加速功能)

### 安装

```bash
git clone https://github.com/aoezhb/hyperion_engine.git
cd hyperion_engine
cargo build --release
```

### 运行节点

**标准模式 (真实网络)**
```bash
cargo run --bin hyperion_engine
```

**演示模式 (模拟环境)**
使用 `--demo` 参数运行，可模拟任务分配、计算过程及奖励收益，无需连接真实网络。非常适合产品演示。
```bash
cargo run -- --demo
```

## 📜 许可证

本项目采用 Apache License 2.0 许可证。详情请参阅 `LICENSE` 文件。

