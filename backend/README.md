# TicketApp Backend

A robust and efficient backend API for a ticketing application, built with **Rust** using the **Axum** web framework and **SQLx** for database interactions with **SQLite**.

## 🚀 Key Features

- **User Management**: Full CRUD operations for users.
- **Event Management**: Create and list upcoming events.
- **Ticket Generation**: Backend support for QR codes and PDF ticket generation.
- **Secure & Fast**: Leveraging Rust's safety and performance with asynchronous processing.
- **RESTful API**: Clean and documented API endpoints.

## 🛠️ Tech Stack

- **Lanuage**: [Rust](https://www.rust-lang.org/)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Database**: [SQLite](https://www.sqlite.org/) with [SQLx](https://github.com/launchbadge/sqlx)
- **Serialization**: [Serde](https://serde.rs/)
- **Utilities**: 
  - `printpdf`: For PDF ticket generation.
  - `qrcode`: For QR code generation.
  - `tower-http`: For CORS and middleware support.

## 🏃 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Installation

1. Clone the repository:
   ```bash
   git clone <your-repo-url>
   cd ticketapp/backend
   ```

2. Configure environment variables:
   Create a `.env` file in the root directory:
   ```env
   DATABASE_URL=sqlite:tickets.db
   ```

3. Run the application:
   ```bash
   cargo run
   ```

The backend will be available at `http://127.0.0.1:4000/api`.

## 📁 Project Structure

```text
├── src/
│   ├── db.rs           # Database initialization
│   ├── domain/         # Data models and structures
│   ├── handlers/       # Request handlers (Business logic)
│   ├── repositories/   # Data access layer
│   ├── routes/         # API route definitions
│   └── main.rs         # Application entry point
├── tickets.db          # SQLite database
└── Cargo.toml          # Rust dependencies and project config
```

## 📝 License

This project is licensed under the MIT License.
