-- Add up migration script here
CREATE TABLE products(
    id          uuid default uuid_generate_v4() primary key,
    name        varchar not null,
    barcode     varchar unique,
    price       numeric(15,2) not null,
    stock       integer not null,
    created_at  timestamp not null,
    updated_at  timestamp not null
);