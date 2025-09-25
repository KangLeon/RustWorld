#!/bin/bash

# 环境设置脚本
echo "🚀 设置 Rust Demo 项目环境..."

# 检查是否存在 .env 文件
if [ ! -f ".env" ]; then
    echo "📋 创建 .env 文件..."
    cp .env.example .env
    echo "✅ .env 文件已创建，请编辑其中的配置"
else
    echo "✅ .env 文件已存在"
fi

# 提示用户配置数据库
echo ""
echo "📝 请编辑 .env 文件并配置以下变量："
echo "   - DATABASE_URL: PostgreSQL 连接字符串"
echo "   - SERVER_HOST: 服务器主机 (默认: 127.0.0.1)"
echo "   - SERVER_PORT: 服务器端口 (默认: 8080)"
echo ""

# 检查 PostgreSQL 是否安装
if command -v psql &> /dev/null; then
    echo "✅ PostgreSQL 已安装"
else
    echo "⚠️  PostgreSQL 未安装，请先安装："
    echo "   macOS: brew install postgresql"
    echo "   Ubuntu: sudo apt-get install postgresql"
fi

echo ""
echo "🔧 常用命令："
echo "   cargo check    # 检查代码"
echo "   cargo build    # 构建项目"
echo "   cargo run      # 运行项目"
echo ""
echo "📚 API 端点："
echo "   POST /api/users    # 创建用户"
echo "   GET  /api/users    # 获取用户列表"
