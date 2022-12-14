use crate::day07::device::Terminal;

pub fn solve() -> u32 {
    let mut terminal = Terminal::new();
    terminal.set_threshold(Some(100_000));
    terminal.get_size_of_dirs()
}
