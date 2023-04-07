use super::qos::Qos;

pub struct SubscriptionRequest {
    topic_filter: String,
    qos: Qos,
    no_local: bool,
    retain_as_published: bool,
    retain_handling_option: RetainHandlingOption
}

pub enum RetainHandlingOption {
    SendAtSubscribeTime,
    SendIfSubscriptionDoesNotExists,
    DonNotSend
}

impl SubscriptionRequest {
    pub fn new(topic_filter: &str, qos: Qos) -> SubscriptionRequest {
        SubscriptionRequest {
            topic_filter: topic_filter.to_string(),
            qos,
            no_local: false,
            retain_as_published: false,
            retain_handling_option: RetainHandlingOption::SendAtSubscribeTime
        }
    }
}
