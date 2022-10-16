use sea_query::Iden;

#[derive(Iden)]
pub enum Tweets {
    Table,
    Id,
    Text,
    Timestamp,
    UserId,
    Signature,
    Hash,
    PreviousId,
}

#[derive(Iden)]
pub enum Users {
    Table,
    PublicKey,
    Username,
    ImageURL,
}
