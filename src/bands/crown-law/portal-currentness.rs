fn normalize_systemd_unit(service: &str) -> String {
    let service = service.trim();
    if service.ends_with(".service") {
        service.to_string()
    } else {
        format!("{service}.service")
    }
}
