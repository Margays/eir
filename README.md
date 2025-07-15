# Eir

Eir is a Rust-based project designed for efficient configuration management and metrics collection. It leverages modern Rust features and libraries to provide robust, extensible, and high-performance solutions for handling metrics endpoints.

## Features
- Flexible configuration loading
- HTTP client integration
- Metrics collection and reporting
- Extensible architecture


## Getting Started

### Prerequisites
- Rust (https://rustup.rs)
- Cargo

### Build
```sh
cargo build --release
```

### Run
```sh
cargo run --
```

### Configuration
The application is configured using a `config.json` file located in the project root. This file allows you to customize endpoints, metrics, and client settings. Below is an example structure and explanation:

- **client**: HTTP client configuration (e.g., base URL, timeout).
- **endpoints**: List of endpoints to which metrics or data will be sent.
- **metrics**: List of metrics to collect, each with a name and a query.

Adjust these fields to match your environment and requirements. The application will read this file at startup to determine its behavior.

## License
This project is licensed under the terms of the MIT License. See the `LICENSE` file for details.

## Contributing
Pull requests and issues are welcome! Please open an issue to discuss your ideas or report bugs.
