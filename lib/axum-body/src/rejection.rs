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
    #[status = BAD_REQUEST]
    #[body = "Failed to buffer the request body"]
    pub struct UnknownBodyError(Error);
}

define_rejection! {
    #[status = UNPROCESSABLE_ENTITY]
    #[body = "Failed to process non empty entity"]
    pub struct NonEmptyRejectionError(Error);
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "Expected request with empty body"]
    pub struct NotEmptyRejection;
}

composite_rejection! {
    pub enum EmptyRejection {
        UnknownBodyError,
        NonEmptyRejectionError,
        NotEmptyRejection,
    }
}
