table! {
    todos (id) {
        id -> Uuid,
        title -> Varchar,
        checked -> Bool,
        created_at -> Timestamp,
    }
}
