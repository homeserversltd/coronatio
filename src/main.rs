use axum::{
    body::{to_bytes, Body},
    extract::{Path, State},
    http::{header, HeaderMap, Method, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    os::unix::net::UnixStream,
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::fs;
use tower_http::services::{ServeDir, ServeFile};

include!("bands/contracts.rs");
include!("bands/server.rs");
include!("bands/routes.rs");
include!("bands/legacy-proxy.rs");
include!("bands/tab-manifests.rs");
include!("bands/crown-readbacks.rs");
include!("bands/topic-readbacks.rs");
include!("bands/frontend-storage.rs");
include!("bands/installer-stats.rs");
include!("bands/legacy-shell.rs");
include!("bands/tests.rs");
