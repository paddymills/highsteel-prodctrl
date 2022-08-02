
use crossbeam::channel;
use tokio::sync::mpsc;

type Queue = channel;
type Results = mpsc;

pub struct ChannelSet<S, R> {
    queue_send: Queue::Sender<S>,
    queue_recv: Queue::Receiver<S>,

    results_send: Results::Sender<R>,
    results_recv: Results::Receiver<R>
}

impl ChannelSet {
    pub fn new(q_size: u32, r_size: u32) -> Self {
        let (queue_send, queue_recv) = Queue::channel(q_size);
        let (results_send, results_recv) = Results::channel(r_size);

        Self { queue_send, queue_recv, results_send, results_recv }
    }

    pub fn get_child_ends(&self) {
        (results_send.clone(), queue_recv.clone())
    }
}
