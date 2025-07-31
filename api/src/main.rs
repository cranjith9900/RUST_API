use poem::{get, handler, listener::TcpListener, Route, Server};
use poem::web::Path;

#[handler]
fn get_websites(Path(website_id): Path<String>) -> String {
    format!("GET: {}", website_id)
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_websites)); 

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
