use crate::client::GoshuinRepositoryClient;

pub struct Context {
    client: GoshuinRepositoryClient,
}

impl Context {
    pub fn new(client: GoshuinRepositoryClient) -> Context {
        Context { client }
    }
}
