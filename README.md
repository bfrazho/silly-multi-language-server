This is a silly project that has servers in multiple languages that send REST calls to each other
in a chain until they reach https://reqres.in/users 

As of right now this project has: Kotlin/Spring Boot -> Rust/Axum/Reqwest -> ReqRes Endpoint

```bash
curl -H 'Content-Type: application/json' -d '{"name": "userName", "job": "someRole"}' -X POST http://localhost:8080/users
```
