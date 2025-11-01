use std::sync::{Arc, Mutex, OnceLock};
use pyo3::prelude::*;
use zenoh::{Config, Session, Wait};
use zenoh::pubsub::Publisher;

// グローバルSession（遅延初期化）
static ZENOH_SESSION: OnceLock<Arc<Session>> = OnceLock::new();

/// Zenoh Sessionを初期化（初回のみ実行）
fn get_or_init_session(config_path: &str) -> PyResult<Arc<Session>> {
  // すでに初期化済みならそれを返す
  if let Some(session) = ZENOH_SESSION.get() {
    return Ok(Arc::clone(session));
  }

  // 初回: Sessionを作成
  let config = Config::from_file(config_path)
    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(
      format!("Failed to load config from {}: {}", config_path, e)
    ))?;

  let session = zenoh::open(config)
    .wait()
    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(
      format!("Failed to open Zenoh session: {}", e)
    ))?;

  let session_arc = Arc::new(session);

  // グローバルに保存（失敗したら他のスレッドが先に初期化した）
  match ZENOH_SESSION.set(Arc::clone(&session_arc)) {
    Ok(_) => Ok(session_arc),
    Err(_) => {
      // 他のスレッドが先に初期化した場合、そちらを使う
      Ok(Arc::clone(ZENOH_SESSION.get().unwrap()))
    }
  }
}

pub fn create_publisher(config_path: &str, topic_name: &str) -> PyResult<Arc<Mutex<Publisher<'static>>>> {
  let session = get_or_init_session(config_path)?;

  let publisher = session
    .declare_publisher(topic_name.to_string())
    .wait()
    .map(|p| Arc::new(Mutex::new(p)))
    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(
      format!("Failed to create publisher for {}: {}", topic_name, e)
    ))?;

  Ok(publisher)
}