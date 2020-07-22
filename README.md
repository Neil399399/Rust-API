# Rust-API
Example API service with Rust.

## Rust Web Framework
### Actix-Web
Tutorial: [rust with actix-web](https://ithelp.ithome.com.tw/articles/10224273)     
Document: [actix-trait](https://docs.rs/actix-web/2.0.0/actix_web/)


## Connect Database
### diesel
Tutorial: [Install diesel](https://ithelp.ithome.com.tw/articles/102)       
Documents: [diesel traits](https://docs.diesel.rs/diesel/query_dsl/trait.RunQueryDsl.html#method.get_result)


## Start
### Start http server
```sh
make local # start postgres in docker and start http server.
```
### clean
```sh
make local-down # stop and remove database.
```

## Notice
1. Project structure
2. Cargo dependencies
3. Rust async functions 
4. Rust error handle
