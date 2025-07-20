use std::fmt;

use tracing::Event;
use tracing::Subscriber;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format;
use tracing_subscriber::fmt::format::Compact;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::fmt::format::Full;
use tracing_subscriber::fmt::format::Json;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::registry::LookupSpan;

pub enum FormatBehavior {
    Full(Format<Full>),
    Compact(Format<Compact>),
    Pretty(Format<Pretty>),
    Json(Format<Json>),
}

impl FormatBehavior {
    pub(crate) fn full() -> Self {
        Self::Full(format())
    }

    pub(crate) fn compact() -> Self {
        Self::Compact(format().compact())
    }

    pub(crate) fn pretty() -> Self {
        Self::Pretty(format().pretty())
    }

    pub(crate) fn json() -> Self {
        Self::Json(format().json())
    }
}

impl<S, N> FormatEvent<S, N> for FormatBehavior
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: 'static + for<'writer> FormatFields<'writer>,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        match self {
            Self::Full(format) => format.format_event(ctx, writer, event),
            Self::Compact(format) => format.format_event(ctx, writer, event),
            Self::Pretty(format) => format.format_event(ctx, writer, event),
            Self::Json(format) => format.format_event(ctx, writer, event),
        }
    }
}
