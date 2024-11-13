# Basic Rust REST API

A simple REST API built with Rust and Actix-web that demonstrates basic CRUD operations for managing books.

## Features

- RESTful endpoints for managing books
- In-memory storage using Mutex for thread-safe data access
- JSON request/response handling
- Basic error handling
- Concurrent request processing

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Dependencies

```toml
[dependencies]
actix-web = "4.4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Project Structure

```
basic-rust-api/
├── Cargo.toml
├── Cargo.lock
└── src/
    └── main.rs
```

## API Endpoints

| Method | Endpoint     | Description          |
|--------|-------------|---------------------|
| GET    | /books      | Get all books       |
| GET    | /books/{id} | Get a book by ID    |
| POST   | /books      | Create a new book   |
| DELETE | /books/{id} | Delete a book by ID |

## Data Model

```rust
struct Book {
    id: u32,
    title: String,
    author: String,
}
```

## Installation and Running

1. Clone the repository:
```bash
git clone https://github.com/Ismat-Samadov/lets_get_rusty.git
cd lets_get_rusty
```

2. Build the project:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

## Usage Examples

### Create a Book
```bash
curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "title": "Rust Programming", "author": "John Doe"}' http://localhost:8080/books
```

### Get All Books
```bash
curl http://localhost:8080/books
```

### Get a Specific Book
```bash
curl http://localhost:8080/books/1
```

### Delete a Book
```bash
curl -X DELETE http://localhost:8080/books/1
```

## Error Handling

The API handles various error cases:
- Returns 404 Not Found when requesting non-existent books
- Returns 201 Created when successfully creating a book
- Returns 200 OK for successful operations
- Returns 400 Bad Request for invalid JSON payloads

## Concurrency

The API uses Rust's `Mutex` to handle concurrent access to the books data store, making it safe for multiple clients to access and modify the data simultaneously.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).

## Contact

Ismat Samadov - [Your Email or Contact Information]
Project Link: https://github.com/Ismat-Samadov/lets_get_rusty