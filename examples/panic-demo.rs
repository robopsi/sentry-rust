#[macro_use]
extern crate sentry;

fn main() {
    let _sentry = sentry::init((
        "https://a94ae32be2584e0bbd7a4cbb95971fee@sentry.io/1041156",
        sentry::ClientOptions {
            release: sentry_crate_release!(),
            ..Default::default()
        },
    ));
    sentry::integrations::panic::register_panic_handler();

    sentry::with_scope(|| {
        sentry::configure_scope(|scope| {
            scope.set_tag("foo", "bar");
        });
        panic!("Holy shit everything is on fire!");
    });
}
