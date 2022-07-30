use sea_query::Iden;

#[derive(Iden)]
pub(super) enum Tweets {
    Table,
    ID,
    Title,
    Description,
    UserID,
}

#[derive(Iden)]
pub(super) enum Users {
    Table,
    PublicKey,
}