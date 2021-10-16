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

// fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .constraints(
//             [
//                 Constraint::Length(9),
//                 Constraint::Min(8),
//                 Constraint::Length(7),
//             ]
//             .as_ref(),
//         )
//         .split(area);
//     draw_gauges(f, app, chunks[0]);
//     draw_charts(f, app, chunks[1]);
//     draw_text(f, chunks[2]);
// }

// fn draw_gauges<B>(frame: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .constraints(
//             [
//                 Constraint::Length(2),
//                 Constraint::Length(3),
//                 Constraint::Length(1),
//             ]
//             .as_ref(),
//         )
//         .margin(1)
//         .split(area);
//     let block = Block::default().borders(Borders::ALL).title("Graphs");
//     frame.render_widget(block, area);

//     // let label = format!("{:.2}%", app.progress * 100.0);
//     // let gauge = Gauge::default()
//     //     .block(Block::default().title("Gauge:"))
//     //     .gauge_style(
//     //         Style::default()
//     //             .fg(Color::Magenta)
//     //             .bg(Color::Black)
//     //             .add_modifier(Modifier::ITALIC | Modifier::BOLD),
//     //     )
//     //     .label(label)
//     //     .ratio(app.progress);
//     // frame.render_widget(gauge, chunks[0]);

//     // let sparkline = Sparkline::default()
//     //     .block(Block::default().title("Sparkline:"))
//     //     .style(Style::default().fg(Color::Green))
//     //     .data(&app.sparkline.points)
//     //     .bar_set(if app.enhanced_graphics {
//     //         symbols::bar::NINE_LEVELS
//     //     } else {
//     //         symbols::bar::THREE_LEVELS
//     //     });
//     // f.render_widget(sparkline, chunks[1]);

//     // let line_gauge = LineGauge::default()
//     //     .block(Block::default().title("LineGauge:"))
//     //     .gauge_style(Style::default().fg(Color::Magenta))
//     //     .line_set(if app.enhanced_graphics {
//     //         symbols::line::THICK
//     //     } else {
//     //         symbols::line::NORMAL
//     //     })
//     //     .ratio(app.progress);
//     // frame.render_widget(line_gauge, chunks[2]);
// }

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

// fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
//         .direction(Direction::Horizontal)
//         .split(area);
//     // let up_style = Style::default().fg(Color::Green);
//     // let failure_style = Style::default()
//     //     .fg(Color::Red)
//     //     .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
//     // let rows = app.servers.iter().map(|s| {
//     //     let style = if s.status == "Up" {
//     //         up_style
//     //     } else {
//     //         failure_style
//     //     };
//     //     Row::new(vec![s.name, s.location, s.status]).style(style)
//     // });
//     // let table = Table::new(rows)
//     //     .header(
//     //         Row::new(vec!["Server", "Location", "Status"])
//     //             .style(Style::default().fg(Color::Yellow))
//     //             .bottom_margin(1),
//     //     )
//     //     .block(Block::default().title("Servers").borders(Borders::ALL))
//     //     .widths(&[
//     //         Constraint::Length(15),
//     //         Constraint::Length(15),
//     //         Constraint::Length(10),
//     //     ]);
//     // f.render_widget(table, chunks[0]);

//     let map = Canvas::default()
//         .block(Block::default().title("World").borders(Borders::ALL))
//         .paint(|ctx| {
//             ctx.draw(&Map {
//                 color: Color::White,
//                 resolution: MapResolution::High,
//             });
//             ctx.layer();
//             ctx.draw(&Rectangle {
//                 x: 0.0,
//                 y: 30.0,
//                 width: 10.0,
//                 height: 10.0,
//                 color: Color::Yellow,
//             });
//             // for (i, s1) in app.servers.iter().enumerate() {
//             //     for s2 in &app.servers[i + 1..] {
//             //         ctx.draw(&Line {
//             //             x1: s1.coords.1,
//             //             y1: s1.coords.0,
//             //             y2: s2.coords.0,
//             //             x2: s2.coords.1,
//             //             color: Color::Yellow,
//             //         });
//             //     }
//             // }
//             // for server in &app.servers {
//             //     let color = if server.status == "Up" {
//             //         Color::Green
//             //     } else {
//             //         Color::Red
//             //     };
//             //     ctx.print(server.coords.1, server.coords.0, "X", color);
//             // }
//         })
//         .marker(if app.enhanced_graphics {
//             symbols::Marker::Braille
//         } else {
//             symbols::Marker::Dot
//         })
//         .x_bounds([-180.0, 180.0])
//         .y_bounds([-90.0, 90.0]);
//     f.render_widget(map, chunks[1]);
// }

// fn draw_third_tab<B>(f: &mut Frame<B>, _app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
//         .split(area);
//     let colors = [
//         Color::Reset,
//         Color::Black,
//         Color::Red,
//         Color::Green,
//         Color::Yellow,
//         Color::Blue,
//         Color::Magenta,
//         Color::Cyan,
//         Color::Gray,
//         Color::DarkGray,
//         Color::LightRed,
//         Color::LightGreen,
//         Color::LightYellow,
//         Color::LightBlue,
//         Color::LightMagenta,
//         Color::LightCyan,
//         Color::White,
//     ];
//     let items: Vec<Row> = colors
//         .iter()
//         .map(|c| {
//             let cells = vec![
//                 Cell::from(Span::raw(format!("{:?}: ", c))),
//                 Cell::from(Span::styled("Foreground", Style::default().fg(*c))),
//                 Cell::from(Span::styled("Background", Style::default().bg(*c))),
//             ];
//             Row::new(cells)
//         })
//         .collect();
//     let table = Table::new(items)
//         .block(Block::default().title("Colors").borders(Borders::ALL))
//         .widths(&[
//             Constraint::Ratio(1, 3),
//             Constraint::Ratio(1, 3),
//             Constraint::Ratio(1, 3),
//         ]);
//     f.render_widget(table, chunks[0]);
// }
