-- Your SQL goes here
CREATE TABLE products(
    id          uuid default uuid_generate_v4() primary key,
    name        varchar not null,
    barcode     varchar unique,
    price       decimal not null,
    stock       integer not null,
    created_at  timestamp not null,
    updated_at  timestamp not null
);