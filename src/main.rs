#[macro_use]
extern crate diesel;

mod models;
mod schema;

use rocket::{get, routes};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;

use crate::models::BlogPost;

#[database("postgres")]
struct DbConn(diesel::PgConnection);

#[get("/ping")]
fn ping() -> String {
    String::from("pong")
}

#[derive(Serialize)]
struct IndexContext {
    greeting: String,
    posts: Vec<BlogPost> 
}

#[get("/")]
async fn index(conn: DbConn) -> Template {

    use crate::schema::blog_posts::dsl::*;
    use crate::diesel::QueryDsl;
    use crate::diesel::ExpressionMethods;
    use rocket_sync_db_pools::diesel::RunQueryDsl;

    let posts: Vec<BlogPost> = conn.run(|c| {
        blog_posts.filter(is_public.eq(true)).load(c)
    }).await.unwrap_or(Vec::new());

    Template::render("layout", IndexContext {
        greeting: String::from("Hello from Rocket and Handlebars!"),
        posts,
    })
}

#[rocket::main]
async fn main() {
    let _server = rocket::build()
        .mount("/", routes![ping, index])
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .launch()
        .await;
}
