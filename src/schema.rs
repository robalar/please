table! {
    please_ids (id) {
        id -> Int4,
        creation -> Timestamptz,
        expiry -> Timestamptz,
        title -> Text,
        refresh_count -> Int4,
    }
}
