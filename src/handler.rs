use crate::app::{App, AppResult, FocusedBlock, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.mode {
        Mode::Normal => match key_event.code {
            // Change mode to Insert
            KeyCode::Char('i') => {
                app.mode = Mode::Insert;
                app.focused_block = FocusedBlock::Prompt;
            }

            // Quit the app
            KeyCode::Char('q') => {
                app.running = false;
            }

            // TODO: handle shift + enter. Limitation from crossterm
            KeyCode::Enter => {
                let user_input: String = app.input.drain(3..).collect();
                let user_input = user_input.trim();
                if user_input.is_empty() {
                    return Ok(());
                }
                app.messages.push(format!(" : {}\n", user_input));

                let assisstant_message =
                    app.gpt.ask(user_input).await.unwrap_or("Error".to_string());
                app.messages.push(format!("🪄: {}\n", assisstant_message));

                app.messages.push("\n".to_string());
            }

            // scroll down
            KeyCode::Char('j') => {
                app.scroll += 1;
            }

            // scroll up
            KeyCode::Char('k') => {
                app.scroll -= 1;
            }

            // Clear the prompt
            KeyCode::Char('d') => {
                if app.previous_key == KeyCode::Char('d') {
                    app.input = String::from(">_ ");
                    app.scroll = 0;
                }
            }

            // Clear the prompt and the chat
            KeyCode::Char('l') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.input = String::from(">_ ");
                    app.messages = Vec::new();
                    app.scroll = 0;
                }
            }

            // Switch the focus
            KeyCode::Tab => match app.focused_block {
                FocusedBlock::Chat => app.focused_block = FocusedBlock::Prompt,
                FocusedBlock::Prompt => app.focused_block = FocusedBlock::Chat,
            },

            // kill the app
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.running = false;
                }
            }

            // Help popup
            KeyCode::Char('h') => {
                app.show_help_popup = true;
            }

            KeyCode::Esc => {
                if app.show_help_popup {
                    app.show_help_popup = false;
                }
            }

            _ => {}
        },

        Mode::Insert => match key_event.code {
            KeyCode::Enter => app.input.push('\n'),

            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                if app.input.len() > 3 {
                    app.input.pop();
                }
            }
            KeyCode::Esc => {
                app.mode = Mode::Normal;
            }
            _ => {}
        },
    }

    app.previous_key = key_event.code;
    Ok(())
}
