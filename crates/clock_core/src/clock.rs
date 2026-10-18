use widget_api::Globalization::{
    Calendar, ClockIdentifiers, DateTimeFormatting::DateTimeFormatter,
};
use windows_core::{HSTRING, Result};
use windows_time::DateTime;

use crate::ClockConfig;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClockRow {
    pub id: String,
    pub label: String,
    pub time: String,
    pub day_cue: String,
    pub offset: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    pub rows: Vec<ClockRow>,
    pub as_of: String,
}

pub fn local_zone_id() -> Result<String> {
    Ok(Calendar::new()?.GetTimeZone()?.to_string_lossy())
}

pub fn validate_zone(id: &str) -> bool {
    Calendar::new()
        .and_then(|calendar| calendar.ChangeTimeZone(&HSTRING::from(id)))
        .is_ok()
}

pub fn capture(config: &ClockConfig, instant: DateTime) -> Result<Snapshot> {
    let formatter = DateTimeFormatter::ShortTime()?;
    let local = calendar_at(instant, None)?;
    let local_day = day_number(local.Year()?, local.Month()?, local.Day()?);
    let utc = calendar_at(instant, Some(&HSTRING::from("Etc/UTC")))?;
    let utc_minute = day_number(utc.Year()?, utc.Month()?, utc.Day()?) * 1440
        + utc.Hour()? * 60
        + utc.Minute()?;
    let mut rows = Vec::with_capacity(config.zones.len());
    for selected in &config.zones {
        let zone = HSTRING::from(selected.id.as_str());
        let result = calendar_at(instant, Some(&zone)).and_then(|calendar| {
            let day = day_number(calendar.Year()?, calendar.Month()?, calendar.Day()?);
            let text = formatter
                .FormatUsingTimeZone(instant, &zone)?
                .to_string_lossy();
            let zoned_minute = day * 1440 + calendar.Hour()? * 60 + calendar.Minute()?;
            Ok((
                text,
                day - local_day,
                format_offset(zoned_minute - utc_minute),
            ))
        });
        let (time, difference, offset) = match result {
            Ok(value) => value,
            Err(_) => {
                rows.push(ClockRow {
                    id: selected.id.clone(),
                    label: selected.display_label(),
                    time: "Unavailable".into(),
                    day_cue: "Check time zone".into(),
                    offset: "UTC offset unavailable".into(),
                });
                continue;
            }
        };
        rows.push(ClockRow {
            id: selected.id.clone(),
            label: selected.display_label(),
            time,
            offset,
            day_cue: match difference {
                -1 => "Yesterday".into(),
                0 => "Today".into(),
                1 => "Tomorrow".into(),
                n if n < 0 => format!("{} days earlier", -n),
                n => format!("{n} days later"),
            },
        });
    }
    Ok(Snapshot {
        rows,
        as_of: formatter.Format(instant)?.to_string_lossy(),
    })
}

fn format_offset(minutes: i32) -> String {
    let sign = if minutes < 0 { '−' } else { '+' };
    let amount = minutes.abs();
    if amount % 60 == 0 {
        format!("UTC{sign}{}", amount / 60)
    } else {
        format!("UTC{sign}{}:{:02}", amount / 60, amount % 60)
    }
}

fn calendar_at(instant: DateTime, zone: Option<&HSTRING>) -> Result<Calendar> {
    let calendar = Calendar::new()?;
    calendar.ChangeCalendarSystem(&HSTRING::from("GregorianCalendar"))?;
    // Calendar.Hour follows the user's 12/24-hour preference. Arithmetic must
    // use a 24-hour clock even though the visible time keeps that preference.
    calendar.ChangeClock(&ClockIdentifiers::TwentyFourHour()?)?;
    calendar.SetDateTime(instant)?;
    if let Some(zone) = zone {
        calendar.ChangeTimeZone(zone)?;
    }
    Ok(calendar)
}

// Civil-date ordinal (days since a fixed epoch); valid for Gregorian dates.
fn day_number(year: i32, month: i32, day: i32) -> i32 {
    let year = year - i32::from(month <= 2);
    let era = year.div_euclid(400);
    let yoe = year - era * 400;
    let shifted_month = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * shifted_month + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_collections::IIterable;

    #[test]
    fn day_number_handles_leap_year_and_midnight() {
        assert_eq!(day_number(2024, 3, 1) - day_number(2024, 2, 29), 1);
        assert_eq!(day_number(2023, 3, 1) - day_number(2023, 2, 28), 1);
        assert_eq!(day_number(2025, 1, 1) - day_number(2024, 12, 31), 1);
    }

    #[test]
    fn detects_invalid_zone() {
        assert!(!validate_zone("Mars/Olympus_Mons"));
        assert!(validate_zone("Asia/Kathmandu"));
    }

    #[test]
    fn default_local_clock_is_renderable() {
        let local = local_zone_id().unwrap();
        assert!(validate_zone(&local));
        let snapshot = capture(&ClockConfig::new_default(&local), DateTime::now()).unwrap();
        assert!(!snapshot.rows.is_empty());
        assert!(snapshot.rows.iter().all(|row| row.time != "Unavailable"));
    }

    #[test]
    fn dst_and_half_hour_offsets() {
        let config = ClockConfig {
            version: 1,
            name: None,
            zones: ["America/New_York", "Asia/Kathmandu"]
                .into_iter()
                .map(|id| crate::ZoneSelection {
                    id: id.into(),
                    label: None,
                })
                .collect(),
        };
        let zone = HSTRING::from("America/New_York");
        let before = calendar_at(DateTime::from_unix_secs(1_710_052_200), Some(&zone)).unwrap();
        let after = calendar_at(DateTime::from_unix_secs(1_710_055_800), Some(&zone)).unwrap();
        assert_eq!((before.Hour().unwrap(), before.Minute().unwrap()), (1, 30));
        assert_eq!((after.Hour().unwrap(), after.Minute().unwrap()), (3, 30));

        let fall_first = calendar_at(DateTime::from_unix_secs(1_730_611_800), Some(&zone)).unwrap();
        let fall_second =
            calendar_at(DateTime::from_unix_secs(1_730_615_400), Some(&zone)).unwrap();
        assert_eq!(
            (fall_first.Hour().unwrap(), fall_first.Minute().unwrap()),
            (1, 30)
        );
        assert_eq!(
            (fall_second.Hour().unwrap(), fall_second.Minute().unwrap()),
            (1, 30)
        );

        let kathmandu = HSTRING::from("Asia/Kathmandu");
        let utc = HSTRING::from("Etc/UTC");
        let instant = DateTime::from_unix_secs(1_710_052_200);
        let offset = calendar_at(instant, Some(&kathmandu)).unwrap();
        let base = calendar_at(instant, Some(&utc)).unwrap();
        assert_eq!((offset.Hour().unwrap(), offset.Minute().unwrap()), (12, 15));
        assert_eq!((base.Hour().unwrap(), base.Minute().unwrap()), (6, 30));

        let snapshot = capture(&config, instant).unwrap();
        let regional = DateTimeFormatter::ShortTime()
            .unwrap()
            .FormatUsingTimeZone(instant, &zone)
            .unwrap()
            .to_string_lossy();
        assert_eq!(snapshot.rows[0].time, regional);
        assert_eq!(snapshot.rows[0].offset, "UTC−5");
        assert_eq!(snapshot.rows[1].offset, "UTC+5:45");
        assert_eq!(
            capture(&config, DateTime::from_unix_secs(1_710_055_800))
                .unwrap()
                .rows[0]
                .offset,
            "UTC−4"
        );
    }

    #[test]
    fn offset_is_correct_at_utc_midnight_with_twelve_hour_display() {
        let config = ClockConfig {
            version: 1,
            name: None,
            zones: ["America/Los_Angeles", "Etc/UTC"]
                .into_iter()
                .map(|id| crate::ZoneSelection {
                    id: id.into(),
                    label: None,
                })
                .collect(),
        };
        let snapshot = capture(&config, DateTime::from_unix_secs(1_790_640_000)).unwrap();
        assert_eq!(snapshot.rows[0].offset, "UTC−7");
        assert_eq!(snapshot.rows[1].offset, "UTC+0");
    }

    #[test]
    fn supports_twelve_and_twenty_four_hour_windows_formatters() {
        let languages: IIterable<HSTRING> = vec![HSTRING::from("en-US")].into();
        let template = HSTRING::from("hour minute");
        let region = HSTRING::from("US");
        let calendar = HSTRING::from("GregorianCalendar");
        let zone = HSTRING::from("Etc/UTC");
        let instant = DateTime::from_unix_secs(1_710_077_700);
        let twelve = DateTimeFormatter::CreateDateTimeFormatterContext(
            &template,
            &languages,
            &region,
            &calendar,
            &ClockIdentifiers::TwelveHour().unwrap(),
        )
        .unwrap()
        .FormatUsingTimeZone(instant, &zone)
        .unwrap()
        .to_string_lossy();
        let twenty_four = DateTimeFormatter::CreateDateTimeFormatterContext(
            &template,
            &languages,
            &region,
            &calendar,
            &ClockIdentifiers::TwentyFourHour().unwrap(),
        )
        .unwrap()
        .FormatUsingTimeZone(instant, &zone)
        .unwrap()
        .to_string_lossy();
        assert_ne!(twelve, twenty_four);
        assert!(twelve.contains("PM") || twelve.contains("AM"));
        assert!(!twenty_four.contains("PM") && !twenty_four.contains("AM"));
    }
}
