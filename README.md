# Rust Backend File Share with End-to-End Encryption

[![Watch the video](https://img.youtube.com/vi/t5w2dauFmhM/maxresdefault.jpg)](https://youtu.be/t5w2dauFmhM)

This project implements a file sharing backend using Rust, featuring end-to-end encryption to ensure the privacy and security of shared files.

## Table of Contents

- [Features](#features)
- [Technologies Used](#technologies-used)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
- [License](#license)
- [Donations](#donations)

## Technologies Used

   - **Rust**: The primary programming language for the backend.
   - **Axum**: A lightweight and ergonomic web framework for building APIs in Rust.
   - **SQLx**: An asynchronous, compile-time verified SQL crate supporting multiple databases (in this case, PostgreSQL).
   - **Argon2**: A secure password hashing library.
   - **jsonwebtoken**: A library for encoding and decoding JSON Web Tokens (JWT) for authentication.
   - **dotenv**: For managing environment variables in development.
   - **Tokio**: An asynchronous runtime for Rust, powering the non-blocking operations.
   - **Axum-Extra**: Additional utilities for Axum, including cookie support.
   - **Tokio-Cron-Scheduler**: A scheduler library for running tasks periodically based on cron-like expressions.
   - **Tower & Tower-HTTP**: Middleware and utilities for building robust HTTP services, including CORS and tracing support.
   - **Serde & Serde JSON**: A framework for serializing and deserializing Rust data structures efficiently, used with JSON data.
   - **Validator**: A validation framework for input validation in Rust.
   - **Chrono**: A date and time library, used with `serde` for working with time formats.
   - **UUID**: A library for generating and parsing universally unique identifiers (UUIDs).
   - **Tracing Subscriber**: A logging library for Rust applications, providing structured logging.
   - **AES & Block Modes**: Libraries for Advanced Encryption Standard (AES) encryption.
   - **RSA**: A library for RSA encryption and decryption.
   - **Rand**: A library for generating random values, used in cryptography and token generation.
   - **Base64**: A library for encoding and decoding Base64, often used in file and cryptographic operations.

## Technologies Used

- **Rust**: The primary programming language for the backend.
- **Actix Web**: A powerful, pragmatic, and extremely fast web framework for Rust.
- **SQLx**: An asynchronous, compile-time verified SQL crate.
- **Argon2**: Password hashing library for secure user authentication.
- **jsonwebtoken**: Library for encoding and decoding JWT tokens.
- **dotenv**: To manage environment variables.

## Getting Started

To get a local copy of this project up and running, follow these steps:

### Prerequisites

- Rust (1.58 or newer) installed. You can install Rust using [rustup](https://rustup.rs/).
- PostgreSQL installed and running. Ensure you have a database created for this project.

### Installation

1. Clone the repository:

   ```
   git clone https://github.com/AarambhDevHub/file-share-rust-backend.git
   cd file-share-rust-backend
   ```
2. Create a .env file in the root of the project with the following variables:

    ```
    # -----------------------------------------------------------------------------
    # Database (PostgreSQL)
    # -----------------------------------------------------------------------------
    DATABASE_URL=postgresql://username:password@localhost:5432/file_share_tutorial 

    # -----------------------------------------------------------------------------
    # JSON Web Token Credentials
    # -----------------------------------------------------------------------------
    JWT_SECRET_KEY=my_ultra_secure_jwt_secret_key
    JWT_MAXAGE=60
    ```

3. Install the necessary dependencies:

    ```
    cargo build
    ```

4. Run database migrations:

    ```
    sqlx migrate run
    ```

5. Start the server

    ```
    cargo run
    ```

## API Endpoints

- **POST /api/auth/register**: Register a new user.
- **POST /api/auth/login**: Login a user and return a JWT token.
- **GET /api/users/me**: Retrieve the authenticated user's information.
- **PUT /api/users/name**: Update the authenticated user's name.
- **PUT /api/users/password**: Change the authenticated user's password.
- **GET /api/users/search-emails**: Search for users by their email addresses.
- **POST /api/file/upload**: Upload a file (requires authentication).
- **GET /api/file/retrieve**: Retrieve an uploaded file by ID (requires authentication).
- **POST /api/list/send**: Send a list of files to another user.
- **GET /api/list/receive**: Retrieve the list of files received from another user.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

## Donations

If you find this project useful and would like to support its continued development, you can make a donation via [Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub).

Thank you for your support!
