use crate::frame::Frame;
use crate::payload::SetupPayload;
use crate::spi::RSocket;
use futures::channel::{mpsc, oneshot};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::result::Result;
use std::sync::Arc;

pub type Tx<T> = mpsc::UnboundedSender<T>;
pub type Rx<T> = mpsc::UnboundedReceiver<T>;
pub type BoxResult<T> = Result<T, Box<dyn Send + Sync + Error>>;
pub type SafeFuture<T> = Pin<Box<dyn Send + Sync + Future<Output = T>>>;

pub(crate) type TxOnce<T> = oneshot::Sender<T>;
pub(crate) type RxOnce<T> = oneshot::Receiver<T>;

pub(crate) fn new_tx_rx_once<T>() -> (TxOnce<T>, RxOnce<T>) {
    oneshot::channel()
}

pub(crate) fn new_tx_rx<T>() -> (Tx<T>, Rx<T>) {
    mpsc::unbounded()
}

pub trait ClientTransport {
    fn attach(self, incoming: Tx<Frame>, sending: Rx<Frame>) -> SafeFuture<BoxResult<()>>;
}

pub trait ServerTransport {
    type Item;
    fn start(
        self,
        starter: Option<fn()>,
        acceptor: impl Fn(Self::Item) + Send + Sync + 'static,
    ) -> SafeFuture<BoxResult<()>>
    where
        Self::Item: ClientTransport + Sized;
}

pub type FnAcceptorWithSetup =
    fn(SetupPayload, Box<dyn RSocket>) -> Result<Box<dyn RSocket>, Box<dyn Error>>;

pub(crate) enum Acceptor {
    Simple(Arc<fn() -> Box<dyn RSocket>>),
    Generate(Arc<FnAcceptorWithSetup>),
    Empty(),
}
