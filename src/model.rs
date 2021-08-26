macro_rules! column_struct{
    (
        ($column_name:ident),
        ($column_type:ty)
    )=>{
        $column_name: $column_type
    }
}

#[macro_export]
macro_rules! model{
    (
        $table_name:ident
        [$(
            $column_name:ident,
            $column_type:ty $(,)?
            $(, pk=$column_pk:expr)?
            $(, null=$column_null:expr)?
            $(, default=$column_default:expr)?
            $(, unique=$column_unique:expr)?
            $(, readonly=$column_readonly:expr)? $(,)?
        ;)+]
    ) => {
        {
            pub struct $table_name{
                $($column_name: $column_type,)+
            }

        }
    }
}
