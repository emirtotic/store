
# 🛒 Store API (Rust + Axum + MySQL)

Simple CRUD REST API built with **Rust**, **Axum**, **SQLx**, and **MySQL** — handles `items`, `categories`, and user authentication for a mock store system.

---

## 🚀 Features

- RESTful API using [Axum](https://docs.rs/axum)
- Async MySQL database access via [SQLx](https://docs.rs/sqlx)
- JWT-based authentication & role-based authorization
- Tracing & logging support
- Basic error handling with proper HTTP status codes
- Clean modular architecture (`routes`, `handlers`, `models`, `db`, `auth`)
- JSON input/output using `serde`

---

## 📦 Tech Stack

- **Rust** 🦀 (2021 edition)
- **Axum** – lightweight web framework
- **SQLx** – compile-time checked SQL queries
- **MySQL** – as relational database
- **Tokio** – async runtime
- **serde / serde_json** – for JSON serialization
- **dotenvy** – environment variable loading
- **jsonwebtoken** – for JWT handling
- **tracing** – structured logging

---

## ⚙️ Setup

### 1. Clone the project

```bash
git clone https://github.com/your-username/store-api.git
cd store-api
```

### 2. Set up MySQL

```sql
CREATE DATABASE store_db;

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

### 3. Environment config

Create a `.env` file:

```env
DATABASE_URL=mysql://YOUR_USERNAME:YOUR_PASSWORD@localhost/store_db
JWT_SECRET=your_jwt_secret
```

---

## ▶️ Running

```bash
cargo run
```

Server will run at: `http://localhost:3000`

---

## 📮 API Endpoints

### 🆓 Public Routes

| Method | Endpoint            | Description          |
|--------|---------------------|----------------------|
| POST   | `/auth/register`    | Register a new user  |
| POST   | `/auth/login`       | Login and get token  |

### 🔓 Open (Requires Token)

| Method | Endpoint                            | Description                        |
|--------|-------------------------------------|------------------------------------|
| GET    | `/items`                            | Get all items                      |
| GET    | `/items/:id`                        | Get item by ID                     |
| GET    | `/items/category/:id`               | Get items by category ID           |
| GET    | `/categories`                       | Get all categories                 |
| GET    | `/categories/:id`                   | Get category by ID                 |
| GET    | `/items/search?name=milk&page=1`    | Search items by name + pagination |
| GET    | `/items/search/category/:category_name` | Get items by category name     |

### 🔐 Protected (Role: `seller`)

| Method | Endpoint            | Description              |
|--------|---------------------|--------------------------|
| POST   | `/items/create`     | Create a new item        |
| POST   | `/items/:id`        | Update item              |
| DELETE | `/items/:id`        | Delete item              |

---

## 🔐 Auth

- JWT-based access tokens
- Tokens expire after **15 minutes**
- Role-based route protection (`customer`, `seller`)

---

## 🧠 Author

Made by **Emir Totic** — Backend Developer 🚀
