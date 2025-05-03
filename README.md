# 🛒 Store API (Rust + Axum + MySQL)

Simple CRUD REST API built with **Rust**, **Axum**, **SQLx**, and **MySQL** — handles `items` and `categories` for a mock store system.

---

## 🚀 Features

- RESTful API using [Axum](https://docs.rs/axum)
- Async MySQL database access via [SQLx](https://docs.rs/sqlx)
- Tracing & logging support
- Basic error handling with proper HTTP status codes
- Clean modular architecture (`routes`, `handlers`, `models`, `db`)
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
- **tracing** – structured logging

---

## ⚙️ Setup

### 1. Clone the project

```bash
git clone https://github.com/your-username/store-api.git
cd store-api
```

### 2. Set up MySQL

Create the database manually or via CLI:

```sql
CREATE DATABASE store_db;
```

You can use this schema:

```sql
CREATE TABLE categories (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE items (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price DOUBLE NOT NULL,
    quantity INT NOT NULL,
    category_id BIGINT,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);
```

### 3. Set up `.env`

Create a `.env` file in the project root:

```env
DATABASE_URL=mysql://YOUR_USERNAME:YOUR_PASSWORD@localhost/store_db
```

Make sure the credentials match your local MySQL config.

---

## ▶️ Running

```bash
cargo run
```

Server will run at:

```
http://localhost:3000
```

---

## 📮 Available Routes

### Items

| Method | Endpoint                    | Description              |
|--------|-----------------------------|--------------------------|
| GET    | `/items`                    | Get all items            |
| GET    | `/items/:id`                | Get item by ID           |
| GET    | `/items/category/:id`       | Get items by category ID |
| POST   | `/items`                    | Create a new item        |
| PUT    | `/items/:id`                | Update an item           |
| DELETE | `/items/:id`                | Delete an item           |

### Categories

| Method | Endpoint            | Description            |
|--------|---------------------|------------------------|
| GET    | `/categories`       | Get all categories     |
| GET    | `/categories/:id`   | Get category by ID     |
| POST   | `/categories`       | Create new category *(TBD)* |

## 🧠 Author

Made by [Emir Totic] – Backend developer 🚀
