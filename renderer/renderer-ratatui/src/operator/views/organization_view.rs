use crate::operator::routes::{OrganizationTab, OrganizationView};
use crate::operator::views::organization_budget::render_organization_budget;
use crate::operator::views::organization_details::render_organization_details;
use crate::operator::views::render_hiring;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::Tabs;
use renderer_api::ClientGameState;

pub fn render_organization_view(
    client_game_state: &ClientGameState,
    frame: &mut Frame,
    main_area: &Rect,
    organization_view: &OrganizationView,
) {
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(*main_area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(inner_chunks[1]);

    let left_pane = chunks[0];
    let right_pane = chunks[1];

    draw_tab_header(frame, inner_chunks[0], organization_view.tab);

    match organization_view.tab {
        OrganizationTab::Detail => render_organization_details(
            client_game_state,
            frame,
            &left_pane,
            &right_pane,
            &organization_view.organization_id,
            &organization_view.selected_index,
        ),
        OrganizationTab::Budget => render_organization_budget(
            client_game_state,
            frame,
            &left_pane,
            &right_pane,
            &organization_view.organization_id,
            &organization_view.selected_index,
            &organization_view.marketing,
            &organization_view.training,
            &organization_view.rnd,
        ),
        OrganizationTab::Hiring => render_hiring(
            client_game_state,
            frame,
            &left_pane,
            &right_pane,
            &organization_view.organization_id,
            &organization_view.selected_index,
        ),
    }
}

fn draw_tab_header(frame: &mut Frame, rect: Rect, active_tab: OrganizationTab) {
    let titles = vec!["Detail", "Budget", "Hiring"]
        .into_iter()
        .map(Line::from)
        .collect::<Vec<Line>>();

    let selected_index = match active_tab {
        OrganizationTab::Detail => 0,
        OrganizationTab::Budget => 1,
        OrganizationTab::Hiring => 2,
    };

    let tabs = Tabs::new(titles)
        .select(selected_index)
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(tabs, rect);
}
