table! {
    pages (id) {
        id -> Uuid,
        slug -> VarChar,
        html -> VarChar,
        css -> VarChar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
