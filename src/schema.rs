table! {
    pages (id) {
        id -> Uuid,
        slug -> Varchar,
        html -> Varchar,
        css -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
