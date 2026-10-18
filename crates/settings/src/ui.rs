mod manager;

use clock_core::{
    ClockConfig, DEFINITIONS, MAX_ZONES, WidgetRecord, WidgetState, capture, definition,
    delete_widget, list_widgets, refresh_clock, rename_clock, save_clock, search_definitions,
    search_zones, validate_zone,
};
use std::{ffi::c_void, ptr};
use widget_api::WidgetSize;
use windows_reactor::*;

use crate::preview::{compact_preview, featured_artwork, featured_preview, preview, world_icon};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Page {
    Home,
    Discover,
    MyWidgets,
    Settings,
}
impl Page {
    fn tag(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Discover => "discover",
            Self::MyWidgets => "widgets",
            Self::Settings => "settings",
        }
    }
    fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "home" => Some(Self::Home),
            "discover" => Some(Self::Discover),
            "widgets" => Some(Self::MyWidgets),
            "settings" => Some(Self::Settings),
            _ => None,
        }
    }
}

pub struct WidgetApp {
    page: Page,
    records: Vec<WidgetRecord>,
    selected: Option<String>,
    selected_definition: Option<String>,
    discover_query: String,
    city_search: CitySearchState,
    name_draft: String,
    selected_city: Option<usize>,
    settings_expanded: bool,
    error: Option<String>,
    appearance_error: Option<String>,
    notice: Option<String>,
    unpin_target: Option<String>,
    narrow: bool,
    wide_home: bool,
    pane_open: bool,
    detail_open: bool,
    preview_expanded: bool,
    city_label_ref: ElementRef<TextBox>,
    instance_name_ref: ElementRef<TextBox>,
}

#[derive(Clone)]
pub enum Message {
    Activated(Option<String>),
    Foreground,
    Appearance,
    Navigate(String),
    Reload,
    Select(String),
    Back,
    Explore(String),
    DiscoverSearch(String),
    SelectDefinition(String),
    CitySearch(String),
    ChooseCity(String),
    AddCity(String),
    RemoveCity(usize),
    MoveCity(usize, isize),
    LabelCity(usize, String),
    SelectCity(usize),
    CloseCity,
    SettingsExpanded(bool),
    TogglePreview,
    NameDraft(String),
    SaveName,
    RefreshSelected,
    OpenBoard,
    AskUnpin,
    ConfirmUnpin(bool),
    WindowSize(WindowSize),
    PaneOpen(bool),
    ClearError,
}

impl WidgetApp {
    fn selected(&self) -> Option<&WidgetRecord> {
        self.records
            .iter()
            .find(|record| Some(&record.id) == self.selected.as_ref())
    }
    fn reset_editor(&mut self) {
        self.city_search.clear();
        self.selected_city = None;
        self.settings_expanded = false;
        self.preview_expanded = false;
        self.name_draft.clear();
    }
    fn reload(&mut self) {
        match list_widgets() {
            Ok(records) => {
                if self
                    .selected
                    .as_ref()
                    .is_some_and(|id| !records.iter().any(|record| &record.id == id))
                {
                    self.selected = None;
                    self.detail_open = false;
                    self.reset_editor();
                }
                if self
                    .unpin_target
                    .as_ref()
                    .is_some_and(|id| !records.iter().any(|record| &record.id == id))
                {
                    self.unpin_target = None;
                }
                self.records = records;
                self.error = None;
            }
            Err(error) => self.error = Some(format!("Could not read pinned widgets: {error}")),
        }
    }
    fn edit_clock(&mut self, edit: impl FnOnce(&mut ClockConfig) -> bool) {
        let Some(index) = self
            .records
            .iter()
            .position(|record| Some(&record.id) == self.selected.as_ref())
        else {
            return;
        };
        let Some(mut config) = self.records[index].clock().cloned() else {
            return;
        };
        if !edit(&mut config) {
            return;
        }
        match save_clock(&self.records[index].id, &config) {
            Ok(()) => {
                self.records[index].set_clock(config);
                self.records[index].last_update = windows_time::DateTime::now();
                self.error = None;
            }
            Err(error) => self.error = Some(format!("Could not update this widget: {error}")),
        }
    }
}

impl Component for WidgetApp {
    type Input = Option<String>;
    type Message = Message;
    fn create(input: &Self::Input, cx: &ComponentContext<Self>) -> Self {
        let _ = crate::lifecycle::listen(cx);
        let design_error = crate::design::install().err();
        let mut app = Self {
            page: Page::Home,
            records: Vec::new(),
            selected: None,
            selected_definition: Some("Clock".into()),
            discover_query: String::new(),
            city_search: CitySearchState::default(),
            name_draft: String::new(),
            selected_city: None,
            settings_expanded: false,
            error: None,
            appearance_error: design_error
                .map(|error| format!("Could not load the app’s appearance: {error}")),
            notice: None,
            unpin_target: None,
            narrow: false,
            wide_home: true,
            pane_open: true,
            detail_open: false,
            preview_expanded: false,
            city_label_ref: ElementRef::new(),
            instance_name_ref: ElementRef::new(),
        };
        app.reload();
        if let Some(id) = input {
            app.page = Page::MyWidgets;
            if let Some(record) = app.records.iter().find(|record| &record.id == id) {
                app.selected = Some(record.id.clone());
                app.name_draft = record
                    .clock()
                    .and_then(|config| config.name.clone())
                    .unwrap_or_default();
                app.detail_open = true;
            } else {
                app.notice =
                    Some("This widget is no longer pinned. Choose another instance below.".into());
            }
        }
        app
    }
    fn update(&mut self, message: Message, cx: &ComponentContext<Self>) {
        match message {
            Message::Appearance => {
                // Re-render themed imagery without reloading host records or changing editors.
                let _ = crate::lifecycle::listen(cx);
            }
            Message::Foreground => {
                self.reload();
                let _ = crate::lifecycle::listen(cx);
            }
            Message::Activated(route) => {
                crate::lifecycle::bring_to_front();
                self.reload();
                self.unpin_target = None;
                if let Some(id) = route {
                    self.page = Page::MyWidgets;
                    if self.records.iter().any(|record| record.id == id) {
                        self.selected = Some(id.clone());
                        self.reset_editor();
                        self.detail_open = true;
                        self.name_draft = self
                            .selected()
                            .and_then(|record| record.clock())
                            .and_then(|config| config.name.clone())
                            .unwrap_or_default();
                        self.notice = None;
                    } else {
                        self.selected = None;
                        self.detail_open = false;
                        self.reset_editor();
                        self.notice = Some(
                            "This widget is no longer pinned. Choose another instance below."
                                .into(),
                        );
                    }
                }
                let _ = crate::lifecycle::listen(cx);
            }
            Message::Navigate(tag) => {
                if let Some(page) = Page::from_tag(&tag) {
                    let page_changed = self.page != page;
                    self.page = page;
                    self.unpin_target = None;
                    self.reload();
                    self.notice = None;
                    if self.narrow && page_changed {
                        self.pane_open = false;
                        self.detail_open = false;
                    }
                    if page == Page::MyWidgets
                        && self.selected.is_none()
                        && let Some(record) = self.records.first()
                    {
                        self.selected = Some(record.id.clone());
                        self.name_draft = record
                            .clock()
                            .and_then(|clock| clock.name.clone())
                            .unwrap_or_default();
                        self.detail_open = !self.narrow;
                    }
                }
            }
            Message::Reload => self.reload(),
            Message::Select(id) => {
                self.page = Page::MyWidgets;
                self.unpin_target = None;
                self.selected = Some(id.clone());
                self.detail_open = true;
                self.reset_editor();
                self.notice = None;
                self.name_draft = self
                    .records
                    .iter()
                    .find(|record| record.id == id)
                    .and_then(|record| record.clock())
                    .and_then(|config| config.name.clone())
                    .unwrap_or_default();
            }
            Message::Back => self.detail_open = false,
            Message::DiscoverSearch(value) => self.discover_query = value,
            Message::Explore(id) => {
                self.page = Page::Discover;
                self.selected_definition = Some(id);
            }
            Message::SelectDefinition(id) => self.selected_definition = Some(id),
            Message::CitySearch(value) => self.city_search.text_changed(value),
            Message::ChooseCity(value) => self.city_search.suggestion_chosen(value),
            Message::AddCity(id) => {
                if validate_zone(&id) {
                    self.edit_clock(|config| config.add(id));
                    self.city_search.clear();
                    self.selected_city = None;
                } else {
                    self.error = Some(format!("Windows does not recognize {id}."));
                }
            }
            Message::RemoveCity(index) => {
                self.edit_clock(|config| {
                    if config.zones.len() <= 1 || index >= config.zones.len() {
                        return false;
                    }
                    config.zones.remove(index);
                    true
                });
                self.selected_city = None;
            }
            Message::MoveCity(index, delta) => {
                self.edit_clock(|config| {
                    let target = index as isize + delta;
                    if target < 0 || target as usize >= config.zones.len() {
                        return false;
                    }
                    config.zones.swap(index, target as usize);
                    true
                });
                let target = index as isize + delta;
                if target >= 0 {
                    self.selected_city = Some(target as usize);
                }
            }
            Message::LabelCity(index, value) => self.edit_clock(|config| {
                let Some(zone) = config.zones.get_mut(index) else {
                    return false;
                };
                let value: String = value.chars().take(40).collect();
                let label = (!value.trim().is_empty()).then_some(value);
                if zone.label == label {
                    return false;
                }
                zone.label = label;
                true
            }),
            Message::NameDraft(value) => self.name_draft = value,
            Message::SelectCity(index) => {
                self.selected_city = Some(index);
                self.settings_expanded = false;
            }
            Message::CloseCity => self.selected_city = None,
            Message::SettingsExpanded(value) => {
                self.settings_expanded = value;
                if value {
                    self.selected_city = None;
                }
            }
            Message::TogglePreview => self.preview_expanded = !self.preview_expanded,
            Message::SaveName => {
                let Some(record) = self.selected() else {
                    return;
                };
                let Some(mut config) = record.clock().cloned() else {
                    return;
                };
                if let Err(error) = config.set_name(&self.name_draft) {
                    self.error = Some(error.into());
                    return;
                }
                let id = record.id.clone();
                match rename_clock(&id, &config) {
                    Ok(()) => {
                        self.reload();
                        self.settings_expanded = false;
                        self.notice = Some("Instance name saved.".into());
                    }
                    Err(error) => {
                        self.error = Some(format!("Could not rename this widget: {error}"))
                    }
                }
            }
            Message::RefreshSelected => {
                if let Some(id) = self.selected.clone() {
                    match refresh_clock(&id) {
                        Ok(()) => self.reload(),
                        Err(error) => {
                            self.error = Some(format!("Could not refresh this widget: {error}"))
                        }
                    }
                }
            }
            Message::OpenBoard => {
                if !open_board() {
                    self.error =
                        Some("Could not open the widget board. Press Win+W to open it.".into());
                }
            }
            Message::AskUnpin => self.unpin_target = self.selected.clone(),
            Message::ConfirmUnpin(confirmed) => {
                if confirmed && let Some(id) = self.unpin_target.as_ref() {
                    match delete_widget(id) {
                        Ok(()) => {
                            self.selected = None;
                            self.detail_open = false;
                            self.reset_editor();
                            self.reload();
                            if let Some(next) = self.records.first() {
                                self.selected = Some(next.id.clone());
                                self.detail_open = true;
                                self.name_draft = next
                                    .clock()
                                    .and_then(|clock| clock.name.clone())
                                    .unwrap_or_default();
                            }
                            self.notice = Some("Widget unpinned.".into());
                        }
                        Err(error) => {
                            self.error = Some(format!("Could not unpin this widget: {error}"))
                        }
                    }
                }
                self.unpin_target = None;
            }
            Message::WindowSize(size) => {
                if self.narrow != (size.width < 1100.0) {
                    self.pane_open = size.width >= 1100.0;
                }
                self.narrow = size.width < 1100.0;
                self.wide_home = size.width >= 1300.0;
            }
            Message::PaneOpen(open) => self.pane_open = open,
            Message::ClearError => {
                self.error = None;
                self.appearance_error = None;
            }
        }
    }
    fn view(&self, _: &Self::Input, cx: &mut ViewContext<Self>) -> View {
        cx.window_title("Widgets RS");
        cx.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::MicaAlt)
                .client_size(1420.0, 860.0)
                .constraints(WindowConstraints {
                    min_width: Some(650.0),
                    min_height: Some(520.0),
                    ..Default::default()
                }),
        );
        cx.on_window_size(cx.callback(Message::WindowSize));
        let page: View = match self.page {
            Page::Home => self.home(cx),
            Page::Discover => self.discover(cx),
            Page::MyWidgets => self.my_widgets(cx),
            Page::Settings => self.settings(cx),
        };
        let nav_items = [
            (Page::Home, "Home", Symbol::Home),
            (Page::Discover, "Discover", Symbol::World),
            (Page::MyWidgets, "My widgets", Symbol::ViewAll),
            (Page::Settings, "Settings", Symbol::Setting),
        ]
        .into_iter()
        .map(|(page, title, symbol)| {
            KeyedView::new(
                page.tag(),
                NavigationViewItem::new()
                    .tag(page.tag())
                    .margin(Thickness::xy(if self.narrow { 0.0 } else { 8.0 }, 4.0))
                    .automation_id(format!("Nav_{}", page.tag()))
                    .is_selected(page == self.page)
                    .slots([
                        SlotView::new(
                            NavigationViewItemSlot::Content,
                            TextBlock::new().text(title).font_size(16.0),
                        ),
                        SlotView::new(
                            NavigationViewItemSlot::Icon,
                            SymbolIcon::new().symbol(symbol),
                        ),
                    ]),
            )
        })
        .collect::<Vec<_>>();
        let navigation = NavigationView::new()
            .pane_display_mode(if self.narrow {
                NavigationViewPaneDisplayMode::LeftCompact
            } else {
                NavigationViewPaneDisplayMode::Left
            })
            .open_pane_length(232.0)
            .is_pane_open(self.pane_open)
            .on_is_pane_open_changed(cx.callback(Message::PaneOpen))
            .is_pane_toggle_button_visible(self.narrow)
            .is_settings_visible(false)
            .is_back_button_visible(NavigationViewBackButtonVisible::Collapsed)
            .on_selected_tag_changed(
                cx.callback(|tag: Option<String>| Message::Navigate(tag.unwrap_or_default())),
            )
            .slots([
                SlotView::collection(NavigationViewSlot::MenuItems, nav_items),
                SlotView::new(
                    NavigationViewSlot::PaneFooter,
                    if self.pane_open {
                        StackPanel::new()
                            .margin(Thickness::uniform(18.0))
                            .spacing(8.0)
                            .children((
                                Grid::new()
                                    .columns([GridLength::Auto, GridLength::Star(1.0)])
                                    .column_spacing(10.0)
                                    .children((
                                        Border::new()
                                            .grid_column(0)
                                            .width(32.0)
                                            .height(32.0)
                                            .background(ThemeBrush::Accent)
                                            .corner_radius(CornerRadius::uniform(7.0))
                                            .content(SymbolIcon::new().symbol(Symbol::ViewAll)),
                                        TextBlock::new()
                                            .grid_column(1)
                                            .vertical_alignment(VerticalAlignment::Center)
                                            .text("Widgets RS")
                                            .font_weight(FontWeight::SEMI_BOLD),
                                    )),
                                TextBlock::new()
                                    .text("Small widgets. A more personal Windows.")
                                    .font_size(12.0)
                                    .text_wrapping(TextWrapping::Wrap),
                            ))
                    } else {
                        Border::new()
                            .margin(Thickness::xy(8.0, 16.0))
                            .width(32.0)
                            .height(32.0)
                            .background(ThemeBrush::Accent)
                            .corner_radius(CornerRadius::uniform(7.0))
                            .content(SymbolIcon::new().symbol(Symbol::ViewAll))
                    },
                ),
                SlotView::new(
                    NavigationViewSlot::Content,
                    Grid::new().children((Grid::new()
                        .rows([GridLength::Auto, GridLength::Auto, GridLength::Star(1.0)])
                        .children((
                            InfoBar::new()
                                .grid_row(0)
                                .title(if self.error.is_some() {
                                    "Widget action"
                                } else {
                                    "App appearance"
                                })
                                .message(
                                    self.error
                                        .clone()
                                        .or_else(|| self.appearance_error.clone())
                                        .unwrap_or_default(),
                                )
                                .severity(InfoBarSeverity::Error)
                                .is_open(self.error.is_some() || self.appearance_error.is_some())
                                .on_closed(cx.message(Message::ClearError)),
                            if let Some(notice) = &self.notice {
                                View::from(
                                    TextBlock::new()
                                        .grid_row(1)
                                        .margin(Thickness::uniform(12.0))
                                        .text(notice.clone()),
                                )
                            } else {
                                Border::new()
                                    .grid_row(1)
                                    .height(0.0)
                                    .content(TextBlock::new().text(""))
                            },
                            Border::new().grid_row(2).content(page),
                        )),)),
                ),
            ]);
        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((
                Border::new()
                    .grid_row_span(2)
                    .width(if self.narrow { 48.0 } else { 232.0 })
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .background(rail_surface())
                    .content(View::fragment(())),
                TitleBar::new()
                    .grid_row(0)
                    .height(52.0)
                    .title("")
                    .automation_id("AppTitleBar")
                    .automation_name("Widgets RS title bar")
                    .preferred_height(WindowTitleBarHeight::Tall)
                    .slots([SlotView::new(
                        TitleBarSlot::Content,
                        Grid::new()
                            .columns([GridLength::Auto, GridLength::Auto])
                            .column_spacing(12.0)
                            .children((
                                Border::new()
                                    .width(28.0)
                                    .height(28.0)
                                    .background(ThemeBrush::Accent)
                                    .corner_radius(CornerRadius::uniform(7.0))
                                    .content(SymbolIcon::new().symbol(Symbol::ViewAll)),
                                TextBlock::new()
                                    .grid_column(1)
                                    .vertical_alignment(VerticalAlignment::Center)
                                    .text("Widgets RS")
                                    .font_size(14.0)
                                    .font_weight(FontWeight::SEMI_BOLD),
                            )),
                    )]),
                Border::new().grid_row(1).content(navigation),
            ))
    }
}

impl WidgetApp {
    fn home(&self, cx: &mut ViewContext<Self>) -> View {
        let pinned = self
            .records
            .iter()
            .take(2)
            .enumerate()
            .map(|(index, record)| {
                let id = record.id.clone();
                let face = record
                    .clock()
                    .map(compact_preview)
                    .unwrap_or_else(|| TextBlock::new().text("Preview unavailable").into());
                KeyedView::new(
                    id.clone(),
                    Border::new()
                        .grid_row(if self.narrow { index as i32 } else { 0 })
                        .grid_column(if self.narrow { 0 } else { index as i32 })
                        .padding(Thickness::uniform(18.0))
                        .background(ThemeBrush::CardBackground)
                        .border_brush(ThemeBrush::CardStroke)
                        .border_thickness(Thickness::uniform(1.0))
                        .corner_radius(CornerRadius::uniform(12.0))
                        .content(
                            StackPanel::new().spacing(12.0).children((
                                Grid::new()
                                    .columns([
                                        GridLength::Auto,
                                        GridLength::Star(1.0),
                                        GridLength::Auto,
                                    ])
                                    .column_spacing(10.0)
                                    .children((
                                        Border::new()
                                            .grid_column(0)
                                            .width(34.0)
                                            .height(34.0)
                                            .background(rail_surface())
                                            .corner_radius(CornerRadius::uniform(8.0))
                                            .content(world_icon()),
                                        StackPanel::new().grid_column(1).spacing(2.0).children((
                                            TextBlock::new()
                                                .text(record.display_name())
                                                .font_size(19.0)
                                                .font_weight(FontWeight::SEMI_BOLD)
                                                .text_wrapping(TextWrapping::Wrap),
                                            TextBlock::new()
                                                .text(format!(
                                                    "World Clock  ·  {}",
                                                    size_name(record.size)
                                                ))
                                                .font_size(12.0),
                                        )),
                                        Button::new()
                                            .grid_column(2)
                                            .style(ButtonStyle::Subtle)
                                            .automation_id(format!("HomeInstance_{id}"))
                                            .automation_name(format!(
                                                "Edit {}",
                                                record.display_name()
                                            ))
                                            .on_click(cx.message(Message::Select(id)))
                                            .content(button_icon(Symbol::More)),
                                    )),
                                Border::new()
                                    .height(1.0)
                                    .background(ThemeBrush::CardStroke)
                                    .content(TextBlock::new().text("")),
                                face,
                            )),
                        ),
                )
            })
            .collect::<Vec<_>>();
        let pinned_region: View = if self.records.is_empty() {
            Border::new().padding(Thickness::uniform(24.0)).background(ThemeBrush::CardBackground)
                .border_brush(ThemeBrush::CardStroke).border_thickness(Thickness::uniform(1.0))
                .corner_radius(CornerRadius::uniform(12.0)).content(StackPanel::new().spacing(12.0).children((
                    SymbolIcon::new().symbol(Symbol::Pin),
                    TextBlock::new().text("Nothing pinned yet").font_size(20.0).font_weight(FontWeight::SEMI_BOLD),
                    TextBlock::new().text("Open the Windows widget board, select Add widgets, and choose World Clock.").text_wrapping(TextWrapping::Wrap),
                    Button::new().style(ButtonStyle::Accent).automation_id("HomeOpenBoard")
                        .on_click(cx.message(Message::OpenBoard)).content("Open widget board"),
                )))
        } else {
            StackPanel::new().spacing(12.0).children((
                Grid::new()
                    .columns(if self.narrow {
                        vec![GridLength::Star(1.0)]
                    } else {
                        vec![GridLength::Star(1.0), GridLength::Star(1.0)]
                    })
                    .rows(vec![GridLength::Auto; if self.narrow { 2 } else { 1 }])
                    .column_spacing(12.0)
                    .row_spacing(12.0)
                    .keyed_children(pinned),
                Grid::new()
                    .columns([GridLength::Auto, GridLength::Auto])
                    .column_spacing(10.0)
                    .children((
                        Button::new()
                            .grid_column(0)
                            .automation_id("HomeViewAll")
                            .on_click(cx.message(Message::Navigate("widgets".into())))
                            .content(command_content(
                                Symbol::Setting,
                                if self.records.len() > 2 {
                                    "View all widgets"
                                } else {
                                    "Manage widgets"
                                },
                            )),
                        Button::new()
                            .grid_column(1)
                            .automation_id("HomeOpenBoard")
                            .on_click(cx.message(Message::OpenBoard))
                            .content(command_content(Symbol::Go, "Open widget board")),
                    )),
            ))
        };
        let sample = sample_clock();
        let main = StackPanel::new().spacing(18.0).children((
            TextBlock::new()
                .text("Your widget space")
                .font_size(40.0)
                .font_weight(FontWeight::SEMI_BOLD),
            TextBlock::new()
                .text("Pin your favorites, discover new widgets, and make Windows yours.")
                .font_size(16.0)
                .text_wrapping(TextWrapping::Wrap),
            TextBlock::new()
                .text(format!("Pinned widgets  ·  {}", self.records.len()))
                .font_size(21.0)
                .font_weight(FontWeight::SEMI_BOLD),
            pinned_region,
            TextBlock::new()
                .text("Explore widgets")
                .font_size(21.0)
                .font_weight(FontWeight::SEMI_BOLD),
            Border::new()
                .padding(Thickness::uniform(20.0))
                .background(feature_surface())
                .border_brush(ThemeBrush::Accent)
                .border_thickness(Thickness::uniform(1.0))
                .corner_radius(CornerRadius::uniform(12.0))
                .content(
                    Grid::new().children((
                        Image::new()
                            .source_data(featured_artwork())
                            .stretch(Stretch::UniformToFill),
                        Grid::new()
                            .columns(if self.narrow {
                                vec![GridLength::Star(1.0)]
                            } else {
                                vec![GridLength::Star(1.0), GridLength::Star(1.0)]
                            })
                            .rows(vec![GridLength::Auto; if self.narrow { 2 } else { 1 }])
                            .column_spacing(16.0)
                            .row_spacing(16.0)
                            .children((
                                StackPanel::new().grid_column(0).spacing(12.0).children((
                                    Grid::new()
                                        .columns([GridLength::Auto, GridLength::Star(1.0)])
                                        .column_spacing(12.0)
                                        .children((
                                            Border::new()
                                                .grid_column(0)
                                                .width(38.0)
                                                .height(38.0)
                                                .background(ThemeBrush::CardBackground)
                                                .corner_radius(CornerRadius::uniform(9.0))
                                                .content(world_icon()),
                                            TextBlock::new()
                                                .grid_column(1)
                                                .vertical_alignment(VerticalAlignment::Center)
                                                .text("World Clock")
                                                .font_size(25.0)
                                                .font_weight(FontWeight::SEMI_BOLD),
                                        )),
                                    TextBlock::new()
                                        .text(DEFINITIONS[0].description)
                                        .text_wrapping(TextWrapping::Wrap),
                                    available_badge(),
                                    Button::new()
                                        .style(ButtonStyle::Accent)
                                        .automation_id("HomeDiscoverClock")
                                        .on_click(cx.message(Message::Explore("Clock".into())))
                                        .content("Explore World Clock"),
                                )),
                                Border::new()
                                    .grid_row(if self.narrow { 1 } else { 0 })
                                    .grid_column(if self.narrow { 0 } else { 1 })
                                    .padding(Thickness::uniform(16.0))
                                    .background(ThemeBrush::CardBackground)
                                    .border_brush(ThemeBrush::CardStroke)
                                    .border_thickness(Thickness::uniform(1.0))
                                    .corner_radius(CornerRadius::uniform(10.0))
                                    .content(featured_preview(&sample)),
                            )),
                    )),
                ),
        ));
        let rail = self.home_guidance(cx);
        let content: View = if self.wide_home {
            Grid::new()
                .margin(Thickness::uniform(28.0))
                .columns([GridLength::Star(1.0), GridLength::Pixel(300.0)])
                .column_spacing(22.0)
                .children((main, rail))
        } else {
            StackPanel::new()
                .margin(Thickness::uniform(28.0))
                .children((main,))
        };
        ScrollViewer::new().content(content)
    }
    fn home_guidance(&self, cx: &ViewContext<Self>) -> View {
        Border::new().grid_column(1).padding(Thickness::uniform(22.0))
            .background(rail_surface()).border_brush(ThemeBrush::CardStroke)
            .border_thickness(Thickness::uniform(1.0)).corner_radius(CornerRadius::uniform(12.0))
            .content(Grid::new().rows([GridLength::Auto,GridLength::Star(1.0),GridLength::Auto])
                .children((
                    StackPanel::new().spacing(24.0).children((
                        StackPanel::new().spacing(14.0).children((
                            SymbolIcon::new().symbol(Symbol::ViewAll).horizontal_alignment(HorizontalAlignment::Left),
                            TextBlock::new().text("Your Windows board").font_size(26.0).font_weight(FontWeight::SEMI_BOLD)
                                .text_wrapping(TextWrapping::Wrap),
                            TextBlock::new().text("Keep the things you check often one glance away.")
                                .font_size(15.0).opacity(quiet_opacity()).text_wrapping(TextWrapping::Wrap),
                        )),
                        guidance_step("Pin a widget", "Open the board, choose Add widgets, and select a widget from Widgets RS."),
                        guidance_step("Make it yours", "Choose an instance in My widgets to edit its content and give it a name."),
                        guidance_step("Arrange your space", "Resize and move your widgets on the Windows board."),
                        Button::new().style(ButtonStyle::Accent).automation_id("HomeRailOpenBoard")
                            .horizontal_alignment(HorizontalAlignment::Stretch)
                            .on_click(cx.message(Message::OpenBoard)).content(command_content(Symbol::Go,"Open widget board")),
                    )),
                    StackPanel::new().grid_row(2).spacing(12.0).children((
                        Border::new().height(1.0).background(ThemeBrush::CardStroke).content(TextBlock::new().text("")),
                        TextBlock::new().text(format!("{} WIDGET TYPE  ·  {} PINNED",DEFINITIONS.len(),self.records.len()))
                            .font_size(11.0).font_weight(FontWeight::SEMI_BOLD).foreground(ThemeBrush::Accent),
                        TextBlock::new().text("A growing collection of useful widgets. New types arrive with app updates.")
                            .font_size(12.0).opacity(quiet_opacity()).text_wrapping(TextWrapping::Wrap),
                    )),
                )))
    }

    fn discover(&self, cx: &mut ViewContext<Self>) -> View {
        let results = search_definitions(&self.discover_query);
        let mut sample = sample_clock();
        for id in ["Australia/Sydney", "Asia/Kolkata", "Pacific/Auckland"] {
            sample.add(id.into());
        }
        let show_details = results
            .iter()
            .any(|item| self.selected_definition.as_deref() == Some(item.id));
        let cards = results
            .iter()
            .map(|item| {
                KeyedView::new(
                    item.id,
                    Border::new()
                        .width(290.0)
                        .horizontal_alignment(HorizontalAlignment::Left)
                        .padding(Thickness::uniform(12.0))
                        .background(ThemeBrush::CardBackground)
                        .border_brush(if self.selected_definition.as_deref() == Some(item.id) {
                            ThemeBrush::Accent
                        } else {
                            ThemeBrush::CardStroke
                        })
                        .border_thickness(Thickness::uniform(1.0))
                        .corner_radius(CornerRadius::uniform(12.0))
                        .content(
                            Button::new()
                                .style(ButtonStyle::Subtle)
                                .automation_id(format!("Catalog_{}", item.id))
                                .automation_name(format!("View {} details", item.title))
                                .horizontal_alignment(HorizontalAlignment::Stretch)
                                .horizontal_content_alignment(HorizontalAlignment::Stretch)
                                .on_click(cx.message(Message::SelectDefinition(item.id.into())))
                                .content(
                                    StackPanel::new().spacing(10.0).children((
                                        SymbolIcon::new()
                                            .horizontal_alignment(HorizontalAlignment::Left)
                                            .symbol(Symbol::World),
                                        Border::new()
                                            .padding(Thickness::uniform(12.0))
                                            .background(ThemeBrush::SolidBackground)
                                            .corner_radius(CornerRadius::uniform(8.0))
                                            .content(compact_preview(&sample)),
                                        TextBlock::new()
                                            .text(item.title)
                                            .font_size(19.0)
                                            .font_weight(FontWeight::SEMI_BOLD),
                                        TextBlock::new()
                                            .text(item.description)
                                            .text_wrapping(TextWrapping::Wrap),
                                        available_badge(),
                                    )),
                                ),
                        ),
                )
            })
            .collect::<Vec<_>>();
        let size_previews: View = if show_details && self.wide_home {
            StackPanel::new().spacing(12.0).children((
                TextBlock::new().text("Preview every size").font_size(21.0).font_weight(FontWeight::SEMI_BOLD),
                TextBlock::new().text("The board chooses the layout for each size. Your selected cities stay in the same order." )
                    .text_wrapping(TextWrapping::Wrap),
                Grid::new()
                    .columns([GridLength::Pixel(300.0), GridLength::Pixel(300.0)])
                    .column_spacing(20.0)
                    .children((
                        StackPanel::new().grid_column(0).spacing(8.0).children((
                            TextBlock::new().text("Small  ·  2 zones").font_weight(FontWeight::SEMI_BOLD),
                            preview(&sample, WidgetSize::Small),
                        )),
                        StackPanel::new().grid_column(1).spacing(8.0).children((
                            TextBlock::new().text("Medium  ·  4 zones").font_weight(FontWeight::SEMI_BOLD),
                            preview(&sample, WidgetSize::Medium),
                        )),
                    )),
                TextBlock::new().text("Large  ·  6 zones").font_weight(FontWeight::SEMI_BOLD),
                Border::new().width(300.0)
                    .horizontal_alignment(HorizontalAlignment::Left).content(preview(&sample, WidgetSize::Large)),
            ))
        } else {
            View::fragment(())
        };
        let main = StackPanel::new().spacing(18.0).children((
            TextBlock::new().text("Discover widgets").font_size(36.0).font_weight(FontWeight::SEMI_BOLD),
            TextBlock::new().text("A home for useful widgets. Available types arrive with app updates.").font_size(16.0).text_wrapping(TextWrapping::Wrap),
            TextBox::new().automation_id("CatalogSearch").automation_name("Search widget catalog")
                .text(self.discover_query.clone()).placeholder_text("Search widgets")
                .on_text_changed(cx.callback(Message::DiscoverSearch)),
            if results.is_empty() { View::from(TextBlock::new().text("No released widgets match your search.")) }
            else { Border::new().padding(Thickness::uniform(24.0)).background(feature_surface())
                .border_brush(ThemeBrush::CardStroke).border_thickness(Thickness::uniform(1.0))
                .corner_radius(CornerRadius::uniform(12.0)).content(
                    Grid::new().children((
                    Image::new().source_data(featured_artwork()).stretch(Stretch::UniformToFill),
                    Grid::new().columns(if self.narrow { vec![GridLength::Star(1.0)] } else { vec![GridLength::Star(1.0), GridLength::Star(1.0)] })
                        .rows(vec![GridLength::Auto; if self.narrow { 2 } else { 1 }])
                        .column_spacing(22.0).row_spacing(16.0).children((
                        StackPanel::new().grid_column(0).spacing(12.0).children((
                            TextBlock::new().text("FEATURED").font_size(12.0).foreground(ThemeBrush::Accent),
                            TextBlock::new().text("World Clock").font_size(28.0).font_weight(FontWeight::SEMI_BOLD),
                            TextBlock::new().text("Keep track of time across the places that matter to you.").text_wrapping(TextWrapping::Wrap),
                            Button::new().style(ButtonStyle::Accent).automation_id("FeaturedClock")
                                .on_click(cx.message(if show_details { Message::OpenBoard } else { Message::SelectDefinition("Clock".into()) }))
                                .content(if show_details { "Open widget board" } else { "View details" }),
                        )),
                        Border::new().grid_row(if self.narrow { 1 } else { 0 })
                            .grid_column(if self.narrow { 0 } else { 1 }).padding(Thickness::uniform(16.0))
                            .background(ThemeBrush::CardBackground).border_brush(ThemeBrush::CardStroke)
                            .border_thickness(Thickness::uniform(1.0)).corner_radius(CornerRadius::uniform(10.0))
                            .content(featured_preview(&sample)),
                    ))))) },
            if results.len() > 1 {
                View::from(TextBlock::new().text("All widgets").font_size(21.0).font_weight(FontWeight::SEMI_BOLD))
            } else {
                View::fragment(())
            },
            if results.len() > 1 {
                StackPanel::new().spacing(12.0).keyed_children(cards)
            } else {
                View::fragment(())
            },
            size_previews,
        ));
        let detail: View = if show_details {
            Border::new().grid_column(1).padding(Thickness::uniform(18.0))
                .background(rail_surface()).border_brush(ThemeBrush::CardStroke)
                .border_thickness(Thickness::uniform(1.0)).corner_radius(CornerRadius::uniform(12.0))
                .content(StackPanel::new().spacing(14.0).children((
                    Grid::new().columns([GridLength::Auto, GridLength::Star(1.0)]).column_spacing(10.0).children((
                        SymbolIcon::new().grid_column(0).symbol(Symbol::World),
                        TextBlock::new().grid_column(1).text("World Clock").font_size(24.0).font_weight(FontWeight::SEMI_BOLD),
                    )),
                    available_badge(),
                    TextBlock::new().text("Keep time in multiple cities on one board card. Each pin has its own cities and labels.").text_wrapping(TextWrapping::Wrap),
                    TextBlock::new().text("Live preview").font_size(17.0).font_weight(FontWeight::SEMI_BOLD),
                    preview(&sample, WidgetSize::Small),
                    TextBlock::new().text("Supported sizes").font_size(17.0).font_weight(FontWeight::SEMI_BOLD),
                    Grid::new().columns([GridLength::Star(1.0), GridLength::Star(1.0), GridLength::Star(1.0)])
                        .column_spacing(6.0).children((
                            size_tile(0, "Small", "2 zones", 18.0, 18.0),
                            size_tile(1, "Medium", "4 zones", 18.0, 28.0),
                            size_tile(2, "Large", "6 zones", 18.0, 38.0),
                        )),
                    TextBlock::new().text("Windows owns pinning. Open the board, choose Add widgets, then select World Clock.").text_wrapping(TextWrapping::Wrap),
                    Button::new().style(ButtonStyle::Accent).automation_id("DiscoverOpenBoard")
                        .on_click(cx.message(Message::OpenBoard)).content("Open widget board"),
                    if self.wide_home { View::fragment(()) }
                    else { StackPanel::new().spacing(12.0).children((
                        TextBlock::new().text("Medium preview").font_weight(FontWeight::SEMI_BOLD),
                        preview(&sample, WidgetSize::Medium),
                        TextBlock::new().text("Large preview").font_weight(FontWeight::SEMI_BOLD),
                        preview(&sample, WidgetSize::Large),
                    )) },
                )))
        } else {
            TextBlock::new().grid_column(1).text("").into()
        };
        let content: View = if self.wide_home {
            Grid::new()
                .margin(Thickness::uniform(28.0))
                .columns([GridLength::Star(1.0), GridLength::Pixel(340.0)])
                .column_spacing(22.0)
                .children((
                    ScrollViewer::new().grid_column(0).content(main),
                    ScrollViewer::new().grid_column(1).content(detail),
                ))
        } else {
            ScrollViewer::new().content(
                StackPanel::new()
                    .margin(Thickness::uniform(28.0))
                    .spacing(20.0)
                    .children((main, detail)),
            )
        };
        content
    }
    fn settings(&self, cx: &mut ViewContext<Self>) -> View {
        let appearance = Border::new().grid_column(0).padding(Thickness::uniform(24.0))
            .background(ThemeBrush::CardBackground).border_brush(ThemeBrush::CardStroke)
            .border_thickness(Thickness::uniform(1.0)).corner_radius(CornerRadius::uniform(12.0))
            .content(StackPanel::new().spacing(14.0).children((
                SymbolIcon::new().horizontal_alignment(HorizontalAlignment::Left).symbol(Symbol::Setting),
                TextBlock::new().text("Appearance").font_size(22.0).font_weight(FontWeight::SEMI_BOLD),
                TextBlock::new().text("Follows Windows").font_size(17.0).font_weight(FontWeight::SEMI_BOLD),
                TextBlock::new().text("Widgets RS uses your system Light, Dark, and Contrast themes. The backdrop and cards adapt with Windows.")
                    .text_wrapping(TextWrapping::Wrap),
            )));
        let runtime = Border::new()
            .grid_column(1)
            .padding(Thickness::uniform(24.0))
            .background(ThemeBrush::CardBackground)
            .border_brush(ThemeBrush::CardStroke)
            .border_thickness(Thickness::uniform(1.0))
            .corner_radius(CornerRadius::uniform(12.0))
            .content(
                StackPanel::new().spacing(14.0).children((
                    SymbolIcon::new()
                        .horizontal_alignment(HorizontalAlignment::Left)
                        .symbol(Symbol::ViewAll),
                    TextBlock::new()
                        .text("App and runtime")
                        .font_size(22.0)
                        .font_weight(FontWeight::SEMI_BOLD),
                    TextBlock::new().text("Version 0.2.0.0  ·  Development x64"),
                    TextBlock::new()
                        .text("Shared Windows App Runtime · WinUI 3 active")
                        .text_wrapping(TextWrapping::Wrap),
                    TextBlock::new()
                        .text(format!("Package identity: {}", package_identity_status()))
                        .font_size(12.0)
                        .text_wrapping(TextWrapping::Wrap),
                )),
            );
        let top: View = if self.wide_home {
            Grid::new()
                .columns([GridLength::Star(1.0), GridLength::Star(1.0)])
                .column_spacing(18.0)
                .children((appearance, runtime))
        } else {
            StackPanel::new()
                .spacing(18.0)
                .children((appearance, runtime))
        };
        ScrollViewer::new().content(StackPanel::new().spacing(18.0).margin(Thickness::uniform(28.0)).children((
            TextBlock::new().text("Settings").font_size(36.0).font_weight(FontWeight::SEMI_BOLD),
            TextBlock::new().text("A quiet home for every widget on your board.").font_size(16.0),
            top,
            Border::new().padding(Thickness::uniform(24.0)).background(ThemeBrush::CardBackground)
                .border_brush(ThemeBrush::CardStroke).border_thickness(Thickness::uniform(1.0))
                .corner_radius(CornerRadius::uniform(12.0)).content(StackPanel::new().spacing(14.0).children((
                    TextBlock::new().text("Offline and private").font_size(22.0).font_weight(FontWeight::SEMI_BOLD),
                    TextBlock::new().text("City search is bundled with the app. Your clock selections and instance names live in the Windows widget host's per-widget custom state. New widget types arrive through app updates.")
                        .text_wrapping(TextWrapping::Wrap),
                    TextBlock::new().text(format!("{} pinned widget instances", self.records.len()))
                        .font_weight(FontWeight::SEMI_BOLD),
                    Button::new().automation_id("SettingsRefresh").on_click(cx.message(Message::Reload))
                        .content("Refresh widget status"),
                ))),
        )))
    }
}

fn type_name(record: &WidgetRecord) -> &'static str {
    definition(&record.definition_id)
        .map(|value| value.title)
        .unwrap_or("Widget")
}

fn command_content(symbol: Symbol, label: &str) -> View {
    Grid::new()
        .columns([GridLength::Auto, GridLength::Auto])
        .column_spacing(8.0)
        .margin(Thickness::xy(3.0, 5.0))
        .children((
            button_icon(symbol),
            TextBlock::new()
                .grid_column(1)
                .text(label)
                .vertical_alignment(VerticalAlignment::Center),
        ))
}

/// SymbolIcon keeps its native glyph size when Width/Height change. Scale its
/// unconstrained glyph in a native Viewbox instead of clipping it into a slot.
/// 16 DIP is a Fluent icon size; the Button retains its larger hit target and
/// supplies foreground colors for normal, accent, disabled, and Contrast states.
fn button_icon(symbol: Symbol) -> View {
    Viewbox::new()
        .width(16.0)
        .height(16.0)
        .stretch(Stretch::Uniform)
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .slots([SlotView::new(
            ViewboxSlot::Child,
            SymbolIcon::new().symbol(symbol),
        )])
}

fn quiet_opacity() -> f64 {
    if clock_core::high_contrast_enabled() {
        1.0
    } else {
        0.74
    }
}
fn available_badge() -> View {
    Border::new()
        .horizontal_alignment(HorizontalAlignment::Left)
        .padding(Thickness::xy(10.0, 5.0))
        .background(ThemeBrush::SolidBackground)
        .border_brush(ThemeBrush::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(12.0))
        .content(
            TextBlock::new()
                .text("●  AVAILABLE")
                .font_size(11.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .foreground(ThemeBrush::Accent),
        )
}

fn guidance_step(title: &str, description: &str) -> View {
    StackPanel::new().spacing(8.0).children((
        TextBlock::new()
            .text(title)
            .font_size(18.0)
            .font_weight(FontWeight::SEMI_BOLD),
        TextBlock::new()
            .text(description)
            .font_size(14.0)
            .opacity(quiet_opacity())
            .text_wrapping(TextWrapping::Wrap),
    ))
}
fn feature_surface() -> Brush {
    if clock_core::high_contrast_enabled() {
        ThemeBrush::SolidBackground.into()
    } else {
        Color::argb(34, 32, 118, 196).into()
    }
}
fn rail_surface() -> Brush {
    if clock_core::high_contrast_enabled() {
        ThemeBrush::SolidBackground.into()
    } else {
        Color::argb(18, 32, 118, 196).into()
    }
}
fn size_tile(
    column: i32,
    label: &'static str,
    dimensions: &'static str,
    width: f64,
    height: f64,
) -> View {
    Border::new()
        .grid_column(column)
        .padding(Thickness::xy(7.0, 10.0))
        .background(ThemeBrush::SolidBackground)
        .border_brush(ThemeBrush::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(8.0))
        .content(
            StackPanel::new().spacing(6.0).children((
                Border::new()
                    .width(width)
                    .height(height)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .border_brush(ThemeBrush::Accent)
                    .border_thickness(Thickness::uniform(1.0))
                    .corner_radius(CornerRadius::uniform(3.0))
                    .content(TextBlock::new().text("")),
                TextBlock::new()
                    .text(label)
                    .font_size(12.0)
                    .horizontal_alignment(HorizontalAlignment::Center),
                TextBlock::new()
                    .text(dimensions)
                    .font_size(11.0)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )),
        )
}
fn sample_clock() -> ClockConfig {
    let mut sample = ClockConfig::new_default("America/Los_Angeles");
    sample.zones.clear();
    for id in ["Europe/London", "America/New_York", "Asia/Tokyo"] {
        sample.add(id.into());
    }
    sample
}

fn city_choice_label(zone: &clock_core::ZoneChoice) -> String {
    format!("{} · {}", zone.city, zone.id)
}

fn city_choice_id(label: &str) -> Option<String> {
    let (_, id) = label.rsplit_once(" · ")?;
    search_zones(id, 8)
        .into_iter()
        .find(|zone| zone.id == id && city_choice_label(zone) == label)
        .map(|zone| zone.id)
}

#[derive(Default)]
struct CitySearchState {
    text: String,
    filter: String,
    chosen_id: Option<String>,
}

impl CitySearchState {
    fn text_changed(&mut self, value: String) {
        // Reactor exposes TextChanged without WinUI's reason. Suggestion
        // navigation changes text too; retain the filter in either event order.
        if value != self.text {
            self.chosen_id = None;
            if city_choice_id(&value).is_none() {
                self.filter = value.clone();
            }
            self.text = value;
        }
    }

    fn suggestion_chosen(&mut self, value: String) {
        self.chosen_id = city_choice_id(&value);
        self.text = value;
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}
fn size_name(size: WidgetSize) -> &'static str {
    if size == WidgetSize::Small {
        "Small"
    } else if size == WidgetSize::Medium {
        "Medium"
    } else {
        "Large"
    }
}

pub fn route_from_args(args: impl IntoIterator<Item = String>) -> Option<String> {
    let arg = args
        .into_iter()
        .find(|arg| arg.starts_with("widgetsrs://edit/"))?;
    let encoded = arg
        .trim_start_matches("widgetsrs://edit/")
        .split(['?', '#'])
        .next()?;
    let mut bytes = Vec::new();
    let raw = encoded.as_bytes();
    let mut index = 0;
    while index < raw.len() {
        if raw[index] == b'%' {
            if index + 2 >= raw.len() {
                return None;
            }
            let hi = (raw[index + 1] as char).to_digit(16)?;
            let lo = (raw[index + 2] as char).to_digit(16)?;
            bytes.push(((hi << 4) | lo) as u8);
            index += 3;
        } else {
            bytes.push(raw[index]);
            index += 1;
        }
    }
    let id = String::from_utf8(bytes).ok()?;
    (!id.is_empty() && id.len() <= 256).then_some(id)
}

#[link(name = "shell32")]
unsafe extern "system" {
    fn ShellExecuteW(
        hwnd: *mut c_void,
        operation: *const u16,
        file: *const u16,
        parameters: *const u16,
        directory: *const u16,
        show: i32,
    ) -> *mut c_void;
}
fn open_board() -> bool {
    let uri: Vec<u16> = "ms-widgetboard:".encode_utf16().chain(Some(0)).collect();
    (unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            ptr::null(),
            uri.as_ptr(),
            ptr::null(),
            ptr::null(),
            1,
        )
    } as usize)
        > 32
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentPackageFullName(length: *mut u32, name: *mut u16) -> i32;
}
fn package_identity_status() -> String {
    let mut length = 0u32;
    unsafe { GetCurrentPackageFullName(&mut length, ptr::null_mut()) };
    if length == 0 || length > 1024 {
        return "Unpackaged development run".into();
    }
    let mut buffer = vec![0u16; length as usize];
    if unsafe { GetCurrentPackageFullName(&mut length, buffer.as_mut_ptr()) } != 0 {
        return "Unavailable".into();
    }
    String::from_utf16_lossy(&buffer[..length.saturating_sub(1) as usize])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routes_decode_and_reject_bad_input() {
        assert_eq!(
            route_from_args(["app".into(), "widgetsrs://edit/a%2Fb?x=1".into()]),
            Some("a/b".into())
        );
        assert_eq!(route_from_args(["widgetsrs://edit/%XX".into()]), None);
        assert_eq!(route_from_args(["widgetsrs://edit/".into()]), None);
    }

    #[test]
    fn city_suggestions_resolve_only_bundled_ids() {
        let zone = search_zones("Tokyo", 8)
            .into_iter()
            .find(|zone| zone.id == "Asia/Tokyo")
            .unwrap();
        assert_eq!(city_choice_id(&city_choice_label(&zone)), Some(zone.id));
        assert_eq!(city_choice_id("Tokyo"), None);
        assert_eq!(city_choice_id("Tokyo · Invalid/Zone"), None);
    }

    #[test]
    fn suggestion_navigation_retains_filter_in_both_event_orders() {
        let label = "Tokyo · Asia/Tokyo".to_owned();
        for text_first in [true, false] {
            let mut search = CitySearchState::default();
            search.text_changed("Tok".into());
            if text_first {
                search.text_changed(label.clone());
                search.suggestion_chosen(label.clone());
            } else {
                search.suggestion_chosen(label.clone());
                search.text_changed(label.clone());
            }
            assert_eq!(search.filter, "Tok");
            assert_eq!(search.chosen_id.as_deref(), Some("Asia/Tokyo"));
            search.text_changed("Lon".into());
            assert_eq!(search.filter, "Lon");
            assert!(search.chosen_id.is_none());
            search.clear();
            assert!(search.text.is_empty() && search.filter.is_empty());
        }
    }
}
