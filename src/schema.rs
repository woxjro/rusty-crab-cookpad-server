table! {
    authorities (id) {
        id -> Int4,
        name -> Varchar,
        hash -> Varchar,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        hash -> Varchar,
    }
}

table! {
    comments (id) {
        id -> Int4,
        user_id -> Int4,
        recipe_id -> Int4,
        content -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

table! {
    ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
        name -> Varchar,
        amount -> Varchar,
    }
}

table! {
    procedures (id) {
        id -> Int4,
        recipe_id -> Int4,
        number -> Int4,
        discription -> Varchar,
        image_path -> Nullable<Varchar>,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        thumbnail_path -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        hash -> Varchar,
    }
}

table! {
    user_types (id) {
        id -> Int4,
        name -> Varchar,
        hash -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        user_type -> Int4,
        email -> Varchar,
        icon_path -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    authorities,
    categories,
    comments,
    ingredients,
    procedures,
    recipes,
    tags,
    user_types,
    users,
);
