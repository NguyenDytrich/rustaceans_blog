use rocket::{get, routes};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[get("/ping")]
fn ping() -> String {
    String::from("pong")
}

#[derive(Serialize)]
struct IndexContext {
    greeting: String 
}

#[get("/")]
fn index() -> Template {
    Template::render("layout", IndexContext {
        greeting: String::from("Hello from Rocket and Handlebars!")
    })
}

#[rocket::main]
async fn main() {
    let _server = rocket::build()
        .mount("/", routes![ping, index])
        .attach(Template::fairing())
        .launch()
        .await;
}
