-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE role AS ENUM ('admin', 'user');
CREATE TABLE users(
    id          uuid default uuid_generate_v4() primary key,
    full_name   varchar not null,
    email       varchar not null unique,
    password    varchar not null,
    role        role not null default 'user',
    created_at  timestamp not null,
    updated_at  timestamp not null
);