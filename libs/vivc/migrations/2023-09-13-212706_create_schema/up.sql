CREATE TABLE vivc (
    id SERIAL PRIMARY KEY,
    cultivar_name VARCHAR(255) NOT NULL,
    prime_name VARCHAR(255) NOT NULL,
    variety_number INTEGER NOT NULL,
    species VARCHAR(255),
    color VARCHAR(255),
    country_or_region VARCHAR(255)
);
