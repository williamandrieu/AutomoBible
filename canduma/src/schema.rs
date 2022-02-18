table! {
    brands (brand_id) {
        brand_id -> Int4,
        image_url -> Nullable<Varchar>,
        name -> Varchar,
        creation_year -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    models (model_id) {
        model_id -> Int4,
        brand_id -> Int4,
        image_url -> Nullable<Varchar>,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_uuid -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        role -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(models -> brands (brand_id));

allow_tables_to_appear_in_same_query!(
    brands,
    models,
    users,
);
