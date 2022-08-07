use sea_query::Iden;

#[derive(Iden)]
pub(super) enum Tweets {
    Table,
    ID,
    Title,
    Description,
    Timestamp,
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
