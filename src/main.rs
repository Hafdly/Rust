use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let thread_counter = Arc::new(AtomicU16::new(1));
    let saludar = std::env::var("saludar").unwrap_or("No Saludo al genio".to_string());
    let thread_index = thread_counter.load(Ordering::SeqCst);
    println!("{}",&saludar);

    
    HttpServer::new(move || {
        
        println!("Starting thread {}", thread_counter.fetch_add(1, Ordering::SeqCst));
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", 
                    web::get().to(move || {
                    HttpResponse::Ok()
                        .header("thread-id", thread_index.to_string())
                        .finish()
                }),
            )
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
