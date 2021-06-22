table! {
    names (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

table! {
    personnel (id) {
        id -> Int4,
        unit_id -> Nullable<Int4>,
        first_name -> Varchar,
        last_name -> Varchar,
        age -> Int4,
        nationality -> Varchar,
        rank -> Int4,
        state -> Varchar,
    }
}

table! {
    ranks (id) {
        id -> Int4,
        level -> Int4,
        name -> Varchar,
        title -> Varchar,
        nationality -> Varchar,
    }
}

table! {
    units (id) {
        id -> Int4,
        parent_id -> Nullable<Int4>,
        name -> Varchar,
        unit_type -> Varchar,
        kills -> Int4,
        losses -> Int4,
        leader_id -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    names,
    personnel,
    ranks,
    units,
);
