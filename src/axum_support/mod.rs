use crate::hyper_tokio::SlackClientHyperConnector;
use crate::listener::SlackClientEventsListenerEnvironment;
use hyper_util::client::legacy::connect::Connect;
use std::sync::Arc;

mod slack_events_middleware;
pub use slack_events_middleware::SlackEventsApiMiddleware;

pub struct SlackEventsAxumListener<
    H: 'static + Send + Sync + Connect + Clone,
    S: Send + Sync + 'static + Clone,
> {
    pub environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
    pub state: S,
}

impl<H: 'static + Send + Sync + Connect + Clone, S: Send + Sync + 'static + Clone>
    SlackEventsAxumListener<H, S>
{
    pub fn new(
        environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
        state: S,
    ) -> Self {
        Self { environment, state }
    }
}

mod slack_oauth_routes;
pub use slack_oauth_routes::*;

mod slack_events_extractors;
pub use slack_events_extractors::SlackEventsExtractors;
