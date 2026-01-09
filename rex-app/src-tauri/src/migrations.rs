use tauri_plugin_sql::{Migration, MigrationKind};

pub fn migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "add_rex_collection_tables",
            sql: "
                -- add_fs_mtime_to_rex_collection_files_table

                CREATE TABLE IF NOT EXISTS rex_collection (
                    id INTEGER PRIMARY KEY,
                    path TEXT NOT NULL
                );

                CREATE UNIQUE INDEX rex_collection_idx_path ON rex_collection(path);

                CREATE TABLE IF NOT EXISTS rex_collection_files (
                    id INTEGER PRIMARY KEY,
                    collection_id INTEGER,

                    fs_path TEXT NOT NULL,
                    fs_size INTEGER NOT NULL,
                    fs_mtime INTEGER,

                    inner_path TEXT NOT NULL DEFAULT '',
                    inner_size INTEGER,

                    fs_md5 TEXT,
                    inner_md5 TEXT,
                    rcheevos_hash TEXT,

                    last_scanned DATETIME DEFAULT CURRENT_TIMESTAMP,

                    FOREIGN KEY (collection_id) REFERENCES rex_collection(id) ON DELETE CASCADE
                );


                -- rex_collection_files

                CREATE UNIQUE INDEX rex_collection_files_idx_path ON rex_collection_files(fs_path, inner_path);
                CREATE INDEX IF NOT EXISTS idx_fs_md5 ON rex_collection_files(fs_md5);
                CREATE INDEX IF NOT EXISTS idx_inner_md5 ON rex_collection_files(inner_md5);
            ",
            kind: MigrationKind::Up,
        },
    ]
}
