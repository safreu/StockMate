use crate::modules::accounts::domain::SessionToken;

pub trait SessionTokenGenerator: Send + Sync {
    fn generate(&self) -> SessionToken;
}

pub trait SessionTokenHasher: Send + Sync {
    fn hash(&self, token: &SessionToken) -> String;
}
