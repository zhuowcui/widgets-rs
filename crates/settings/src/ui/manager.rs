use super::*;

impl WidgetApp {
    pub(super) fn my_widgets(&self, cx: &mut ViewContext<Self>) -> View {
        let instances = self
            .records
            .iter()
            .map(|record| {
                let id = record.id.clone();
                let selected = self.selected.as_ref() == Some(&id);
                let updated =
                    widget_api::Globalization::DateTimeFormatting::DateTimeFormatter::ShortTime()
                        .and_then(|formatter| formatter.Format(record.last_update))
                        .map(|value| value.to_string_lossy())
                        .unwrap_or_else(|_| "Unknown".into());
                KeyedView::new(
                    id.clone(),
                    Border::new()
                        .background(if selected {
                            feature_surface()
                        } else {
                            ThemeBrush::CardBackground.into()
                        })
                        .border_brush(ThemeBrush::CardStroke)
                        .border_thickness(Thickness::uniform(1.0))
                        .corner_radius(CornerRadius::uniform(10.0))
                        .content(
                            Button::new()
                                .style(ButtonStyle::Subtle)
                                .automation_id(format!("Instance_{id}"))
                                .automation_name(format!(
                                    "{}{}; {}; {}; updated {}",
                                    if selected { "Selected instance; " } else { "" },
                                    record.display_name(),
                                    type_name(record),
                                    size_name(record.size),
                                    updated
                                ))
                                .horizontal_alignment(HorizontalAlignment::Stretch)
                                .horizontal_content_alignment(HorizontalAlignment::Stretch)
                                .on_click(cx.message(Message::Select(id)))
                                .content(
                                    Grid::new()
                                        .columns([
                                            GridLength::Pixel(3.0),
                                            GridLength::Auto,
                                            GridLength::Star(1.0),
                                        ])
                                        .column_spacing(8.0)
                                        .children((
                                            Border::new()
                                                .grid_column(0)
                                                .height(44.0)
                                                .vertical_alignment(VerticalAlignment::Center)
                                                .corner_radius(CornerRadius::uniform(2.0))
                                                .background(if selected {
                                                    ThemeBrush::Accent.into()
                                                } else {
                                                    Brush::from(Color::argb(0, 0, 0, 0))
                                                })
                                                .content(View::fragment(())),
                                            world_icon()
                                                .grid_column(1)
                                                .vertical_alignment(VerticalAlignment::Center),
                                            StackPanel::new()
                                                .grid_column(2)
                                                .spacing(4.0)
                                                .children((
                                                TextBlock::new()
                                                    .text(record.display_name())
                                                    .font_size(16.0)
                                                    .font_weight(FontWeight::SEMI_BOLD)
                                                    .text_wrapping(TextWrapping::Wrap),
                                                Grid::new()
                                                    .columns([
                                                        GridLength::Star(1.0),
                                                        GridLength::Auto,
                                                    ])
                                                    .children((
                                                        TextBlock::new()
                                                            .grid_column(0)
                                                            .text(type_name(record))
                                                            .font_size(12.0)
                                                            .vertical_alignment(
                                                                VerticalAlignment::Center,
                                                            ),
                                                        size_badge(record.size, 1),
                                                    )),
                                                TextBlock::new()
                                                    .text(
                                                        if selected
                                                            && clock_core::high_contrast_enabled()
                                                        {
                                                            format!("Selected · Updated {updated}")
                                                        } else {
                                                            format!("Updated {updated}")
                                                        },
                                                    )
                                                    .opacity(quiet_opacity())
                                                    .font_size(12.0),
                                            )),
                                        )),
                                ),
                        ),
                )
            })
            .collect::<Vec<_>>();
        let list_content: View = if self.records.is_empty() {
            StackPanel::new()
                .spacing(12.0)
                .margin(Thickness::uniform(16.0))
                .children((
                    SymbolIcon::new()
                        .symbol(Symbol::Pin)
                        .horizontal_alignment(HorizontalAlignment::Left),
                    TextBlock::new()
                        .text("Your board starts here")
                        .font_size(20.0)
                        .font_weight(FontWeight::SEMI_BOLD),
                    TextBlock::new()
                        .text("Open the widget board and choose World Clock from its picker.")
                        .text_wrapping(TextWrapping::Wrap),
                    Button::new()
                        .automation_id("EmptyManageDiscover")
                        .on_click(cx.message(Message::Navigate("discover".into())))
                        .content("Explore widgets"),
                ))
        } else {
            ListView::new()
                .automation_id("PinnedInstanceList")
                .selection_mode(ListViewSelectionMode::None)
                .collection_slot(ListViewSlot::Items, instances)
        };
        let list = surface().padding(Thickness::uniform(12.0)).content(
            Grid::new()
                .rows([GridLength::Auto, GridLength::Star(1.0), GridLength::Auto])
                .row_spacing(12.0)
                .children((
                    TextBlock::new()
                        .grid_row(0)
                        .text("Instances")
                        .font_size(16.0)
                        .font_weight(FontWeight::SEMI_BOLD)
                        .margin(Thickness::xy(6.0, 4.0)),
                    Border::new().grid_row(1).content(list_content),
                    Button::new()
                        .grid_row(2)
                        .style(ButtonStyle::Subtle)
                        .automation_id("ManageDiscover")
                        .horizontal_alignment(HorizontalAlignment::Stretch)
                        .on_click(cx.message(Message::Navigate("discover".into())))
                        .content(command_content(Symbol::Find, "Discover widgets")),
                )),
        );
        let header: View = if self.narrow && self.detail_open && self.selected().is_some() {
            Grid::new()
                .columns([GridLength::Star(1.0), GridLength::Auto])
                .column_spacing(20.0)
                .children((
                    Button::new()
                        .style(ButtonStyle::Subtle)
                        .automation_id("BackToInstances")
                        .horizontal_alignment(HorizontalAlignment::Left)
                        .on_click(cx.message(Message::Back))
                        .content(command_content(Symbol::Back, "All widgets")),
                    Border::new()
                        .grid_column(1)
                        .content(self.management_commands(cx)),
                ))
        } else if self.narrow {
            StackPanel::new().spacing(16.0).children((
                page_heading(
                    "My widgets",
                    &format!("{} pinned widgets", self.records.len()),
                ),
                self.management_commands(cx),
            ))
        } else {
            Grid::new()
                .columns([GridLength::Star(1.0), GridLength::Auto])
                .column_spacing(20.0)
                .children((
                    page_heading(
                        "My widgets",
                        &format!("{} pinned widgets", self.records.len()),
                    ),
                    Border::new()
                        .grid_column(1)
                        .vertical_alignment(VerticalAlignment::Center)
                        .content(self.management_commands(cx)),
                ))
        };
        let workspace: View = if self.narrow {
            if self.detail_open && self.selected().is_some() {
                self.detail(cx)
            } else {
                list
            }
        } else {
            Grid::new()
                .columns([GridLength::Pixel(310.0), GridLength::Star(1.0)])
                .column_spacing(20.0)
                .children((list, surface().grid_column(1).content(self.detail(cx))))
        };
        Grid::new()
            .margin(Thickness::uniform(24.0))
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .row_spacing(20.0)
            .children((header, Border::new().grid_row(1).content(workspace)))
    }

    fn management_commands(&self, cx: &ViewContext<Self>) -> View {
        Grid::new()
            .columns([GridLength::Auto, GridLength::Auto])
            .column_spacing(10.0)
            .children((
                Button::new()
                    .grid_column(0)
                    .automation_id("RefreshList")
                    .on_click(cx.message(Message::Reload))
                    .content(command_content(Symbol::Refresh, "Refresh")),
                Button::new()
                    .grid_column(1)
                    .automation_id("ManageOpenBoard")
                    .on_click(cx.message(Message::OpenBoard))
                    .content(command_content(Symbol::Go, "Open widget board")),
            ))
    }

    fn detail(&self, cx: &mut ViewContext<Self>) -> View {
        let Some(record) = self.selected() else {
            return StackPanel::new()
                .spacing(12.0)
                .margin(Thickness::uniform(28.0))
                .children((
                    SymbolIcon::new()
                        .symbol(Symbol::ViewAll)
                        .horizontal_alignment(HorizontalAlignment::Left),
                    TextBlock::new()
                        .text("Make your board yours")
                        .font_size(24.0)
                        .font_weight(FontWeight::SEMI_BOLD),
                    TextBlock::new()
                        .text("Choose a pinned widget to preview and personalize it.")
                        .text_wrapping(TextWrapping::Wrap),
                ));
        };
        match &record.state {
            WidgetState::Clock(config) => self.clock_detail(cx, record, config),
        }
    }

    fn clock_detail(
        &self,
        cx: &mut ViewContext<Self>,
        record: &WidgetRecord,
        config: &ClockConfig,
    ) -> View {
        let snapshot = capture(config, windows_time::DateTime::now()).ok();
        let zones = config
            .zones
            .iter()
            .enumerate()
            .map(|(index, zone)| {
                let row = snapshot
                    .as_ref()
                    .and_then(|snapshot| snapshot.rows.get(index));
                let time = row
                    .map(|row| row.time.clone())
                    .unwrap_or_else(|| "—".into());
                let cue = row
                    .map(|row| format!("{}  ·  {}", row.offset, row.day_cue))
                    .unwrap_or_else(|| zone.id.clone());
                KeyedView::new(
                    zone.id.clone(),
                    StackPanel::new().children((
                        if index == 0 { spacer(0.0) } else { divider() },
                        Button::new()
                            .style(ButtonStyle::Subtle)
                            .automation_id(format!("City_{index}"))
                            .automation_name(format!("Edit {}", zone.display_label()))
                            .horizontal_alignment(HorizontalAlignment::Stretch)
                            .horizontal_content_alignment(HorizontalAlignment::Stretch)
                            .min_height(80.0)
                            .on_click(cx.message(Message::SelectCity(index)))
                            .content(
                                Grid::new()
                                    .columns([
                                        GridLength::Star(1.0),
                                        GridLength::Auto,
                                        GridLength::Auto,
                                    ])
                                    .column_spacing(12.0)
                                    .children((
                                        StackPanel::new().grid_column(0).spacing(6.0).children((
                                            TextBlock::new()
                                                .text(zone.display_label())
                                                .font_size(17.0)
                                                .font_weight(FontWeight::SEMI_BOLD)
                                                .text_wrapping(TextWrapping::Wrap),
                                            TextBlock::new()
                                                .text(cue)
                                                .font_size(12.0)
                                                .opacity(quiet_opacity()),
                                        )),
                                        TextBlock::new()
                                            .grid_column(1)
                                            .text(time)
                                            .font_size(if self.wide_home { 28.0 } else { 24.0 })
                                            .font_weight(FontWeight::SEMI_BOLD)
                                            .vertical_alignment(VerticalAlignment::Center),
                                        Border::new()
                                            .grid_column(2)
                                            .vertical_alignment(VerticalAlignment::Center)
                                            .content(button_icon(Symbol::More)),
                                    )),
                            ),
                    )),
                )
            })
            .collect::<Vec<_>>();
        let matches = if self.city_search.filter.trim().is_empty() {
            Vec::new()
        } else {
            search_zones(&self.city_search.filter, 8)
                .into_iter()
                .filter(|zone| !config.zones.iter().any(|selected| selected.id == zone.id))
                .map(|zone| city_choice_label(&zone))
                .collect::<Vec<_>>()
        };
        let search_results: View = if let Some(id) = &self.city_search.chosen_id {
            let label = self.city_search.text.split(" · ").next().unwrap_or("city");
            Button::new()
                .style(ButtonStyle::Accent)
                .automation_id("AddChosenCity")
                .automation_name(format!("Add {label} time zone"))
                .is_enabled(config.zones.len() < MAX_ZONES)
                .on_click(cx.message(Message::AddCity(id.clone())))
                .content(command_content(Symbol::Add, &format!("Add {label}")))
        } else if self.city_search.filter.trim().is_empty() {
            spacer(0.0)
        } else if matches.is_empty() {
            TextBlock::new()
                .text("No new cities match your search.")
                .text_wrapping(TextWrapping::Wrap)
                .into()
        } else {
            spacer(0.0)
        };
        let city_editor: View = if let Some((index, zone)) = self
            .selected_city
            .and_then(|index| config.zones.get(index).map(|zone| (index, zone)))
        {
            focus_after_publish(
                cx,
                "city-label-focus",
                record.id.clone(),
                &self.city_label_ref,
            );
            surface().padding(Thickness::uniform(16.0)).content(
                StackPanel::new().spacing(12.0).children((
                    Grid::new()
                        .columns([GridLength::Star(1.0), GridLength::Auto])
                        .children((
                            TextBlock::new()
                                .text(format!("Edit {}", zone.display_label()))
                                .font_size(21.0)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .text_wrapping(TextWrapping::Wrap),
                            Button::new()
                                .grid_column(1)
                                .style(ButtonStyle::Subtle)
                                .automation_id("CloseCityEditor")
                                .on_click(cx.message(Message::CloseCity))
                                .content("Done"),
                        )),
                    TextBlock::new()
                        .text("Custom label")
                        .font_weight(FontWeight::SEMI_BOLD),
                    TextBox::new()
                        .element_ref(&self.city_label_ref)
                        .automation_id(format!("Label_{index}"))
                        .automation_name(format!("Custom label for {}", zone.display_label()))
                        .text(zone.label.clone().unwrap_or_default())
                        .placeholder_text("Use the city name")
                        .on_text_changed(
                            cx.callback(move |value| Message::LabelCity(index, value)),
                        ),
                    Grid::new()
                        .columns([
                            GridLength::Auto,
                            GridLength::Auto,
                            GridLength::Star(1.0),
                            GridLength::Auto,
                        ])
                        .column_spacing(6.0)
                        .children((
                            Button::new()
                                .grid_column(0)
                                .automation_id(format!("MoveUp_{index}"))
                                .automation_name(format!("Move {} earlier", zone.display_label()))
                                .is_enabled(index > 0)
                                .on_click(cx.message(Message::MoveCity(index, -1)))
                                .content("Move up"),
                            Button::new()
                                .grid_column(1)
                                .automation_id(format!("MoveDown_{index}"))
                                .automation_name(format!("Move {} later", zone.display_label()))
                                .is_enabled(index + 1 < config.zones.len())
                                .on_click(cx.message(Message::MoveCity(index, 1)))
                                .content("Move down"),
                            Button::new()
                                .grid_column(3)
                                .style(ButtonStyle::Subtle)
                                .automation_id(format!("RemoveSelected_{index}"))
                                .automation_name(format!("Remove {}", zone.display_label()))
                                .is_enabled(config.zones.len() > 1)
                                .on_click(cx.message(Message::RemoveCity(index)))
                                .content("Remove city"),
                        )),
                )),
            )
        } else {
            spacer(0.0)
        };
        let cities: View = if self
            .selected_city
            .is_some_and(|index| index < config.zones.len())
        {
            city_editor
        } else {
            surface().padding(Thickness::uniform(18.0)).content(
                StackPanel::new().spacing(16.0).children((
                    Grid::new()
                        .columns([GridLength::Star(1.0), GridLength::Auto])
                        .children((
                            TextBlock::new()
                                .text("Cities")
                                .font_size(21.0)
                                .font_weight(FontWeight::SEMI_BOLD),
                            TextBlock::new()
                                .grid_column(1)
                                .text(format!("{} of {}", config.zones.len(), MAX_ZONES))
                                .font_size(12.0)
                                .vertical_alignment(VerticalAlignment::Center),
                        )),
                    AutoSuggestBox::new()
                        .automation_id("CitySearch")
                        .automation_name("Search cities or time zones to add")
                        .min_height(40.0)
                        .text(self.city_search.text.clone())
                        .items_source(matches)
                        .is_enabled(config.zones.len() < MAX_ZONES)
                        .placeholder_text("Add a city, region, or time zone…")
                        .on_text_changed(cx.callback(Message::CitySearch))
                        .on_suggestion_chosen(cx.callback(Message::ChooseCity)),
                    search_results,
                    StackPanel::new().keyed_children(zones),
                    TextBlock::new()
                        .text("Select a city to personalize it.")
                        .font_size(12.0)
                        .opacity(quiet_opacity())
                        .text_wrapping(TextWrapping::Wrap),
                )),
            )
        };
        let full_preview =
            self.wide_home || record.size == WidgetSize::Small || self.preview_expanded;
        let preview_size = if full_preview {
            record.size
        } else {
            WidgetSize::Small
        };
        let board_preview = surface().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(16.0).children((
                Grid::new()
                    .columns([GridLength::Star(1.0), GridLength::Auto])
                    .children((
                        TextBlock::new()
                            .text("Board preview")
                            .font_size(18.0)
                            .font_weight(FontWeight::SEMI_BOLD),
                        size_badge(preview_size, 1),
                    )),
                Border::new()
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .content(preview(config, preview_size)),
                TextBlock::new()
                    .text("Resize and arrange this widget on the Windows board.")
                    .font_size(12.0)
                    .opacity(quiet_opacity())
                    .text_wrapping(TextWrapping::Wrap),
                if self.wide_home || record.size == WidgetSize::Small {
                    spacer(0.0)
                } else {
                    Button::new()
                        .style(ButtonStyle::Subtle)
                        .automation_id("ToggleBoardPreview")
                        .automation_name(if full_preview {
                            "Show compact preview"
                        } else {
                            "Show actual board size"
                        })
                        .on_click(cx.message(Message::TogglePreview))
                        .content(if full_preview {
                            "Show compact preview"
                        } else {
                            "Show actual board size"
                        })
                },
            )),
        );
        let body: View = if self.narrow && self.selected_city.is_some() {
            cities
        } else if self.wide_home {
            Grid::new()
                .columns([GridLength::Star(1.0), GridLength::Pixel(334.0)])
                .column_spacing(16.0)
                .children((cities, Border::new().grid_column(1).content(board_preview)))
        } else {
            StackPanel::new()
                .spacing(16.0)
                .children((cities, board_preview))
        };
        let heading = Grid::new()
            .columns([GridLength::Auto, GridLength::Star(1.0), GridLength::Auto])
            .column_spacing(14.0)
            .children((
                Viewbox::new()
                    .width(36.0)
                    .height(36.0)
                    .vertical_alignment(VerticalAlignment::Center)
                    .slots([SlotView::new(ViewboxSlot::Child, world_icon())]),
                StackPanel::new().grid_column(1).spacing(6.0).children((
                    TextBlock::new()
                        .text(record.display_name())
                        .font_size(28.0)
                        .font_weight(FontWeight::SEMI_BOLD)
                        .text_wrapping(TextWrapping::Wrap),
                    TextBlock::new()
                        .text(format!(
                            "{}  ·  {}",
                            type_name(record),
                            size_name(record.size)
                        ))
                        .font_size(14.0)
                        .opacity(quiet_opacity()),
                )),
                Button::new()
                    .grid_column(2)
                    .style(ButtonStyle::Subtle)
                    .automation_id("RenameInstance")
                    .vertical_alignment(VerticalAlignment::Center)
                    .on_click(cx.message(Message::SettingsExpanded(!self.settings_expanded)))
                    .content(command_content(Symbol::Edit, "Rename")),
            ));
        let name_editor: View = if self.settings_expanded {
            focus_after_publish(
                cx,
                "instance-name-focus",
                record.id.clone(),
                &self.instance_name_ref,
            );
            surface().padding(Thickness::uniform(16.0)).content(
                StackPanel::new().spacing(10.0).children((
                    TextBlock::new()
                        .text("Instance name")
                        .font_weight(FontWeight::SEMI_BOLD),
                    TextBlock::new()
                        .text("Shown only in this app. Leave blank to use the city names.")
                        .font_size(12.0)
                        .text_wrapping(TextWrapping::Wrap),
                    Grid::new()
                        .columns([GridLength::Star(1.0), GridLength::Auto, GridLength::Auto])
                        .column_spacing(8.0)
                        .children((
                            TextBox::new()
                                .element_ref(&self.instance_name_ref)
                                .automation_id("InstanceName")
                                .automation_name("Instance name")
                                .text(self.name_draft.clone())
                                .placeholder_text("Name this instance")
                                .on_text_changed(cx.callback(Message::NameDraft)),
                            Button::new()
                                .grid_column(1)
                                .style(ButtonStyle::Accent)
                                .automation_id("SaveInstanceName")
                                .on_click(cx.message(Message::SaveName))
                                .content("Save"),
                            Button::new()
                                .grid_column(2)
                                .automation_id("CancelInstanceName")
                                .on_click(cx.message(Message::SettingsExpanded(false)))
                                .content("Cancel"),
                        )),
                )),
            )
        } else {
            spacer(0.0)
        };
        let confirmation: View = if self.unpin_target.as_deref() == Some(record.id.as_str()) {
            ContentDialog::new()
                .is_open(true)
                .title(format!("Unpin {}?", record.display_name()))
                .primary_button_text("Unpin widget")
                .close_button_text("Cancel")
                .on_closed(cx.callback(|result| {
                    Message::ConfirmUnpin(result == ContentDialogResult::Primary)
                }))
                .content(
                    TextBlock::new()
                        .text(format!(
                            "Remove this {} {} from your Windows board?",
                            size_name(record.size),
                            type_name(record)
                        ))
                        .text_wrapping(TextWrapping::Wrap),
                )
        } else {
            spacer(0.0)
        };
        Grid::new()
            .margin(Thickness::uniform(if self.narrow { 0.0 } else { 20.0 }))
            .rows([GridLength::Star(1.0), GridLength::Auto])
            .row_spacing(16.0)
            .children((
                ScrollViewer::new()
                    .automation_id("ClockEditorScroll")
                    .automation_name("Widget editor")
                    .grid_row(0)
                    .content(StackPanel::new().spacing(18.0).children((
                        heading,
                        name_editor,
                        body,
                    ))),
                Grid::new()
                    .grid_row(1)
                    .columns([GridLength::Star(1.0), GridLength::Auto, GridLength::Auto])
                    .column_spacing(10.0)
                    .children((
                        TextBlock::new()
                            .text("Saved automatically")
                            .opacity(quiet_opacity())
                            .font_size(12.0)
                            .vertical_alignment(VerticalAlignment::Center),
                        Button::new()
                            .grid_column(1)
                            .automation_id("RefreshWidget")
                            .on_click(cx.message(Message::RefreshSelected))
                            .content(command_content(Symbol::Refresh, "Refresh widget")),
                        Button::new()
                            .grid_column(2)
                            .automation_id("UnpinWidget")
                            .automation_name(format!("Unpin {}", record.display_name()))
                            .is_enabled(self.unpin_target.is_none())
                            .on_click(cx.message(Message::AskUnpin))
                            .content(command_content(Symbol::UnPin, "Unpin")),
                    )),
                confirmation,
            ))
    }
}

fn focus_after_publish(
    cx: &mut ViewContext<WidgetApp>,
    key: &'static str,
    instance_id: String,
    reference: &ElementRef<TextBox>,
) {
    let reference = reference.clone();
    cx.use_effect(key, instance_id, move || {
        let _ = reference.request_focus();
        None
    });
}

fn surface() -> Border {
    Border::new()
        .background(ThemeBrush::CardBackground)
        .border_brush(ThemeBrush::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(12.0))
}
fn size_badge(size: WidgetSize, column: i32) -> View {
    Border::new()
        .grid_column(column)
        .padding(Thickness::xy(8.0, 2.0))
        .background(rail_surface())
        .border_brush(ThemeBrush::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(8.0))
        .content(TextBlock::new().text(size_name(size)).font_size(12.0))
}
fn spacer(height: f64) -> View {
    if height == 0.0 {
        View::fragment(())
    } else {
        Border::new()
            .height(height)
            .content(TextBlock::new().text(""))
    }
}
fn divider() -> View {
    Border::new()
        .height(1.0)
        .background(ThemeBrush::CardStroke)
        .content(TextBlock::new().text(""))
}
fn page_heading(title: &str, description: &str) -> View {
    StackPanel::new().spacing(8.0).children((
        TextBlock::new()
            .text(title)
            .font_size(36.0)
            .font_weight(FontWeight::SEMI_BOLD),
        TextBlock::new()
            .text(description)
            .font_size(14.0)
            .opacity(quiet_opacity()),
    ))
}
