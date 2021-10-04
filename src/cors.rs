fn development_cors() -> actix_cors::Cors {
    actix_cors::Cors::permissive()
}

fn default_cors() -> actix_cors::Cors {
    actix_cors::Cors::default()
}

pub fn create_cors() -> actix_cors::Cors {
    match std::env::var("RUST_ENV") {
        Ok(env) => {
            if env == "development" {
                development_cors()
            } else {
                default_cors()
            }
        }
        Err(_) => default_cors(),
    }
}
