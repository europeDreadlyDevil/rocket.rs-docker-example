#[macro_use] extern crate rocket;

#[launch]
fn app() -> _ {
    rocket::build().mount("/", routes![call])
}

#[get("/get")]
fn call() -> &'static str {
    "Hello from service2".into()
}