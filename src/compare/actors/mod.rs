
mod actor;
mod bom;
mod dxf;
mod handle;
mod sndb;

pub use super::*;
pub use actor::*;
pub use bom::bom_db;
pub use dxf::*;
pub use handle::*;
pub use sndb::sn_db;

type Mark = String;
type Qty = u32;

pub use crossbeam::channel as QueueChannel;
pub use tokio::sync::mpsc as ResultChannel;
pub type ActorReceiver  = QueueChannel::Receiver<ActorTask>;
pub type ActorSender    = ResultChannel::Sender<ActorResult>;
pub type HandleReceiver = ResultChannel::Receiver<ActorResult>;
pub type HandleSender   = QueueChannel::Sender<ActorTask>;
