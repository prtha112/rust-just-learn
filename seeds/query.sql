CREATE TABLE IF NOT EXISTS users (
  id      BIGSERIAL PRIMARY KEY,
  username    TEXT NOT NULL,
  password    TEXT NOT NULL,
  active  BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS categories (
  id      BIGSERIAL PRIMARY KEY,
  name    TEXT NOT NULL,
  active  BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS products (
  id          BIGSERIAL PRIMARY KEY,
  name        TEXT NOT NULL,
  description TEXT,
  price       NUMERIC(10, 2) NOT NULL,
  stock       INT NOT NULL,
  category_id BIGINT NOT NULL REFERENCES categories(id),
  active      BOOLEAN NOT NULL DEFAULT TRUE
);