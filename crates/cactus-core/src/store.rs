use crate::paths::AppPaths;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub folder: String,
    pub position: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub visit_count: i64,
    pub last_visit: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRecord {
    pub id: i64,
    pub url: String,
    pub filename: String,
    pub path: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionState {
    pub windows: Vec<SessionWindow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionWindow {
    pub private: bool,
    pub active_tab: usize,
    pub tabs: Vec<String>,
}

pub struct Store {
    conn: Mutex<Connection>,
}

impl Store {
    pub fn open(paths: &AppPaths) -> Result<Self> {
        let conn = Connection::open(paths.db_path())
            .with_context(|| format!("opening db {}", paths.db_path().display()))?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA foreign_keys = ON;
            CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE,
                folder TEXT NOT NULL DEFAULT 'Bar',
                position INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE,
                visit_count INTEGER NOT NULL DEFAULT 1,
                last_visit TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS downloads (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                filename TEXT NOT NULL,
                path TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_history_last ON history(last_visit DESC);
            CREATE INDEX IF NOT EXISTS idx_bookmarks_pos ON bookmarks(folder, position);
            "#,
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn add_bookmark(&self, title: &str, url: &str, folder: &str) -> Result<Bookmark> {
        let now = Utc::now();
        let conn = self.conn.lock().unwrap();
        let position: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(position), -1) + 1 FROM bookmarks WHERE folder = ?1",
                params![folder],
                |r| r.get(0),
            )
            .unwrap_or(0);
        conn.execute(
            "INSERT INTO bookmarks (title, url, folder, position, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(url) DO UPDATE SET title=excluded.title, folder=excluded.folder",
            params![title, url, folder, position, now.to_rfc3339()],
        )?;
        let id = conn.last_insert_rowid();
        Ok(Bookmark {
            id,
            title: title.to_string(),
            url: url.to_string(),
            folder: folder.to_string(),
            position,
            created_at: now,
        })
    }

    pub fn remove_bookmark_by_url(&self, url: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM bookmarks WHERE url = ?1", params![url])?;
        Ok(())
    }

    pub fn list_bookmarks(&self) -> Result<Vec<Bookmark>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, url, folder, position, created_at FROM bookmarks ORDER BY folder, position, id",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(Bookmark {
                id: r.get(0)?,
                title: r.get(1)?,
                url: r.get(2)?,
                folder: r.get(3)?,
                position: r.get(4)?,
                created_at: parse_dt(&r.get::<_, String>(5)?),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn list_bookmark_bar(&self) -> Result<Vec<Bookmark>> {
        Ok(self
            .list_bookmarks()?
            .into_iter()
            .filter(|b| b.folder == "Bar")
            .collect())
    }

    pub fn record_history(&self, title: &str, url: &str) -> Result<()> {
        if url.starts_with("cactus://") || url.starts_with("about:") || url == "about:blank" {
            return Ok(());
        }
        let now = Utc::now().to_rfc3339();
        let title = if title.is_empty() { url } else { title };
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO history (title, url, visit_count, last_visit)
             VALUES (?1, ?2, 1, ?3)
             ON CONFLICT(url) DO UPDATE SET
               title=excluded.title,
               visit_count=visit_count+1,
               last_visit=excluded.last_visit",
            params![title, url, now],
        )?;
        Ok(())
    }

    pub fn list_history(&self, limit: i64) -> Result<Vec<HistoryEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, url, visit_count, last_visit FROM history
             ORDER BY last_visit DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |r| {
            Ok(HistoryEntry {
                id: r.get(0)?,
                title: r.get(1)?,
                url: r.get(2)?,
                visit_count: r.get(3)?,
                last_visit: parse_dt(&r.get::<_, String>(4)?),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn search_history(&self, query: &str, limit: i64) -> Result<Vec<HistoryEntry>> {
        let q = format!("%{query}%");
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, url, visit_count, last_visit FROM history
             WHERE title LIKE ?1 OR url LIKE ?1
             ORDER BY last_visit DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![q, limit], |r| {
            Ok(HistoryEntry {
                id: r.get(0)?,
                title: r.get(1)?,
                url: r.get(2)?,
                visit_count: r.get(3)?,
                last_visit: parse_dt(&r.get::<_, String>(4)?),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn clear_history(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub fn add_download(
        &self,
        url: &str,
        filename: &str,
        path: &Path,
        status: &str,
    ) -> Result<DownloadRecord> {
        let now = Utc::now();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO downloads (url, filename, path, status, created_at) VALUES (?1,?2,?3,?4,?5)",
            params![url, filename, path.to_string_lossy(), status, now.to_rfc3339()],
        )?;
        Ok(DownloadRecord {
            id: conn.last_insert_rowid(),
            url: url.to_string(),
            filename: filename.to_string(),
            path: path.to_string_lossy().into_owned(),
            status: status.to_string(),
            created_at: now,
        })
    }

    pub fn list_downloads(&self, limit: i64) -> Result<Vec<DownloadRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, url, filename, path, status, created_at FROM downloads
             ORDER BY id DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |r| {
            Ok(DownloadRecord {
                id: r.get(0)?,
                url: r.get(1)?,
                filename: r.get(2)?,
                path: r.get(3)?,
                status: r.get(4)?,
                created_at: parse_dt(&r.get::<_, String>(5)?),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn save_session(paths: &AppPaths, session: &SessionState) -> Result<()> {
        let text = serde_json::to_string_pretty(session)?;
        std::fs::write(paths.session_path(), text)?;
        Ok(())
    }

    pub fn load_session(paths: &AppPaths) -> Result<Option<SessionState>> {
        let path = paths.session_path();
        if !path.exists() {
            return Ok(None);
        }
        let text = std::fs::read_to_string(path)?;
        Ok(Some(serde_json::from_str(&text)?))
    }
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bookmarks_roundtrip() {
        let dir = std::env::temp_dir().join(format!("cactus-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let paths = AppPaths {
            config_dir: dir.join("config"),
            data_dir: dir.join("data"),
            cache_dir: dir.join("cache"),
        };
        paths.ensure().unwrap();
        let store = Store::open(&paths).unwrap();
        store
            .add_bookmark("Example", "https://example.com", "Bar")
            .unwrap();
        let list = store.list_bookmarks().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].url, "https://example.com");
        store.record_history("Example", "https://example.com").unwrap();
        assert_eq!(store.list_history(10).unwrap().len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
