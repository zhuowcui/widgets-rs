use clock_core::{ClockConfig, capture};
use windows_reactor::*;

/// The system Fluent globe scales cleanly and follows the text foreground.
pub(crate) fn world_icon() -> FontIcon {
    FontIcon::new().glyph("\u{e774}")
}

pub(crate) fn featured_artwork() -> EncodedImage {
    if crate::lifecycle::prefers_dark() {
        EncodedImage::from_static(include_bytes!("../../clock_core/assets/earth-limb.png"))
    } else {
        EncodedImage::from_static(include_bytes!(
            "../../clock_core/assets/earth-limb-light.png"
        ))
    }
}

pub fn preview(config: &ClockConfig, size: widget_api::WidgetSize) -> View {
    let snapshot = match capture(config, windows_time::DateTime::now()) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            return TextBlock::new()
                .text(format!("Preview unavailable: {error}"))
                .into();
        }
    };
    let limit = if size == widget_api::WidgetSize::Small {
        2
    } else if size == widget_api::WidgetSize::Medium {
        4
    } else {
        6
    };
    let visible = snapshot.rows.len().min(limit);
    let hero = size != widget_api::WidgetSize::Small && visible <= 2;
    let height = if size == widget_api::WidgetSize::Small {
        148.0
    } else if size == widget_api::WidgetSize::Medium {
        306.0
    } else {
        462.0
    };
    let backdrop = match (size, crate::lifecycle::prefers_dark()) {
        (widget_api::WidgetSize::Small, true) => {
            EncodedImage::from_static(include_bytes!("../../clock_core/assets/small-dark.jpg"))
        }
        (widget_api::WidgetSize::Small, false) => {
            EncodedImage::from_static(include_bytes!("../../clock_core/assets/small-light.jpg"))
        }
        (widget_api::WidgetSize::Medium, true) => {
            EncodedImage::from_static(include_bytes!("../../clock_core/assets/medium-dark.jpg"))
        }
        (widget_api::WidgetSize::Medium, false) => {
            EncodedImage::from_static(include_bytes!("../../clock_core/assets/medium-light.jpg"))
        }
        (_, true) => {
            EncodedImage::from_static(include_bytes!("../../clock_core/assets/large-dark.jpg"))
        }
        (_, false) => {
            EncodedImage::from_static(include_bytes!("../../clock_core/assets/large-light.jpg"))
        }
    };
    let rows = snapshot
        .rows
        .iter()
        .take(limit)
        .enumerate()
        .map(|(index, row)| {
            let face: View = if hero {
                StackPanel::new().spacing(2.0).children((
                    TextBlock::new()
                        .text(row.label.clone())
                        .font_size(15.0)
                        .font_weight(FontWeight::SEMI_BOLD),
                    TextBlock::new()
                        .text(format!("{}  ·  {}", row.offset, row.day_cue))
                        .font_size(12.0),
                    TextBlock::new()
                        .text(row.time.clone())
                        .font_size(if size == widget_api::WidgetSize::Medium {
                            48.0
                        } else {
                            56.0
                        })
                        .font_weight(FontWeight::SEMI_BOLD),
                ))
            } else {
                Grid::new()
                    .columns([GridLength::Star(1.0), GridLength::Auto])
                    .column_spacing(8.0)
                    .children((
                        StackPanel::new().grid_column(0).spacing(1.0).children((
                            TextBlock::new()
                                .text(row.label.clone())
                                .font_size(13.0)
                                .font_weight(FontWeight::SEMI_BOLD),
                            TextBlock::new().text(row.day_cue.clone()).font_size(11.0),
                        )),
                        TextBlock::new()
                            .grid_column(1)
                            .vertical_alignment(VerticalAlignment::Center)
                            .text(row.time.clone())
                            .font_size(if size == widget_api::WidgetSize::Small {
                                24.0
                            } else {
                                18.0
                            })
                            .font_weight(FontWeight::SEMI_BOLD),
                    ))
            };
            let row_height = if hero {
                if size == widget_api::WidgetSize::Medium {
                    106.0
                } else {
                    162.0
                }
            } else if size == widget_api::WidgetSize::Small {
                35.0
            } else if size == widget_api::WidgetSize::Medium {
                55.0
            } else {
                (330 / visible).clamp(55, 110) as f64
            };
            KeyedView::new(
                row.id.clone(),
                StackPanel::new().children((
                    if index == 0 {
                        Border::new().height(0.0).content(TextBlock::new().text(""))
                    } else {
                        Border::new()
                            .height(1.0)
                            .background(ThemeBrush::CardStroke)
                            .content(TextBlock::new().text(""))
                    },
                    Border::new().height(row_height).content(face),
                )),
            )
        })
        .collect::<Vec<_>>();
    let extra = snapshot.rows.len().saturating_sub(limit);
    let footer = if extra == 0 {
        format!("As of {}", snapshot.as_of)
    } else {
        format!("+{extra} more  ·  As of {}", snapshot.as_of)
    };
    Border::new()
        .width(300.0)
        .height(height)
        .background(ThemeBrush::SolidBackground)
        .border_brush(ThemeBrush::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(12.0))
        .content(
            Grid::new().children((
                Image::new().source_data(backdrop).stretch(Stretch::Fill),
                Grid::new()
                    .margin(Thickness::uniform(14.0))
                    .rows([GridLength::Auto, GridLength::Star(1.0), GridLength::Auto])
                    .children((
                        Grid::new()
                            .grid_row(0)
                            .columns([GridLength::Auto, GridLength::Star(1.0)])
                            .column_spacing(8.0)
                            .children((
                                world_icon().grid_column(0),
                                TextBlock::new()
                                    .grid_column(1)
                                    .text("World Clock")
                                    .font_size(14.0)
                                    .font_weight(FontWeight::SEMI_BOLD),
                            )),
                        StackPanel::new()
                            .grid_row(1)
                            .vertical_alignment(VerticalAlignment::Center)
                            .keyed_children(rows),
                        TextBlock::new().grid_row(2).text(footer).font_size(11.0),
                    )),
            )),
        )
}

/// A quiet two-row clock face for catalog and instance cards. It uses the same
/// captured instant and regional formatting as the board preview.
pub fn compact_preview(config: &ClockConfig) -> View {
    let snapshot = match capture(config, windows_time::DateTime::now()) {
        Ok(snapshot) => snapshot,
        Err(_) => return TextBlock::new().text("Clock preview unavailable").into(),
    };
    let rows = snapshot
        .rows
        .iter()
        .take(2)
        .enumerate()
        .map(|(index, row)| {
            KeyedView::new(
                row.id.clone(),
                StackPanel::new().spacing(10.0).children((
                    if index == 0 {
                        Border::new().height(0.0).content(TextBlock::new().text(""))
                    } else {
                        Border::new()
                            .height(1.0)
                            .background(ThemeBrush::CardStroke)
                            .content(TextBlock::new().text(""))
                    },
                    Grid::new()
                        .columns([GridLength::Star(1.0), GridLength::Auto])
                        .column_spacing(12.0)
                        .children((
                            StackPanel::new().grid_column(0).spacing(2.0).children((
                                TextBlock::new()
                                    .text(row.label.clone())
                                    .font_size(15.0)
                                    .font_weight(FontWeight::SEMI_BOLD),
                                TextBlock::new().text(row.day_cue.clone()).font_size(12.0),
                            )),
                            TextBlock::new()
                                .grid_column(1)
                                .vertical_alignment(VerticalAlignment::Center)
                                .text(row.time.clone())
                                .font_size(28.0)
                                .font_weight(FontWeight::SEMI_BOLD),
                        )),
                )),
            )
        })
        .collect::<Vec<_>>();
    StackPanel::new().spacing(10.0).keyed_children(rows)
}

/// A larger three-city composition for the featured catalog surface.
pub fn featured_preview(config: &ClockConfig) -> View {
    let snapshot = match capture(config, windows_time::DateTime::now()) {
        Ok(snapshot) => snapshot,
        Err(_) => return TextBlock::new().text("Clock preview unavailable").into(),
    };
    let rows = snapshot
        .rows
        .iter()
        .take(3)
        .enumerate()
        .map(|(index, row)| {
            KeyedView::new(
                row.id.clone(),
                StackPanel::new().spacing(10.0).children((
                    if index == 0 {
                        Border::new().height(0.0).content(TextBlock::new().text(""))
                    } else {
                        Border::new()
                            .height(1.0)
                            .background(ThemeBrush::CardStroke)
                            .content(TextBlock::new().text(""))
                    },
                    Grid::new()
                        .columns([GridLength::Star(1.0), GridLength::Auto])
                        .column_spacing(12.0)
                        .children((
                            StackPanel::new().grid_column(0).spacing(3.0).children((
                                TextBlock::new()
                                    .text(row.label.clone())
                                    .font_size(15.0)
                                    .font_weight(FontWeight::SEMI_BOLD),
                                TextBlock::new().text(row.day_cue.clone()).font_size(12.0),
                            )),
                            TextBlock::new()
                                .grid_column(1)
                                .vertical_alignment(VerticalAlignment::Center)
                                .text(row.time.clone())
                                .font_size(28.0)
                                .font_weight(FontWeight::MEDIUM),
                        )),
                )),
            )
        })
        .collect::<Vec<_>>();
    StackPanel::new().spacing(10.0).keyed_children(rows)
}
