use super::Qos;

#[derive(Clone)]
pub struct SubscriptionRequest {
    topic_filter: String,
    max_qos: Qos,
    no_local: bool,
    retain_as_published: bool,
    retain_handling_option: RetainHandlingOption
}

#[derive(Clone)]
pub enum RetainHandlingOption {
    SendAtSubscribeTime,
    SendIfSubscriptionDoesNotExists,
    DonNotSend
}

impl SubscriptionRequest {
    pub fn new(topic_filter: &str, max_qos: Qos) -> SubscriptionRequest {
        SubscriptionRequest {
            topic_filter: topic_filter.to_string(),
            max_qos,
            no_local: false,
            retain_as_published: false,
            retain_handling_option: RetainHandlingOption::SendAtSubscribeTime
        }
    }
}
