// Bounded read-only HTTP framing shared by the private proxy and staff reads.
// Transport selection stays with the caller; no authority or wire schema lives here.
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub(super) struct Reply {
    pub status: u16,
    pub body: Vec<u8>,
}
#[derive(Clone, Copy, Debug)]
pub(super) struct Failure {
    pub outcome: &'static str,
    pub status: Option<u16>,
}
impl Failure {
    pub fn new(outcome: &'static str) -> Self {
        Self {
            outcome,
            status: None,
        }
    }
}
const MAX_RESPONSE: usize = 2 * 1024 * 1024;

pub(super) async fn exchange<S: AsyncRead + AsyncWrite + Unpin>(
    mut stream: S,
    host: &str,
    path: &str,
    timeout: Duration,
) -> Result<Reply, Failure> {
    tokio::time::timeout(timeout, async {
        let request = format!(
            "GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\nAccept: */*\r\n\r\n"
        );
        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|_| Failure::new("write-failed"))?;
        let mut raw = Vec::new();
        // Bound the whole exchange, not just idle time between bytes.
        (&mut stream)
            .take((MAX_RESPONSE + 1) as u64)
            .read_to_end(&mut raw)
            .await
            .map_err(|_| Failure::new("read-failed"))?;
        if raw.len() > MAX_RESPONSE {
            return Err(Failure::new("decode-error"));
        }
        decode(&raw)
    })
    .await
    .unwrap_or_else(|_| Err(Failure::new("timeout")))
}

fn decode(raw: &[u8]) -> Result<Reply, Failure> {
    let split = raw
        .windows(4)
        .position(|v| v == b"\r\n\r\n")
        .ok_or(Failure::new("decode-error"))?;
    if split > 16 * 1024 {
        return Err(Failure::new("decode-error"));
    }
    let head = std::str::from_utf8(&raw[..split]).map_err(|_| Failure::new("decode-error"))?;
    let status = head
        .lines()
        .next()
        .and_then(|v| v.split_whitespace().nth(1))
        .and_then(|v| v.parse::<u16>().ok())
        .filter(|v| (100..600).contains(v))
        .ok_or(Failure::new("decode-error"))?;
    let fail = Failure {
        outcome: "decode-error",
        status: Some(status),
    };
    let mut chunked = false;
    let mut length = None;
    for line in head.lines().skip(1) {
        if let Some((key, value)) = line.split_once(':') {
            if key.eq_ignore_ascii_case("transfer-encoding") {
                if !value.trim().eq_ignore_ascii_case("chunked") {
                    return Err(fail);
                }
                chunked = true;
            }
            if key.eq_ignore_ascii_case("content-length") {
                let n = value.trim().parse::<usize>().map_err(|_| fail)?;
                if length.is_some() && length != Some(n) {
                    return Err(fail);
                }
                length = Some(n);
            }
        }
    }
    let bytes = &raw[split + 4..];
    let body = if chunked {
        let mut cursor = bytes;
        let mut decoded = Vec::new();
        loop {
            let end = cursor.windows(2).position(|v| v == b"\r\n").ok_or(fail)?;
            let line = std::str::from_utf8(&cursor[..end]).map_err(|_| fail)?;
            let size = usize::from_str_radix(line.split(';').next().unwrap_or("").trim(), 16)
                .map_err(|_| fail)?;
            cursor = &cursor[end + 2..];
            if size == 0 {
                break;
            }
            if size > MAX_RESPONSE || cursor.len() < size + 2 || &cursor[size..size + 2] != b"\r\n"
            {
                return Err(fail);
            }
            decoded.extend_from_slice(&cursor[..size]);
            cursor = &cursor[size + 2..];
        }
        decoded
    } else {
        if let Some(length) = length {
            if length != bytes.len() {
                return Err(fail);
            }
        }
        bytes.to_vec()
    };
    Ok(Reply { status, body })
}

pub(super) async fn tcp_get(
    addr: std::net::SocketAddr,
    path: &str,
    timeout: Duration,
) -> Result<Reply, Failure> {
    tokio::time::timeout(timeout, async {
        let stream = tokio::net::TcpStream::connect(addr)
            .await
            .map_err(|_| Failure::new("connect-failed"))?;
        exchange(stream, &addr.to_string(), path, timeout).await
    })
    .await
    .unwrap_or_else(|_| Err(Failure::new("timeout")))
}
