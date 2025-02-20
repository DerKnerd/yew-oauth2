use crate::agent::Client;
use std::time::Duration;

use super::LoginOptions;

#[derive(Clone, Debug)]
pub struct AgentConfiguration<C: Client> {
    pub config: C::Configuration,
    pub scopes: Vec<String>,
    pub grace_period: Duration,
    pub audience: Option<String>,
    pub options: Option<LoginOptions>,
    pub valid_audiences: Option<Vec<String>>,
}

impl<C: Client> PartialEq for AgentConfiguration<C> {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
            && self.scopes == other.scopes
            && self.grace_period == other.grace_period
    }
}

impl<C: Client> Eq for AgentConfiguration<C> {}
