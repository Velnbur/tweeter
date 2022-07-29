use sea_query::Iden;

#[derive(Iden)]
pub(super) enum Tasks {
    Table,
    ID,
    Title,
    Description,
    Priority,
}

#[derive(Iden)]
pub(super) enum Users {
    Table,
    ID,
    PublicKey,
}