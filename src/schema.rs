// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Text,
        code -> Text,
        name -> Text,
        account_type -> Text,
        parent_id -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    entries (id) {
        id -> Text,
        transaction_id -> Text,
        account_id -> Text,
        debit_amount -> Text,
        credit_amount -> Text,
        description -> Nullable<Text>,
        created_at -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Text,
        reference -> Text,
        description -> Text,
        transaction_date -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(entries -> accounts (account_id));
diesel::joinable!(entries -> transactions (transaction_id));

diesel::allow_tables_to_appear_in_same_query!(accounts, entries, transactions,);
