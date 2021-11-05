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
        discription -> Nullable<Varchar>,
    }
}

table! {
    recipes_categories_categorization (id) {
        id -> Int4,
        recipe_id -> Int4,
        category_id -> Int4,
    }
}

table! {
    recipes_tags_tagging (id) {
        id -> Int4,
        recipe_id -> Int4,
        tag_id -> Int4,
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
    user_types_authorities_ownership (id) {
        id -> Int4,
        user_type_id -> Int4,
        authority_id -> Int4,
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
        password -> Varchar,
        api_key -> Nullable<Varchar>,
    }
}

table! {
    users_recipes_browsing_history (id) {
        id -> Int4,
        user_id -> Int4,
        recipe_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users_recipes_like (id) {
        id -> Int4,
        user_id -> Int4,
        recipe_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    authorities,
    categories,
    comments,
    ingredients,
    procedures,
    recipes,
    recipes_categories_categorization,
    recipes_tags_tagging,
    tags,
    user_types,
    user_types_authorities_ownership,
    users,
    users_recipes_browsing_history,
    users_recipes_like,
);
