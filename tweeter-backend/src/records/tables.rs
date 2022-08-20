use sea_query::Iden;

#[derive(Iden)]
pub(super) enum Tweets {
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
pub(super) enum Users {
    Table,
    PublicKey,
    Username,
    ImageURL,
}
