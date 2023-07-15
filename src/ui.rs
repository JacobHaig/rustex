use crate::{
    app::{App, WindowState},
    widgets,
};
use strum::*;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(4),
                Constraint::Length(8), // Console/Informational
            ]
            .as_ref(),
        )
        .split(frame.size());

    draw_header(frame, app, chunks[0]);
    draw_viewport(frame, app, chunks[1]);
    draw_console(frame, chunks[2]);
}

fn draw_header<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let titles = app
        .context
        .file_manager_window
        .file_list
        .iter()
        .map(|menu| {
            Spans::from(Span::styled(
                menu.name.to_str().unwrap(),
                Style::default().fg(Color::Green),
            ))
        })
        .collect();

    let tabs = tui::widgets::Tabs::new(titles)
        .block(Block::default().borders(Borders::TOP).title(Span::styled(
            "Open Files",
            Style::default().fg(Color::Magenta),
        )))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.context.file_manager_window.current_file_index);

    frame.render_widget(tabs, area);
}

// The viewport is the 'main' area that the user interacts with.
// It is made up the interaction menu and the editor piece.
fn draw_viewport<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let constraints = if app.context.current_window == WindowState::Navigation {
        vec![Constraint::Length(15), Constraint::Percentage(80)]
    } else {
        vec![Constraint::Percentage(0), Constraint::Percentage(100)]
    };

    let chunks = &Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);

    // Draw Left Side Panel
    draw_editor(app, frame, chunks);
    draw_interaction_menu(app, frame, chunks);
}

fn draw_editor<B: Backend>(app: &mut App, frame: &mut Frame<B>, chunks: &[Rect]) {
    let display_string = app
        .context
        .file_manager_window
        .current_file()
        .get_display_text();
    let display_text = Text::from(display_string);

    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        app.context
            .file_manager_window
            .current_file()
            .name
            .to_str()
            .unwrap(),
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));

    let paragraph = Paragraph::new(display_text)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, chunks[1]);
}

fn draw_interaction_menu<B: Backend>(app: &mut App, frame: &mut Frame<B>, chunks: &[Rect]) {
    let menu_items: Vec<ListItem> = widgets::navigation::ActionMenuWindow::iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.to_string()))]))
        .collect();

    let tasks = List::new(menu_items)
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP)
                .title(Span::styled(
                    "Menu",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(
        tasks,
        chunks[0],
        &mut app.context.navigation_window.list_state,
    );
}

fn draw_console<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![
        Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
        Spans::from(""),
        Spans::from(vec![
            Span::from("For example: "),
            Span::styled("under", Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled("the", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("rainbow", Style::default().fg(Color::Blue)),
            Span::raw("."),
        ]),
        Spans::from(vec![
            Span::raw("Oh and if you didn't "),
            Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(" you can "),
            Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
            Span::raw(" your "),
            Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::raw(".")
        ]),
        Spans::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        "Terminal",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
