use sea_query::Iden;

#[derive(Iden)]
pub(super) enum Tweets {
    Table,
    ID,
    Title,
    Description,
    Signature,
    Hash,
    UserID,
}

#[derive(Iden)]
pub(super) enum Users {
    Table,
    PublicKey,
    Username,
    ImageURL,
}
