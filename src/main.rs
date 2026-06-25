use axum::{
    extract::{Path, State},
    http::{header, Method, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::PathBuf,
    sync::Arc,
    thread,
    time::Duration,
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
