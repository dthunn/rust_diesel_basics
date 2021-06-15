table! {
    employees (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    posts,
);
