table! {
    swp_article (id) {
        id -> Int4,
        slug -> Varchar,
        title -> Varchar,
        body -> Text,
        lead -> Text,
        route_id -> Int4,
    }
}

table! {
    swp_route (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    swp_article_media (id) {
        id -> Int4,
        article_id -> Int4,
        image_id -> Int4,
        key -> Varchar,
        body -> Text,
        description -> Text,
        located -> Varchar,
        by_line -> Varchar,
        mimetype -> Varchar,
        usage_terms -> Text,
    }
}

table! {
    swp_image (id) {
        id -> Int4,
        asset_id -> Varchar,
        file_extension -> Varchar,
    }
}

joinable!(swp_article -> swp_route (route_id));
joinable!(swp_article_media -> swp_image (image_id));

allow_tables_to_appear_in_same_query!(
    swp_article,
    swp_route,
    swp_image,
    swp_article_media
);
