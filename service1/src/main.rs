#[macro_use] extern crate rocket;

#[launch]
fn app() -> _ {
    rocket::build().mount("/", routes![client::get])
}



mod client {
    #[get("/get")]
    pub async fn get() -> String {
        reqwest::get("http://service2:80/get").await.unwrap().text().await.unwrap()
    }
}