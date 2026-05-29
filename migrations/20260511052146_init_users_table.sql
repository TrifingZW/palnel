-- 招生老师（控制台用户）表
CREATE TABLE IF NOT EXISTS user (
    id         INTEGER PRIMARY KEY NOT NULL,  -- 自增主键
    username   TEXT NOT NULL UNIQUE,       -- 登录用户名（唯一）
    password   TEXT NOT NULL,              -- 密码哈希（Argon2 / bcrypt）
    role       TEXT NOT NULL DEFAULT 'teacher'   -- 角色：admin / teacher
);

-- 索引：按用户名快速查找（登录场景）
CREATE INDEX IF NOT EXISTS idx_user_username ON user(username);

-- 索引：按角色筛选
CREATE INDEX IF NOT EXISTS idx_user_role ON user(role);

-- 默认管理员账户（开发/测试用，生产环境请修改密码）
INSERT OR IGNORE INTO user (username, password, role)
VALUES ('admin', 'admin123', 'admin');
