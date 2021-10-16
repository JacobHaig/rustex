use crate::app::App;

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
        .files
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
        .select(app.files.tab_index);
    frame.render_widget(tabs, area);
}

fn draw_viewport<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let constraints = if app.interaction_menu_visable {
        vec![Constraint::Length(15), Constraint::Percentage(80)]
    } else {
        vec![Constraint::Percentage(0), Constraint::Percentage(100)]
    };

    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);

    // Draw Left Side Panel
    if app.interaction_menu_visable {
        let menu_items: Vec<ListItem> = app
            .interaction_menu
            .items
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
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

        frame.render_stateful_widget(tasks, chunks[0], &mut app.interaction_menu.state);
    }

    let cursor_y = app.files.current_file_menu().cursor_y;
    let cursor_x = app.files.current_file_menu().cursor_x;

    // Insert the cursor and new line characters
    // into the string to be displayed
    let text_string: String = app
        .files
        .current_file_menu()
        .get_lines(0, 8)
        .iter()
        .enumerate()
        .map(|(y, strs)| {
            let mut s = strs.clone();
            if y == cursor_y {
                s.insert(cursor_x, '█');
            }
            s
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Place Holder for the Viewport
    let text = Text::from(text_string);

    // Text::from(
    //     r#""#,
    // );

    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        app.files.current_file_menu().name.to_str().unwrap(),
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, chunks[1]);
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
