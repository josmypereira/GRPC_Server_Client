# gRPC Server-Client Example in Rust

This project demonstrates the implementation of a gRPC-based server and client in Rust using **Tonic**, **Prost**, and **Protocol Buffers**. Below are the detailed steps for building, running, and understanding this project.

# Prerequisites

1. Rust Toolchain: Ensure you have the Rust toolchain installed. If not, install it from https://rustup.rs.
2. Protocol Buffer Compiler (`protoc`): Install the Protocol Buffer compiler from https://github.com/protocolbuffers/protobuf/releases.

# Project Structure
.
├── Cargo.toml                # Project dependencies and metadata
├── build.rs                  # Compiles the `.proto` files
├── src
│   ├── proto
│   │   └── auth.proto        # Protocol Buffer definitions
│   ├── service
│   │   ├── auth.rs           # gRPC service implementation
│   │   └── mod.rs            # Module definition
│   ├── client.rs             # gRPC client implementation
│   └── server.rs             # gRPC server implementation
└── target                    # Build artifacts (generated after building)

# Steps to Build and Run

# 1. Clone the Repository

git clone <repository-url>
cd <repository-folder>

# 2. Compile the `.proto` File

Run the `build.rs` script to generate Rust code from the `auth.proto` file:
cargo build

This will generate the necessary code for the gRPC server and client in the `auth_service` module.

# 3. Run the Server

Start the gRPC server by running:
cargo run --bin grpc-server

The server will listen for requests on `http://[::1]:50051` (localhost).

# 4. Run the Client

Open another terminal and execute the gRPC client:
cargo run --bin grpc-client

The client sends a `signIn` request to the server with predefined credentials and prints the response.

# Example Output

# Server Output
When the server receives a `signIn` request, it logs:
Got a response from the following user: SignInReq { username: "josmy", password: "password" }

# Client Output
The client prints the response from the server:
response: Response { metadata: MetadataMap { headers: {} }, message: SignInResp { user: Some(User { first_name: "josmy", last_name: "pereira", email: "josmy" }), token: "josmy", refresh_token: "josmy" } }

# Files Overview

# `auth.proto`
Defines the gRPC service (`AuthService`) and the message structures for requests and responses.

# `auth.rs`
Implements the gRPC server methods (`signIn`, `signOut`, `signUp`, and `update`) as defined in `auth.proto`.

# `client.rs`
Contains the gRPC client implementation that connects to the server and sends a `signIn` request.

# `server.rs`
Bootstraps the gRPC server using the `AuthService` implementation and listens for incoming requests.

# Dependencies

- `tonic`: gRPC library for Rust.
- `prost`: Protocol Buffer implementation for Rust.
- `tokio`: Asynchronous runtime for Rust.

# Customization

1. Modify `auth.proto` to add new services or messages.
2. Update the `auth.rs` file to implement custom logic for the new services.
3. Regenerate the Rust code using:
   cargo build

# Troubleshooting

- Error: `failed to resolve: could not find`
  Ensure the `protoc` compiler is installed and accessible in your PATH.
- Server not reachable:
  Ensure the server is running on the same host and port (`http://[::1]:50051`).

# References

- Tonic Documentation: https://docs.rs/tonic/latest/tonic/
- Protocol Buffers: https://developers.google.com/protocol-buffers
- Tokio: https://tokio.rs