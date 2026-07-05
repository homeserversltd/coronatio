mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn spawn_one_shot_http_response(status: u16, body: &'static str, delay: Duration) -> String {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buffer = [0_u8; 1024];
                let _ = stream.read(&mut buffer);
                if !delay.is_zero() {
                    thread::sleep(delay);
                }
                let response = format!(
                    "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = stream.write_all(response.as_bytes());
            }
        });
        format!("http://{}", addr)
    }

    include!("tests/part-01.rs");
    include!("tests/part-02.rs");
    include!("tests/part-03.rs");
    include!("tests/part-04.rs");
    include!("tests/part-05.rs");
}
