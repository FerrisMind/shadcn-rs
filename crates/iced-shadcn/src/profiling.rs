#[cfg(any(feature = "profile-tracing", feature = "profile-tracy"))]
use std::sync::Once;

#[cfg(feature = "profile-tracy")]
pub(crate) type ProfileGuard = tracy_client::Span;

#[cfg(all(feature = "profile-tracing", not(feature = "profile-tracy")))]
pub(crate) type ProfileGuard = tracing::span::EnteredSpan;

#[cfg(not(any(feature = "profile-tracing", feature = "profile-tracy")))]
pub(crate) struct ProfileGuard;

#[cfg(any(feature = "profile-tracing", feature = "profile-tracy"))]
static PROFILE_RUNTIME_INIT: Once = Once::new();

pub fn init_runtime() {
    #[cfg(feature = "profile-tracy")]
    PROFILE_RUNTIME_INIT.call_once(|| {
        let _ = tracy_client::Client::start();
    });

    #[cfg(all(feature = "profile-tracing", not(feature = "profile-tracy")))]
    PROFILE_RUNTIME_INIT.call_once(|| {
        use tracing_subscriber::layer::SubscriberExt;

        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn,iced_shadcn=trace"));

        let subscriber = tracing_subscriber::registry().with(filter).with(
            tracing_subscriber::fmt::layer()
                .compact()
                .without_time()
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
                .with_target(false),
        );

        let _ = tracing::subscriber::set_global_default(subscriber);
    });
}

#[cfg(feature = "profile-tracy")]
pub(crate) fn profile_span(name: &'static str) -> ProfileGuard {
    tracy_client::Client::running()
        .expect("profile_span without a running Tracy client")
        .span_alloc(Some(name), module_path!(), file!(), line!(), 0)
}

#[cfg(all(feature = "profile-tracing", not(feature = "profile-tracy")))]
pub(crate) fn profile_span(name: &'static str) -> ProfileGuard {
    tracing::trace_span!("profile", section = name).entered()
}

#[cfg(not(any(feature = "profile-tracing", feature = "profile-tracy")))]
pub(crate) fn profile_span(_name: &'static str) -> ProfileGuard {
    ProfileGuard
}
