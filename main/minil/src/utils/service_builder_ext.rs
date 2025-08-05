use axum::middleware::FromFnLayer;
use tower::ServiceBuilder;
use tower::layer::util::Stack;

pub(crate) trait ServiceBuilderExt<L> {
    fn middleware_fn<F, T>(self, f: F) -> ServiceBuilder<Stack<FromFnLayer<F, (), T>, L>>;

    fn middleware_fn_with_state<F, S, T>(
        self,
        state: S,
        f: F,
    ) -> ServiceBuilder<Stack<FromFnLayer<F, S, T>, L>>;
}

impl<L> ServiceBuilderExt<L> for ServiceBuilder<L> {
    fn middleware_fn<F, T>(self, f: F) -> ServiceBuilder<Stack<FromFnLayer<F, (), T>, L>> {
        self.layer(axum::middleware::from_fn(f))
    }

    fn middleware_fn_with_state<F, S, T>(
        self,
        state: S,
        f: F,
    ) -> ServiceBuilder<Stack<FromFnLayer<F, S, T>, L>> {
        self.layer(axum::middleware::from_fn_with_state(state, f))
    }
}
