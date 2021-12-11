#[get("/health")]
pub fn health() -> String {
    "OK".to_string()
}

#[get("/liveness")]
pub fn liveness() -> String {
    "OK".to_string()
}



