// Smooth Replay step 1: records timed events. First event type: block breaks.
use std::sync::Mutex;
use std::time::Instant;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub millis: u128,
    pub pos: [i32; 3],
}

static RECORDING: Mutex<Option<(Instant, Vec<Event>)>> = Mutex::new(None);

pub fn record_event(pos: [i32; 3]) -> usize {
    let mut guard = RECORDING.lock().unwrap_or_else(|e| e.into_inner());
    let (start, events) = guard.get_or_insert_with(|| (Instant::now(), Vec::new()));
    events.push(Event {
        millis: start.elapsed().as_millis(),
        pos,
    });
    events.len()
}

pub fn block_break_message(pos: [i32; 3]) -> String {
    format!(
        "[example_mod] Breaking block at {}, {}, {}",
        pos[0], pos[1], pos[2]
    )
}

pub fn before_break(pos: [i32; 3]) {
    let count = record_event(pos);
    eprintln!("{} (event #{})", block_break_message(pos), count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_preserves_signed_block_coordinates() {
        assert_eq!(
            block_break_message([-3, 64, 12]),
            "[example_mod] Breaking block at -3, 64, 12"
        );
    }
}
