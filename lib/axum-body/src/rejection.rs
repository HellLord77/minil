use axum_core::__composite_rejection as composite_rejection;
use axum_core::__define_rejection as define_rejection;
use axum_core::extract::rejection::BytesRejection;

define_rejection! {
    #[status = UNPROCESSABLE_ENTITY]
    #[body = "Failed to process either entity"]
    pub struct BothRejectionError(Error);
}

composite_rejection! {
    pub enum BothRejection {
        BothRejectionError,
        BytesRejection,
    }
}

define_rejection! {
    #[status = UNPROCESSABLE_ENTITY]
    #[body = "Failed to process both entities"]
    pub struct EitherRejectionError(Error);
}

composite_rejection! {
    pub enum EitherRejection {
        EitherRejectionError,
        BytesRejection,
    }
}

define_rejection! {
    #[status = UNPROCESSABLE_ENTITY]
    #[body = "Failed to process entity"]
    pub struct BodyNotEmpty;
}

composite_rejection! {
    pub enum EmptyRejection {
        BodyNotEmpty,
    }
}
