-- Your SQL goes here
CREATE TABLE transactions(
    id              uuid default uuid_generate_v4() primary key,
    user_id         uuid not null references users(id),
    created_at      timestamp not null
);

CREATE TABLE transaction_products(
    transaction_id  uuid not null references transactions(id),
    product_id      uuid not null references products(id),
    quantity        integer not null,
    price           numeric(15,2),
    primary key (transaction_id, product_id)
)