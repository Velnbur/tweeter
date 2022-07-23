use sea_query::Iden;

#[derive(Iden)]
pub(crate) enum Tasks {
    Table,
    ID,
    Title,
    Description,
    Priority,
}
