# Custom rust http client

Simple implementation of generic HTTP client for rust. 

Basically it's a wrapper around [reqwest](https://crates.io/crates/reqwest) crate which actually another wrapper around HTTP Client.

For testing of the HTTP Client used another API written on .NET ([source](https://github.com/oleksandr-rapiy/minimal-todo-api)). Which implement basic CRUD operation around ToDo domain.