# Location Service

A RESTful service built with Rust that handles location data storage and management.

## Features

- Store location data with client and session information
- RESTful API using Axum framework
- MongoDB integration using mongodm
- Asynchronous operations with Tokio

## Prerequisites

- Rust 1.86.0 or higher
- MongoDB instance
- Cargo package manager

## Project Structure
```
location-service/ 
├── src/ 
│ ├── config/ # Configuration management 
│ ├── controllers/ # HTTP request handlers 
│ ├── dtos/ # Data Transfer Objects 
│ ├── exceptions/ # Error handling 
│ ├── models/ # Database models
│ ├── repositories/ # Data access layer 
│ ├── services/ # Business logic 
│ └── main.rs # Application entry point 
├── Cargo.toml # Project dependencies 
└── README.md
```


## Getting Started

1. Clone the repository:
   
```
bash git clone <repository-url> cd location-service
```

2. Set up environment variables:
```aiignore
bash cargo run
```

## API Endpoints

### Save Location
```
POST /location
```

Request body:

```json 
{ 
  "client_id": 123, 
  "session_id": "session-uuid", 
  "location": {
    "latitude": 51.5074,
    "longitude": -0.1278
  }
}
```

## Dependencies

- tokio 1.45.0 - Asynchronous runtime
- axum 0.8.4 - Web framework
- mongodm 0.10.0 - MongoDB ODM
- serde 1.0.219 - Serialization framework
- tracing-subscriber 0.3.19 - Logging
- anyhow 1.0.98 - Error handling
- async-trait 0.1.88 - Async traits support
- serde_json 1.0.140 - JSON serialization

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
