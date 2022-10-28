-- Add migration script here
CREATE TABLE IF NOT EXISTS photos
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    base_path       TEXT                NOT NULL,                   -- 文件所在目录
    media_type      TEXT                NOT NULL,                   -- 媒体类型 'video' 'image'
    file_type       TEXT                NOT NULL,                   -- 文件类型 ‘jpg’ 'png' 'mp4'
    file_name       TEXT                NOT NULL,                   -- 文件名
    create_time     TIMESTAMP           NOT NULL,                   -- 创建时间
    hash_code       TEXT                NOT NULL,                   -- 哈希
    file_size       INTEGER             NOT NULL,                   -- 文件大小 byte
    favorite        BOOLEAN             NOT NULL DEFAULT "FALSE",   -- 是否收藏
    width           INTEGER             NOT NULL,                   -- 宽 pixe
    height          INTEGER             NOT NULL,                   -- 高 pixe
    iso             INTEGER             NOT NULL DEFAULT 0,         -- iso 感光度
    camera_model    TEXT                NOT NULL DEFAULT "Unkmown", -- 手机型号
    camera_make     TEXT                NOT NULL DEFAULT "Unkmown", -- 手机类型
    exposure_time   TEXT                NOT NULL DEFAULT "Unkmown", -- 曝光时间
    f_number        TEXT                NOT NULL DEFAULT "Unkmown", -- 光圈
    focal_length    TEXT                NOT NULL DEFAULT "Unkmown"  -- 焦距
);

CREATE TABLE IF NOT EXISTS config
(
    update_time TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 更新时间
    scan_path   TEXT        NOT NULL,                           -- 扫描路径
    thumb_path  TEXT        NOT NULL,                           -- 缩略图路径
    scan_time   TIMESTAMP   DEFAULT NULL                        -- 扫描时间
);
