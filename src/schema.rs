#[derive(Debug, DbEnum)]
pub enum ClientCategory {
    Privileged,
    Public,
}

table! {
    use super::ClientCategory;
    clients (id) {
        id -> Uuid,
        name -> Text,
        secret -> Text,
        category -> ClientCategory,
        owner -> Uuid,
    }
}

table! {
    people (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        hash -> Text,
    }
}

joinable!(clients -> people (owner));

allow_tables_to_appear_in_same_query!(
    clients,
    people,
);
