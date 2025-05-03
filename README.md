# 🛒 Store API (Rust + Axum + MySQL)

Simple CRUD REST API built with **Rust**, **Axum**, **SQLx**, and **MySQL** — handles `items`, `categories`, and `users` for a mock store system. Supports authentication (JWT) and role-based authorization.

---

## 🚀 Features

- RESTful API using [Axum](https://docs.rs/axum)
- Async MySQL access with [SQLx](https://docs.rs/sqlx)
- JWT-based login/auth middleware
- Role-based authorization (`seller`, `customer`)
- Password hashing with `argon2`
- Full CRUD for `items`, `categories`
- User registration & login
- Error handling with proper HTTP status codes
- Tracing/logging for observability
- Clean modular structure (`routes`, `handlers`, `auth`, `models`, `middleware`, `db`)

---

## 📦 Tech Stack

- **Rust** 🦀 (2021 edition)
- **Axum** – async web framework
- **SQLx** – type-safe SQL access
- **MySQL** – relational database
- **Tokio** – async runtime
- **serde / serde_json** – JSON (de)serialization
- **dotenvy** – load env variables from `.env`
- **jsonwebtoken** – JWT encoding/decoding
- **argon2** – secure password hashing
- **tracing** – for logging

---

## ⚙️ Setup

### 1. Clone the project

```bash
git clone https://github.com/your-username/store-api.git
cd store-api
```

### 2. Set up MySQL

Create the schema and tables:

```sql
CREATE DATABASE store_db;

USE store_db;

CREATE TABLE categories (
    id   BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE items (
    id          BIGINT AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    price       DOUBLE NOT NULL,
    quantity    INT NOT NULL,
    category_id BIGINT,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE roles (
    id   BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

INSERT INTO roles (name) VALUES ('seller'), ('customer');

CREATE TABLE users (
    id            BIGINT AUTO_INCREMENT PRIMARY KEY,
    username      VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role_id       BIGINT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES roles(id)
);
```

### 3. Create `.env`

```env
DATABASE_URL=mysql://root:12345678@localhost/store_db
JWT_SECRET=secret
```

Adjust values as needed.

---

## ▶️ Running the Project

```bash
cargo run
```

Server will run at: [http://localhost:3000](http://localhost:3000)

---

## 🛠️ Endpoints

### 🔓 Public Routes

| Method | Endpoint          | Description              |
|--------|-------------------|--------------------------|
| POST   | `/auth/register`  | Register new user        |
| POST   | `/auth/login`     | Login and get JWT token  |

---

### 🔐 Protected (Any Authenticated User)

| Method | Endpoint                  | Description                    |
|--------|---------------------------|--------------------------------|
| GET    | `/items`                  | List all items                 |
| GET    | `/items/:id`              | Get item by ID                 |
| GET    | `/items/category/:id`     | Get items by category ID       |
| GET    | `/categories`             | List all categories            |
| GET    | `/categories/:id`         | Get category by ID             |

Use `Authorization: Bearer <token>` header.

---

### 🛡️ Seller Only Routes

| Method | Endpoint           | Description             |
|--------|--------------------|-------------------------|
| POST   | `/items/create`    | Create new item         |
| POST   | `/items/:id`       | Update item             |
| DELETE | `/items/:id`       | Delete item             |

Requires a `seller` role token.

---

## 🔐 Auth Notes

- JWT token is issued on login
- Include it in requests via:

```http
Authorization: Bearer <token>
```

- Token contains role (`seller` or `customer`) and expires after 15 minutes.

---

## 🧠 Author

Made by **Emir Totic** – Backend engineer