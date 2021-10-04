use actix_web::web;

mod example;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/example")
            .service(example::get_all)
            .service(example::get_one)
            .service(example::create_one)
            .service(example::delete_one)
            .service(example::update_one)
    );
}
