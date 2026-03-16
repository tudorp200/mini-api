# GEMINI.md - Project Context

## Project Overview
This project, named **hw1**, is a custom-built, lightweight HTTP API written in Rust. It does not use standard high-level web frameworks like Actix or Rocket, but instead implements its own concurrency model, HTTP request/response parsing, and routing logic.

The API provides CRUD operations for a simple e-commerce system with the following entities:
- **Categories**: Product groupings.
- **Products**: Individual items with price and stock information.
- **Baskets**: Shopping carts for users.
- **Basket Items**: Links between products and baskets.
- **Orders**: Finalized purchases associated with a basket.

### Key Technologies
- **Rust (2024 Edition)**
- **PostgreSQL**: Primary database.
- **r2d2**: Database connection pooling.
- **crossbeam**: Used for a custom thread-safe job queue in the thread pool.
- **serde & serde_json**: For JSON serialization and deserialization.
- **Docker & Docker Compose**: For containerized database setup.

### Architecture
The project follows a modular structure with a clear separation of concerns:
- **`src/http/`**: Contains the custom HTTP `Router`, `Request`, and `Response` implementations.
- **`src/concurrency/`**: Implements a custom `ThreadPool` and `Worker` system to handle incoming TCP connections concurrently.
- **`src/controllers/`**: Handles application logic and translates HTTP requests into repository calls. Includes generic `BaseController` and entity-specific controllers.
- **`src/repositories/`**: Manages database access using a repository pattern. Includes generic `BaseRepository`.
- **`src/models/`**: Defines the data structures and traits for the system entities.
- **`src/traits.rs`**: Defines shared behavior for repositories and controllers.

## Building and Running

### Prerequisites
- Rust and Cargo
- Docker and Docker Compose (for the database)

### Setup Database
To start the PostgreSQL database with the required schema:
```powershell
# From the project root
docker-compose -f src/db/docker-compose.yml up -d
```
The database initialization script is located at `src/db/init.sql`.

### Build and Run
```powershell
cargo build
cargo run
```
The server listens on `127.0.0.1:4221` by default.

### Testing
There are currently no automated tests in the project.
```powershell
# Placeholder for future tests
cargo test
```

## Development Conventions

### Architecture and Traits
The system heavily utilizes traits to enforce structure and reduce boilerplate:
- **`Model` Trait (`src/traits.rs`)**: Every database-backed entity must implement `Model`, providing methods for table names, row parsing, and SQL query generation (`insert_query`, `update_query`).
- **`Repository` Trait (`src/traits.rs`)**: Defines standard CRUD operations (`save`, `find_by_id`, `find_all`, `update`, `delete`) for a given `Model`.
- **`Controller` Trait (`src/traits.rs`)**: Associates a controller with its specific `Model` and `Repository`.

### Routing
Routes are defined in `src/main.rs` using the `Router` instance. It supports dynamic path parameters (e.g., `/products/:id`).
- `GET`, `POST`, `PUT`, `PATCH`, `DELETE` are supported.

### Concurrency
The server uses a fixed-size `ThreadPool` (default 6 workers) implemented in `src/concurrency/thread_pool.rs`. Each incoming connection is handled by a worker from the pool.

### Data Access
Always use the Repository pattern. Generic operations should be placed in `BaseRepository`. Entity-specific logic should go into specialized repositories like `BasketItemRepository`. All models (like `Product`, `Category`, `Basket`, `Order`) are located in `src/models/` and must implement the `Model` trait.

### HTTP Implementation
- **Parsing**: Requests are manually parsed from `TcpStream` in `src/http/request.rs`.
- **Routing**: `src/http/router.rs` handles path matching and dynamic parameter extraction.
- **Serialization**: `serde` and `serde_json` are used for all JSON interactions.
