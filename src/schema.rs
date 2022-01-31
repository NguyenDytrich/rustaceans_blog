table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Nullable<Text>,
        created_at -> Timestamptz,
        is_public -> Bool,
    }
}
