use axum::{
    extract::{Multipart, Path, Query, State},
    http::{header, HeaderValue, Method, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, VecDeque},
    env,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::{Path as FsPath, PathBuf},
    process::Command,
    sync::{Arc, Mutex, OnceLock},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::fs;
use tower_http::services::ServeDir;

include!("bands/contracts.rs");
include!("bands/runtime.rs");
include!("bands/routes.rs");
include!("bands/caduceus.rs");
include!("bands/router-readback.rs");
include!("bands/full-rust-routes.rs");
include!("bands/crown-law.rs");
include!("bands/shell.rs");

#[cfg(test)]
include!("bands/tests.rs");
