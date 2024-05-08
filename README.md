# Rust Clean Architecture

## why

This is a simple example of how to implement a clean architecture in Rust.

## architecture

This project is divided into four main modules:

- `application`: This module contains the application logic. It is the core of the application and it is independent of the infrastructure. It contains the use cases and the business rules.
- `controller`: This module contains the web layer. It is responsible for handling the HTTP requests and responses.
- `domain`: This module contains the domain logic. It is the heart of the application and it contains the entities and the value objects.
- `infrastructure`: This module contains the infrastructure logic. It is responsible for the database, the web framework, the email service, etc.

## how

The application is a simple CRUD API that manages users. It has the following use cases:

- Create a user

```json
{
  "name": "John Doe",
  "email": "test@test.com",
  "email_verified": false
}
```

- Get a user by id

```json
{
  "id": 1
}
```

- Update a user

```json
{
  "id": 1,
  "name": "Jane Doe",
  "email": "test@testtest.com",
  "email_verified": true
}
```

- Delete a user

```json
{
  "id": 1
}
```

## how to run

To run the application, you need to have Rust installed. You can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

```
$ make run
```
