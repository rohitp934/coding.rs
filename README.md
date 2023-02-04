# Rusty Coding

This repository is a backend web server written in Rust that allows you to execute and run code through a REST API. It uses Rust to run the code and return the output.

## Frameworks and Technology Used

This project uses a variety of technologies to provide its functionality, including:

- **Actix web**: Actix web is a fast, reliable, and flexible web framework used to implement the backend web server.
- **Tokio**: Tokio is an asynchronous runtime for Rust, used to provide a high level of concurrency and responsiveness to the server.
- **Rust STD**: The Rust Standard Library is used to spawn and execute candidate code, providing a safe and efficient way to run untrusted code.
- **Rayon**: Rayon is a data-parallelism library for Rust, used to parallelize the execution of different test cases, allowing for efficient and concurrent execution.

Each of these technologies was carefully chosen to provide a stable and performant platform for running and testing candidate code.

## Getting Started

To get started with the project, you can follow these steps:

1. **Clone the repository to your local machine using the following command:**

    ```bash
    git clone https://github.com/<username>/rust-code-execution-server.git
    ```
2. **Navigate to the repository root directory:**

    ```bash
    cd rust-code-execution-server
    ```
3. **Use cargo run to run the project:**
    ```bash
    cargo run
    ```

## Contributing

We welcome contributions to the project! Before contributing, please read the contribution guidelines.

## Style

We use `rust-analyzer` for formatting and `clippy` as the linter for this project.

## License

This project is licensed under the MIT License.
