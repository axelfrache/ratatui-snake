use crate::app::App;
use crate::game::Direction;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_input(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('r') => app.restart(),
        KeyCode::Char(' ') => app.toggle_pause(),
        KeyCode::Up | KeyCode::Char('w') => app.snake.set_direction(Direction::Up),
        KeyCode::Down | KeyCode::Char('s') => app.snake.set_direction(Direction::Down),
        KeyCode::Left | KeyCode::Char('a') => app.snake.set_direction(Direction::Left),
        KeyCode::Right | KeyCode::Char('d') => app.snake.set_direction(Direction::Right),
        _ => {}
    }
}
