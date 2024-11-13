// src/main.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Book structure to store our data
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

// AppState to store our books in memory
struct AppState {
    books: Mutex<Vec<Book>>,
}

// GET all books
async fn get_books(data: web::Data<AppState>) -> impl Responder {
    let books = data.books.lock().unwrap();
    HttpResponse::Ok().json(&*books)
}

// GET book by id
async fn get_book(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let book_id = path.into_inner();
    let books = data.books.lock().unwrap();
    
    if let Some(book) = books.iter().find(|b| b.id == book_id) {
        HttpResponse::Ok().json(book)
    } else {
        HttpResponse::NotFound().finish()
    }
}

// POST new book
async fn create_book(book: web::Json<Book>, data: web::Data<AppState>) -> impl Responder {
    let mut books = data.books.lock().unwrap();
    books.push(book.into_inner());
    HttpResponse::Created().finish()
}

// DELETE book
async fn delete_book(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let book_id = path.into_inner();
    let mut books = data.books.lock().unwrap();
    
    if let Some(index) = books.iter().position(|b| b.id == book_id) {
        books.remove(index);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize app state with empty books vector
    let app_state = web::Data::new(AppState {
        books: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/books", web::get().to(get_books))
            .route("/books/{id}", web::get().to(get_book))
            .route("/books", web::post().to(create_book))
            .route("/books/{id}", web::delete().to(delete_book))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}