// @generated automatically by Diesel CLI.

diesel::table! {
    sys_dept (id) {
        #[max_length = 32]
        id -> Char,
        #[max_length = 30]
        name -> Nullable<Varchar>,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        #[max_length = 11]
        telephone -> Nullable<Varchar>,
        #[max_length = 200]
        address -> Nullable<Varchar>,
        #[max_length = 100]
        logo -> Nullable<Varchar>,
        #[max_length = 32]
        parent_id -> Nullable<Char>,
        seq_no -> Nullable<Integer>,
        status -> Nullable<Integer>,
        #[max_length = 30]
        create_by -> Nullable<Varchar>,
        create_time -> Nullable<Datetime>,
        #[max_length = 30]
        update_by -> Nullable<Varchar>,
        update_time -> Nullable<Datetime>,
        #[max_length = 200]
        remark -> Nullable<Varchar>,
    }
}
