use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    println!("Hello, world!");
    let x = 10;
    let y = 20;
    let z = add(x, y);
    print!("{}", format!("Here is the result: {x} + {y} = {z}"));

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/add", web::post().to(post_add))
    });

    println!("Serving on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000")
        .expect("Failed to bind to port 3000")
        .run()
        .await
        .expect("error running server");
}

fn add(m: u64, n: u64) -> u64 {
    m + n
}

use serde::Deserialize;
#[derive(Deserialize)]
struct AddParameters {
    m: u64,
    n: u64,
}
async fn post_add(form: web::Form<AddParameters>) -> HttpResponse {
    let add_result = add(form.m, form.n);
    HttpResponse::Ok().content_type("text/html").body(format!(
        "Addition of {} and {} is {}",
        form.m, form.n, add_result
    ))
}
async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
    <title>Calculator</title>
    <form action="/add" method="post">
    <input type="text" name="n"/>
    <input type="text" name="m"/>
    <button type="submit">Compute Addition</button>
    </form>
    "#,
    )
}
