## Prerequisites
```
postgresql
cargo
```

## Install dependencies
```
cargo install diesel_cli --no-default-features --features postgres
```

## Run
```
// setup database
diesel setup
diesel migration run

# run in first terminal
cargo run --bin db_controller

# run in second terminal
cargo run --bin web_server

# run in third terminal
curl --request GET 'http://localhost:8889/api/health'
curl --request GET 'http://localhost:8889/api/user_profile/1'
```

## Todo
- [ ] register endpoint
- [ ] error handler
- [ ] endpoint return type