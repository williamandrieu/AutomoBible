CREATE TABLE models
(
    model_id   SERIAL PRIMARY KEY,
    brand_id   INT       NOT NULL,
    image_url  VARCHAR,
    name       VARCHAR   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_brand
        FOREIGN KEY (brand_id)
            REFERENCES brands (brand_id)
);
