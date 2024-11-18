# Axum SQLx CRUD API with Pagination

A production-ready REST API built with Axum, featuring robust pagination middleware and PostgreSQL integration via SQLx.

## Features

- 🚀 Built with Axum web framework
- 📦 PostgreSQL database integration using SQLx
- 📄 Pagination middleware with customizable parameters
- ✨ Clean and modular architecture
- 🛡️ Comprehensive error handling
- 🔒 Input validation and sanitization
- 📝 JSON response formatting

## Project Structure

```
src/
├── api/           # API endpoints and handlers
│   ├── v1/        # API version 1
│   │   ├── items.rs
│   │   └── mod.rs
│   └── mod.rs
├── db/            # Database connection and initialization
│   └── mod.rs
├── error/         # Error types and handling
│   └── mod.rs
├── middleware/    # Middleware components
│   ├── pagination.rs
│   └── mod.rs
├── models/        # Data structures and models
│   └── mod.rs
└── main.rs        # Application entry point
```

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Cargo package manager

## Getting Started

1. Clone the repository:
```bash
git clone <repository-url>
cd axum_sqlx_crud
```

2. Create a `.env` file in the project root:
```env
DATABASE_URL=postgres://username:password@localhost/database_name
```

3. Build and run the project:
```bash
cargo run
```

The server will start at `http://127.0.0.1:3000`.

## API Endpoints

### Get Items (Paginated)
```
GET /items
```

Query Parameters:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Items per page (default: 10, max: 100)

Example Response:
```json
{
  "success": true,
  "data": {
    "data": [
      {
        "id": 1,
        "name": "Item 1"
      },
      // ... more items
    ],
    "meta": {
      "current_page": 1,
      "per_page": 10,
      "total_items": 25,
      "total_pages": 3
    }
  }
}
```

## Error Handling

The API returns consistent error responses in JSON format:

```json
{
  "success": false,
  "error": {
    "message": "Error description",
    "code": 400
  }
}
```

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Configuration

The following environment variables are supported:

- `DATABASE_URL`: PostgreSQL connection string (required)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

