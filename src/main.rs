use color_eyre::eyre::Result;
use ratatui::{Frame, crossterm, layout::Constraint};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut term = ratatui::init();
    run(&mut term)?;
    Ok(())
}
fn run(term: &mut ratatui::DefaultTerminal) -> Result<()> {
    loop {
        if let Some(event) = check_events() {
            match event {
                EventType::Up => todo!(),
                EventType::Down => todo!(),
                EventType::Left => todo!(),
                EventType::Right => todo!(),
            }
        }
        term.draw(|frame| draw(frame))?;
    }
}
fn draw(frame: &mut Frame) {}
enum EventType {
    Up,
    Down,
    Left,
    Right,
}
fn check_events() -> Option<EventType> {
    None
}
