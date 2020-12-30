use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sailfish::TemplateOnce;

mod create_page;

#[derive(TemplateOnce)]
#[template(path = "_build/templates/aah.html")]
struct HelloTemplate {
    name: String,
    layout_title: String,
}

#[get("/{name}")]
async fn index(web::Path(name): web::Path<String>) -> impl Responder {
    let ctx = HelloTemplate {
        name: name,
        layout_title: "The title!".into(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(ctx.render_once().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
