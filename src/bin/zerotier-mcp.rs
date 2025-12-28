use zerotier_sdk_rust_mcp::{Client, McpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取本地 API Token（可选，默认自动读取系统 authtoken.secret）
    let local_token = std::env::var("ZEROTIER_LOCAL_TOKEN").ok();
    
    // 从环境变量获取 Central API Token（可选）
    let central_token = std::env::var("ZEROTIER_CENTRAL_TOKEN").ok();

    // 配置本地客户端
    let local_client = if let Some(token) = local_token {
        Client::with_token(token)
    } else {
        Client::new()
    };

    let mut server = McpServer::new().with_local_client(local_client);
    
    if let Some(token) = central_token {
        server = server.with_central_token(token);
    }

    server.serve_stdio().await?;
    Ok(())
}
