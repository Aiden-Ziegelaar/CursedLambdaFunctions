use std::{future::Future, pin::Pin, sync::Arc, task::{Context, Poll, Waker}};

use tokio::sync::Mutex;

use axum::{routing::{get, post}, Router};
use futures::lock::BiLock;
use lambda_extension::*;

pub mod methods;
pub mod types;
pub mod actions;
pub mod env;

pub const DEFAULT_PROXY_PORT: u16 = 1337;

pub const LAMBDA_RUNTIME_API_PORT: u16 = 9000;

pub static LAMBDA_RUNTIME_API_VERSION: &str = "2018-06-01";

pub const EXTENSION_NAME: &str = "rustProxy";

async fn events_extension(signal: &Arc<Mutex<ShutdownSignalCompleter>>, event: LambdaEvent ) -> Result<(), Error>{
    match event.next {
        NextEvent::Shutdown(e) => {
            tracing::info!(event_type = "shutdown", event = ?e, "shutting down");
            signal.lock().await.complete().await;
        },
        NextEvent::Invoke(e) => {
            tracing::info!(event_type = "invoke", event = ?e, "invoking");
        },
    }
    Ok(())
}

enum State {
    Uninitialized,
    Pending(Waker),
    Complete,
}

impl State {
    fn new() -> Self {
        Self::Uninitialized
    }
}

pub struct ShutdownSignal {
    state: BiLock<State>,
}

pub struct ShutdownSignalCompleter {
    state: BiLock<State>,
}

impl ShutdownSignalCompleter {
    pub async fn complete(self: &mut Self) {
        let mut state = self.state.lock().await;

        match &mut *state {
            State::Uninitialized => *state = State::Complete,
            State::Pending(w) => {
                let waker = w.clone();
                *state = State::Complete;
                waker.wake();
            }
            State::Complete => {}
        }
    }
}

impl ShutdownSignal {
    pub fn new() -> (Self, ShutdownSignalCompleter) {
        let (a, b) = BiLock::new(State::new());
        (Self { state: a }, ShutdownSignalCompleter { state: b })
    }
}

impl Future for ShutdownSignal {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut state = match self.state.poll_lock(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(v) => v,
        };

        match &mut *state {
            State::Uninitialized => {
                *state = State::Pending(cx.waker().clone());
                Poll::Pending
            }
            State::Pending(w) => {
                *w = cx.waker().clone();
                Poll::Pending
            }
            State::Complete => Poll::Ready(()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    env::env::latch_runtime_env();

    let (waiter, completer) = ShutdownSignal::new();

    tokio::task::spawn(async {
        let app = Router::new()
        .route(format!("/{}/runtime/init/error", LAMBDA_RUNTIME_API_VERSION).as_str(), post(methods::init_error::post_init_error))
        .route(format!("/{}/runtime/invocation/next", LAMBDA_RUNTIME_API_VERSION).as_str(), get(methods::invocation_next::get_invocation_next))
        .route(format!("/{}/runtime/invocation/:invocation_id/response", LAMBDA_RUNTIME_API_VERSION).as_str(), post(methods::invocation_response::post_invocation_response))
        .route(format!("/{}/runtime/invocation/:invocation_id/error", LAMBDA_RUNTIME_API_VERSION).as_str(), post(methods::invocation_error::post_invocation_error));
        
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", DEFAULT_PROXY_PORT)).await.unwrap();
        
        axum::serve(listener, app)
            .with_graceful_shutdown(waiter).await.unwrap();
    });

    //actions::register_extension::register_extension().await;

    let completer_mutex = Arc::new(Mutex::new(completer));

    Extension::new()
        .with_extension_name(EXTENSION_NAME)
        .with_events_processor(service_fn( |event| events_extension(&completer_mutex, event)))
        .run().await?;

    Ok(())
}
