use tauri_plugin_sql::{Migration, MigrationKind};

pub fn migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create_rex_collection_files_table",
            sql: "
                CREATE TABLE IF NOT EXISTS rex_collection_files (
                    fs_path TEXT NOT NULL,
                    inner_path TEXT NOT NULL DEFAULT '',

                    fs_size INTEGER NOT NULL,
                    inner_size INTEGER,

                    fs_md5 TEXT,
                    inner_md5 TEXT,
                    rcheevos_hash TEXT,
                    last_scanned DATETIME DEFAULT CURRENT_TIMESTAMP,

                    PRIMARY KEY (fs_path, inner_path)
                );

                CREATE INDEX IF NOT EXISTS idx_fs_md5 ON rex_collection_files(fs_md5);
                CREATE INDEX IF NOT EXISTS idx_inner_md5 ON rex_collection_files(inner_md5);
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add_fs_mtime_to_rex_collection_files_table",
            sql: "
                ALTER TABLE rex_collection_files ADD COLUMN fs_mtime INTEGER;
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "add_fs_mtime_to_rex_collection_files_table",
            sql: "
                CREATE TABLE IF NOT EXISTS rex_collection (
                    id INTEGER PRIMARY KEY,
                    path TEXT NOT NULL
                );

                CREATE UNIQUE INDEX idx_path ON rex_collection(path);
            ",
            kind: MigrationKind::Up,
        },
    ]
}
