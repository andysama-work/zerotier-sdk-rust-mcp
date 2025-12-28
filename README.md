# ZeroTier SDK for Rust

ZeroTier API 的 Rust SDK，支持 MCP（Model Context Protocol）集成。

## 功能特性

- **本地 Service API** (`client`): 管理本地 ZeroTier 节点（localhost:9993）
- **云端 Central API** (`central`): 管理云端网络（api.zerotier.com）
- **MCP 服务**: 提供 MCP 工具集成

## 安装

### 一键安装

**Linux / macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/fromsko/zerotier-sdk-rust-mcp/main/scripts/install.sh | bash
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/fromsko/zerotier-sdk-rust-mcp/main/scripts/install.ps1 | iex
```

### 手动下载

从 [Releases](https://github.com/fromsko/zerotier-sdk-rust-mcp/releases) 页面下载对应平台的二进制文件。

### 从源码构建

```bash
git clone https://github.com/fromsko/zerotier-sdk-rust-mcp.git
cd zerotier-sdk-rust-mcp
cargo build --release
```

## MCP 配置

```json
{
  "mcpServers": {
    "zerotier": {
      "command": "zerotier-mcp",
      "env": {
        "ZEROTIER_LOCAL_TOKEN": "your_local_token",
        "ZEROTIER_CENTRAL_TOKEN": "your_central_token"
      }
    }
  }
}
```

### 环境变量说明

| 变量名 | 必填 | 说明 |
|--------|------|------|
| `ZEROTIER_LOCAL_TOKEN` | 否 | 本地 ZeroTier 服务 API Token，用于管理本地节点。如不设置，将自动从系统默认位置读取 |
| `ZEROTIER_CENTRAL_TOKEN` | 否 | 云端 Central API Token，用于管理云端网络 |

### 获取 Token

**本地 Token（authtoken.secret）：**

- Windows: `C:\ProgramData\ZeroTier\One\authtoken.secret`（需管理员权限读取）
- macOS: `~/Library/Application Support/ZeroTier/authtoken.secret`
- Linux: `/var/lib/zerotier-one/authtoken.secret`

```powershell
# Windows (以管理员身份运行 PowerShell)
Get-Content "C:\ProgramData\ZeroTier\One\authtoken.secret"
```

```bash
# Linux/macOS
sudo cat /var/lib/zerotier-one/authtoken.secret
```

**云端 Token：** [my.zerotier.com](https://my.zerotier.com) → Account → API Access Tokens

## MCP 工具列表

### 本地 API 工具

| 工具名 | 描述 |
|--------|------|
| `zt_status` | 获取本地节点状态 |
| `zt_networks` | 列出已加入的网络 |
| `zt_join` | 加入网络 |
| `zt_leave` | 离开网络 |
| `zt_peers` | 列出所有 Peers |

### 云端 API 工具

| 工具名 | 描述 |
|--------|------|
| `zt_central_networks` | 列出云端网络 |
| `zt_central_members` | 列出网络成员 |
| `zt_central_authorize` | 授权成员 |
| `zt_central_authorize_with_ip` | 授权成员并指定 IP |
| `zt_central_deauthorize` | 取消授权 |

## 作为库使用

```rust
use zerotier_sdk_rust_mcp::{Client, Central};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 本地节点管理
    let local = Client::new();
    let status = local.status().await?;
    println!("节点地址: {}", status.address);

    // 云端管理
    let cloud = Central::new("your_api_token");
    let networks = cloud.networks().list().await?;
    
    Ok(())
}
```

## 模块结构

```
zerotier-sdk-rust-mcp/
├── src/
│   ├── lib.rs           # 库入口
│   ├── client/          # 本地 Service API
│   ├── central/         # 云端 Central API
│   ├── mcp/             # MCP 服务
│   └── bin/
│       └── zerotier-mcp.rs
├── scripts/
│   ├── install.sh       # Linux/macOS 安装脚本
│   └── install.ps1      # Windows 安装脚本
└── .github/workflows/
    └── release.yml      # 自动发布工作流
```

## License

MIT
