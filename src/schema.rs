table! {
    authorities (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        hash -> Nullable<Varchar>,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        hash -> Nullable<Varchar>,
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
        recipe_id -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        amount -> Nullable<Varchar>,
    }
}

table! {
    procedures (id) {
        id -> Int4,
        recipe_id -> Nullable<Int4>,
        number -> Nullable<Int4>,
        discription -> Nullable<Varchar>,
        image_path -> Nullable<Varchar>,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        title -> Varchar,
        thumbnail_path -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        hash -> Nullable<Varchar>,
    }
}

table! {
    user_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        hash -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        user_type -> Nullable<Int4>,
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
