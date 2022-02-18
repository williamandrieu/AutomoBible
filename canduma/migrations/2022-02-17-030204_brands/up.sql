CREATE TABLE brands
(
    brand_id      SERIAL PRIMARY KEY,
    image_url     VARCHAR,
    name          VARCHAR   NOT NULL,
    creation_year VARCHAR,
    created_at    TIMESTAMP NOT NULL DEFAULT NOW()
);
