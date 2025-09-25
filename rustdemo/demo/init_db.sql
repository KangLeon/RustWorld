-- 创建数据库 (如果不存在)
-- 注意: 这需要以 postgres 用户身份运行
-- CREATE DATABASE demo_db;

-- 使用数据库
\c demo_db

-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 插入示例数据 (可选)
INSERT INTO users (username, email) VALUES 
    ('admin', 'admin@example.com'),
    ('test_user', 'test@example.com')
ON CONFLICT (username) DO NOTHING;

-- 显示表结构
\d users

-- 显示数据
SELECT * FROM users;
