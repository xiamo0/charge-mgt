use std::collections::VecDeque;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub index: usize,
    pub at: SystemTime,
    pub text: String,
}

pub struct Log {
    sent: VecDeque<HistoryEntry>,
    recv: VecDeque<HistoryEntry>,
    next_index: usize,
    capacity: usize,
}

impl Log {
    pub fn new(capacity: usize) -> Self {
        Self {
            sent: VecDeque::with_capacity(capacity),
            recv: VecDeque::with_capacity(capacity),
            next_index: 1,
            capacity,
        }
    }

    fn push(q: &mut VecDeque<HistoryEntry>, entry: HistoryEntry, capacity: usize) {
        if q.len() >= capacity {
            q.pop_front();
        }
        q.push_back(entry);
    }

    pub fn record_sent(&mut self, text: String) -> usize {
        let idx = self.next_index;
        self.next_index += 1;
        Self::push(
            &mut self.sent,
            HistoryEntry {
                index: idx,
                at: SystemTime::now(),
                text,
            },
            self.capacity,
        );
        idx
    }

    pub fn record_recv(&mut self, text: String) {
        let idx = self.next_index;
        self.next_index += 1;
        Self::push(
            &mut self.recv,
            HistoryEntry {
                index: idx,
                at: SystemTime::now(),
                text,
            },
            self.capacity,
        );
    }

    pub fn sent_history(&self) -> impl Iterator<Item = &HistoryEntry> {
        self.sent.iter()
    }

    pub fn last_recv(&self) -> Option<&HistoryEntry> {
        self.recv.back()
    }

    pub fn last_sent(&self) -> Option<&HistoryEntry> {
        self.sent.back()
    }

    pub fn find_sent_by_index(&self, index: usize) -> Option<&HistoryEntry> {
        self.sent.iter().find(|e| e.index == index)
    }

    pub fn sent_count(&self) -> usize {
        self.sent.len()
    }
}
