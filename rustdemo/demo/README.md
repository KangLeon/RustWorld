# Rust Demo 项目

一个使用 Rust、Actix-web 和 PostgreSQL 构建的简单 Web API 项目。

## 🚀 快速开始

### 1. 环境设置

运行环境设置脚本：

```bash
./setup_env.sh
```

### 2. 配置环境变量

编辑 `.env` 文件：

```bash
# 数据库配置
DATABASE_URL=postgresql://用户名:密码@localhost:5432/数据库名

# 服务器配置
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# 日志级别
RUST_LOG=debug
```

### 3. 安装 PostgreSQL

**macOS:**
```bash
brew install postgresql
brew services start postgresql
```

**Ubuntu:**
```bash
sudo apt-get install postgresql postgresql-contrib
sudo systemctl start postgresql
```

### 4. 创建数据库

```bash
# 连接到 PostgreSQL
psql -U postgres

# 创建数据库
CREATE DATABASE demo_db;

# 创建用户表
\c demo_db
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 5. 运行项目

```bash
# 检查代码
cargo check

# 构建项目
cargo build

# 运行项目
cargo run
```

## 📚 API 文档

### 创建用户

```bash
curl -X POST http://127.0.0.1:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com"
  }'
```

### 获取用户列表

```bash
curl http://127.0.0.1:8080/api/users
```

## 🛠️ 开发

### 项目结构

```
src/
├── main.rs          # 主入口文件
├── config.rs        # 配置管理
├── db/              # 数据库模块
│   └── mod.rs
├── handlers/        # API 处理器
│   └── mod.rs
└── models/          # 数据模型
    └── mod.rs
```

### 环境变量说明

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `DATABASE_URL` | PostgreSQL 连接字符串 | 必需 |
| `SERVER_HOST` | 服务器监听地址 | 127.0.0.1 |
| `SERVER_PORT` | 服务器端口 | 8080 |
| `RUST_LOG` | 日志级别 | debug |

## 🔧 常见问题

### 1. 数据库连接失败

确保：
- PostgreSQL 服务正在运行
- 数据库连接字符串正确
- 数据库和用户已创建
- 网络连接正常

### 2. 编译错误

确保：
- Rust 版本 >= 1.70
- 所有依赖已正确安装
- 环境变量已正确设置

### 3. 端口被占用

修改 `.env` 文件中的 `SERVER_PORT` 为其他可用端口。
