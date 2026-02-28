mod http;

use http::request::Request;
use http::response::Response;
use http::router::Router;


fn main() {

    let mut app = Router::new();

    app.get("/books", get_books);
    // app.post("/books", create_book);
    // app.put("/books", update_book);
    // app.delete("/books", delete_book);

    app.listen("127.0.0.1:4221");
}

fn get_books(req: Request) -> Response {
    Response::new(200, "OK", "[{\"title\": \"Dune\"}]")
}

