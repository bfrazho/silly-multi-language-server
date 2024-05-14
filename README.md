This is a silly project that has servers in multiple languages that send REST calls to each other
in a chain until they reach https://reqres.in/users 

As of right now this project has: Go/Gin -> Kotlin/Spring Boot -> Rust/Axum/Reqwest -> ReqRes Endpoint

Start go/gin server
```bash
cd go
go run main.go
```

Start kotlin/spring boot server
```bash
cd kotlin
./gradlew bootRun
```

Start rust/axum server
```bash
cd rust
cargo run --release
```

Curl the post endpoint starting from the Go/Gin Server
```bash
curl -H 'Content-Type: application/json' -d '{"name": "userName", "job": "someRole"}' -X POST http://localhost:7000/users
```
