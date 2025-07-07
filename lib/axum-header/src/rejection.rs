use axum_core::__composite_rejection as composite_rejection;
use axum_core::__define_rejection as define_rejection;

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "Failed to deserialize header"]
    pub struct FailedToDeserializeHeader(Error);
}

composite_rejection! {
    pub enum HeaderRejection {
        FailedToDeserializeHeader,
    }
}
