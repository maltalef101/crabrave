use color_eyre::Result;
use tui::{
    backend::Backend,
    layout::{Alignment, Direction, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Ratio(2, 10),
            Constraint::Ratio(6, 10),
            Constraint::Ratio(2, 10),
        ])
        .split(frame.size());

    let inner_layout_middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(outer_layout[1]);

    let inner_layout_bottom = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(outer_layout[2]);

    let status = &app.mpd_status;
    let queue = &app.mpd_client.queue().expect("failed to get the queue from mpd");

    let cur_name = if let Some(queue_pos) = status.song {
        let song = &queue[queue_pos.pos as usize];
        let name = if let Some(meta_name) = &song.name {
            meta_name.to_owned()
        } else {
            song.file.to_owned()
        };

        name
    } else {
        String::from("Nothing currently playing")
    };

    let cur_artist = if let Some(queue_pos) = status.song {

    } else {
        String::from("");
    };

    frame.render_widget(
        Paragraph::new(cur_name)
        .block(
            Block::default()
                .title(" Playing ")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
        )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Left),
        inner_layout_bottom[0],
    );
}
