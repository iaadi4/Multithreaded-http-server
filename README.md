# Multithreaded HTTP Server in Rust

This project is a simple, multithreaded HTTP server written in Rust. It's a learning project to understand the basics of multithreading and TCP listeners in Rust.

## Features

*   **Multithreaded:** Uses a custom-built thread pool to handle multiple incoming connections concurrently.
*   **Static File Serving:** Serves static HTML files from the `public` directory.
*   **Basic Routing:**
    *   `GET /`: Serves the `hello.html` page.
    *   `GET /sleep`: Simulates a long-running request (5 seconds) to demonstrate non-blocking behavior.
    *   Other paths: Returns a `404 Not Found` page.
*   **Graceful Shutdown:** The server will gracefully shut down after handling two requests.

## How to Run

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/multithreaded-http-server.git
    cd multithreaded-http-server
    ```
2.  **Build the project:**
    ```bash
    cargo build
    ```
3.  **Run the server:**
    ```bash
    cargo run
    ```
4.  **Access the server:**
    Open your web browser and navigate to `http://127.0.0.1:7878`.

## Project Structure

```
.
├── Cargo.toml      # Project dependencies and metadata
├── public          # Static files served by the server
│   ├── 404.html    # 404 Not Found page
│   └── hello.html  # Main page
├── src
│   ├── lib.rs      # Thread pool implementation
│   └── main.rs     # Main application logic (TCP listener and request handling)
└── README.md       # This file
```
