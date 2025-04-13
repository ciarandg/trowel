use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct PlanningViewState {
    rx: Receiver<String>,
    pub plan_stdout: Vec<String>,
}

impl PlanningViewState {
    const MPSC_BUFFER_SIZE: usize = 100;

    pub fn new() -> (Self, Sender<String>) {
        let (tx, rx) = mpsc::channel(Self::MPSC_BUFFER_SIZE);
        (
            Self {
                rx,
                plan_stdout: Vec::new(),
            },
            tx,
        )
    }

    pub async fn next_line(&mut self) -> Option<()> {
        tokio::select! {
            Some(line) = self.rx.recv() => {
                self.plan_stdout.push(line);
                Some(())
            }
            else => None
        }
    }
}
