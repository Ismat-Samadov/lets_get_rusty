# Let's Get Rusty

A learning repository for Rust programming language featuring practical examples and projects.

## Project Structure

```
lets_get_rusty/
├── README.md
└── basic-rust-api/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Projects

### 1. Basic Rust API

A simple REST API built with Rust and Actix-web demonstrating basic CRUD operations for managing books.

#### Features

- RESTful endpoints for managing books
- In-memory storage using Mutex for thread-safe data access
- JSON request/response handling
- Basic error handling
- Concurrent request processing

#### Running the API

1. Navigate to the API directory:
```bash
cd basic-rust-api
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

#### API Endpoints

| Method | Endpoint     | Description          |
|--------|-------------|---------------------|
| GET    | /books      | Get all books       |
| GET    | /books/{id} | Get a book by ID    |
| POST   | /books      | Create a new book   |
| DELETE | /books/{id} | Delete a book by ID |

#### Usage Examples

Create a Book:
```bash
curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "title": "Rust Programming", "author": "John Doe"}' http://localhost:8080/books
```

Get All Books:
```bash
curl http://localhost:8080/books
```

Get a Specific Book:
```bash
curl http://localhost:8080/books/1
```

Delete a Book:
```bash
curl -X DELETE http://localhost:8080/books/1
```

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

Clone the repository:
```bash
git clone https://github.com/Ismat-Samadov/lets_get_rusty.git
cd lets_get_rusty
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request