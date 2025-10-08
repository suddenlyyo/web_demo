# 部署和运维指南

## 1. 环境要求

### 1.1 系统要求
- Linux/macOS/Windows（支持Rust的系统）
- 至少2GB内存
- 至少100MB磁盘空间

### 1.2 软件依赖
- Rust 1.70+
- MySQL 5.7+
- 对应数据库客户端库

## 2. 部署步骤

### 2.1 克隆代码
```bash
git clone <repository-url>
cd web_demo
```

### 2.2 配置数据库
1. 创建MySQL数据库
2. 在项目根目录执行`sql/demo.sql`脚本初始化表结构
3. 配置`config.toml`或设置`DATABASE_URL`环境变量

### 2.3 编译项目
```bash
# 编译指定框架和ORM实现
# 以Axum + SQLx为例
cd axum_demo
cargo build --release --no-default-features --features sqlx_impl
```

### 2.4 运行服务
```bash
# 设置环境变量
export DATABASE_URL=mysql://user:password@localhost/database
export HOST=0.0.0.0
export PORT=8000

# 运行服务
./target/release/axum_demo
```

## 3. 配置说明

### 3.1 数据库配置
支持两种配置方式：
1. 环境变量：`DATABASE_URL`
2. 配置文件：`config.toml`

示例config.toml：
```toml
# 数据库配置
[database]
url = "mysql://root:Lv997945%21@localhost:3306/demo"

# Diesel数据库连接池配置（可选）
# 如果未配置，则使用默认值
[database.diesel]
# max_size = 15            # 连接池最大连接数（默认值：10）
# min_idle = 3             # 连接池最小空闲连接数（默认值：None）
# connection_timeout = 5   # 获取连接的超时时间（秒）（默认值：30）
# max_lifetime = 1800      # 连接池中连接的最大存活时间（秒）（默认值：1800）
# idle_timeout = 600       # 连接池中空闲连接的超时时间（秒）（默认值：600）
# test_on_check_out = true # 借出连接时测试其有效性（默认值：true）

# SeaORM数据库连接池配置（可选）
# 如果未配置，则使用默认值
[database.seaorm]
# max_connections = 100    # 连接池最大连接数（默认值：10）
# min_connections = 5      # 连接池最小连接数（默认值：None）
# connect_timeout = 8      # 连接超时时间（秒）（默认值：无穷大）
# acquire_timeout = 8      # 获取连接的超时时间（秒）（默认值：30）
# idle_timeout = 8         # 连接池中空闲连接的超时时间（秒）（默认值：无穷大）
# max_lifetime = 8         # 连接池中连接的最大存活时间（秒）（默认值：无穷大）

# SQLx数据库连接池配置（可选）
# 如果未配置，则使用默认值
[database.sqlx]
# max_connections = 10     # 连接池最大连接数（默认值：10）
# min_connections = 0      # 连接池最小连接数（默认值：None）
# acquire_timeout = 30     # 获取连接的超时时间（秒）（默认值：30）
# idle_timeout = 600       # 连接池中空闲连接的超时时间（秒）（默认值：600）
# max_lifetime = 1800      # 连接池中连接的最大存活时间（秒）（默认值：1800）
```

### 3.2 服务配置

不同框架有不同的配置方式：

#### Axum/Actix Web
- `HOST`：服务监听地址，默认127.0.0.1
- `PORT`：服务监听端口，默认8000

#### Rocket
支持三种配置方式：
1. 默认配置
2. Rocket.toml 文件
3. 环境变量（以 ROCKET_ 为前缀）
   - `ROCKET_ADDRESS`：服务监听地址
   - `ROCKET_PORT`：服务监听端口

## 4. 不同框架的部署

### 4.1 Axum
```bash
cd axum_demo

# 使用 SQLx 实现
cargo run --no-default-features --features sqlx_impl

# 使用 Diesel 实现
cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cargo run --no-default-features --features seaorm_impl
```

### 4.2 Actix Web
```bash
cd actix_web_demo

# 使用 SQLx 实现
cargo run --no-default-features --features sqlx_impl

# 使用 Diesel 实现
cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cargo run --no-default-features --features seaorm_impl
```

### 4.3 Rocket
```bash
cd rocket_demo

# 使用 SQLx 实现
cargo run --no-default-features --features sqlx_impl

# 使用 Diesel 实现
cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cargo run --no-default-features --features seaorm_impl
```

## 5. 监控和日志

### 5.1 日志查看
```bash
# 查看日志文件（如果配置了日志输出到文件）
tail -f /var/log/web_demo.log
```

### 5.2 健康检查
访问 `http://host:port/` 进行健康检查，应当返回欢迎信息。

## 6. 常见问题处理

### 6.1 数据库连接失败
检查：
1. `DATABASE_URL`是否正确设置
2. 数据库服务是否运行
3. 网络连接是否正常
4. 数据库用户是否有足够权限

### 6.2 编译错误
检查：
1. Rust版本是否符合要求（1.70+）
2. 是否安装了必要的依赖库
3. 是否正确指定了特性参数

### 6.3 运行时错误
检查：
1. 端口是否被占用
2. 配置文件路径是否正确
3. 环境变量是否正确设置

## 7. 性能优化建议

1. 使用release模式编译：`cargo build --release`
2. 根据负载调整数据库连接池大小
3. 合理设置HTTP服务器的工作线程数
4. 使用反向代理（如Nginx）处理静态文件和负载均衡