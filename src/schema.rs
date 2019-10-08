table! {
    swp_article (id) {
        id -> Int4,
        slug -> Varchar,
        status -> Varchar,
        title -> Varchar,
        body -> Text,
        lead -> Text,
        route_id -> Int4,
        comments_count -> Int4,
        extra -> Nullable<Text>,
        metadata -> Nullable<Text>,
        // feature_media -> Int4,
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

table! {
    swp_image_rendition (id) {
        id -> Int4,
        media_id -> Int4,
        image_id -> Int4,
        width -> Int4,
        height -> Int4,
        name -> Varchar,
    }
}

table! {
    swp_author (id) {
        id -> Int4,
        name -> Varchar,
        role -> Varchar,
        job_title -> Varchar,
        biography -> Nullable<Text>,
        slug -> Nullable<Varchar>,
        twitter -> Nullable<Varchar>,
        facebook -> Nullable<Varchar>,
        instagram -> Nullable<Varchar>,
    }
}

table! {
    swp_article_author (article_id) {
        article_id -> Int4,
        author_id -> Int4,
    }
}

table! {
    swp_keyword (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    swp_article_keyword (article_id) {
        article_id -> Int4,
        keyword_id -> Int4,
    }
}

table! {
    swp_article_statistics (id) {
        id -> Int4,
        article_id -> Int4,
        page_views_number -> Int4,
    }
}

joinable!(swp_article -> swp_route (route_id));
joinable!(swp_article_media -> swp_image (image_id));
joinable!(swp_image_rendition -> swp_image (image_id));
joinable!(swp_image_rendition -> swp_article_media (media_id));

joinable!(swp_article_author -> swp_author (author_id));
joinable!(swp_article_author -> swp_article (article_id));

joinable!(swp_article_keyword -> swp_article (article_id));
joinable!(swp_article_keyword -> swp_keyword (keyword_id));

joinable!(swp_article_statistics -> swp_article (article_id));

allow_tables_to_appear_in_same_query!(
    swp_article,
    swp_route,
    swp_image,
    swp_article_media,
    swp_image_rendition,
    swp_article_author,
    swp_author,
    swp_article_keyword,
    swp_keyword,
    swp_article_statistics
);
