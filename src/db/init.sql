-- init.sql


CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    price REAL NOT NULL,
    stock INTEGER NOT NULL
);


CREATE TABLE IF NOT EXISTS baskets (
    id SERIAL PRIMARY KEY,
    status VARCHAR(50) NOT NULL 
);


CREATE TABLE IF NOT EXISTS basket_items (
   basket_id INTEGER NOT NULL REFERENCES baskets(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    PRIMARY KEY (basket_id, product_id)
);


CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    basket_id INTEGER NOT NULL REFERENCES baskets(id) ON DELETE CASCADE,
    total_paid REAL NOT NULL,
    status VARCHAR(50) NOT NULL 
);

INSERT INTO categories (name) VALUES ('Electronics'), ('Books');
INSERT INTO products (category_id, name, price, stock) VALUES 
(1, 'Mechanical Keyboard', 120.50, 45),
(1, 'Gaming Mouse', 49.99, 100),
(2, 'Rust Programming Book', 35.00, 20);