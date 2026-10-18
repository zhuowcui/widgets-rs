use base64::{Engine, engine::general_purpose::STANDARD};
use std::{ffi::c_void, fmt::Write, sync::OnceLock};
use ttf_parser::{Face, OutlineBuilder};

/// A time face built from the installed Windows font. Only outlines are sent to
/// the host; no font file is copied into the package or the widget payload.
pub(crate) struct TimeVisual {
    pub light: String,
    pub dark: String,
    pub width: u32,
    pub height: u32,
}

static FONT: OnceLock<Option<Vec<u8>>> = OnceLock::new();

pub(crate) fn time_visual(text: &str, font_size: f32) -> Option<TimeVisual> {
    // DateTimeFormatter inserts bidi controls around time fields. DirectWrite
    // treats these as invisible; outlining their font glyphs would draw arrows.
    // Use native text for scripts that require shaping or a different direction.
    let text = outline_text(text)?;
    let bytes = FONT.get_or_init(|| {
        let windows = std::env::var_os("WINDIR")?;
        std::fs::read(std::path::PathBuf::from(windows).join("Fonts/seguisb.ttf")).ok()
    });
    let face = Face::parse(bytes.as_ref()?, 0).ok()?;
    let glyphs = text
        .chars()
        .map(|character| face.glyph_index(character))
        .collect::<Option<Vec<_>>>()?;
    let mut advance = 0.0f32;
    let mut top = i16::MIN;
    let mut bottom = i16::MAX;
    for &glyph in &glyphs {
        advance += face.glyph_hor_advance(glyph)? as f32;
        if let Some(bounds) = face.glyph_bounding_box(glyph) {
            top = top.max(bounds.y_max);
            bottom = bottom.min(bounds.y_min);
        }
    }
    if top <= bottom || advance <= 0.0 {
        return None;
    }
    let scale = (font_size / face.units_per_em() as f32).min(254.0 / advance);
    let width = (advance * scale + 2.0).ceil() as u32;
    let height = ((top - bottom) as f32 * scale + 2.0).ceil() as u32;
    let mut outline = SvgOutline {
        path: String::new(),
        scale,
        x: 1.0,
        baseline: top as f32 * scale + 1.0,
    };
    for glyph in glyphs {
        face.outline_glyph(glyph, &mut outline);
        outline.x += face.glyph_hor_advance(glyph)? as f32 * scale;
    }
    let svg = |fill: &str| {
        let source = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\"><path fill=\"{fill}\" stroke=\"none\" fill-rule=\"nonzero\" marker-start=\"none\" marker-mid=\"none\" marker-end=\"none\" d=\"{}\"/></svg>",
            outline.path
        );
        format!("data:image/svg+xml;base64,{}", STANDARD.encode(source))
    };
    Some(TimeVisual {
        light: svg("#151A22"),
        dark: svg("#F7FAFF"),
        width,
        height,
    })
}

fn outline_text(text: &str) -> Option<String> {
    text.chars()
        .filter(|character| !matches!(character, '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'))
        .map(|character| match character {
            '\u{00a0}' | '\u{202f}' => Some(' '),
            character if character.is_ascii_graphic() || character == ' ' => Some(character),
            _ => None,
        })
        .collect()
}

struct SvgOutline {
    path: String,
    scale: f32,
    x: f32,
    baseline: f32,
}
impl SvgOutline {
    fn point(&self, x: f32, y: f32) -> (f32, f32) {
        (self.x + x * self.scale, self.baseline - y * self.scale)
    }
}
impl OutlineBuilder for SvgOutline {
    fn move_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.point(x, y);
        let _ = write!(self.path, " M {x:.2} {y:.2}");
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.point(x, y);
        let _ = write!(self.path, " L {x:.2} {y:.2}");
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (x1, y1) = self.point(x1, y1);
        let (x, y) = self.point(x, y);
        let _ = write!(self.path, " Q {x1:.2} {y1:.2} {x:.2} {y:.2}");
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (x1, y1) = self.point(x1, y1);
        let (x2, y2) = self.point(x2, y2);
        let (x, y) = self.point(x, y);
        let _ = write!(
            self.path,
            " C {x1:.2} {y1:.2} {x2:.2} {y2:.2} {x:.2} {y:.2}"
        );
    }
    fn close(&mut self) {
        self.path.push_str(" Z");
    }
}

#[repr(C)]
struct HighContrast {
    size: u32,
    flags: u32,
    scheme: *mut u16,
}
#[link(name = "user32")]
unsafe extern "system" {
    fn SystemParametersInfoW(action: u32, size: u32, data: *mut c_void, flags: u32) -> i32;
}
pub fn high_contrast_enabled() -> bool {
    let mut value = HighContrast {
        size: std::mem::size_of::<HighContrast>() as u32,
        flags: 0,
        scheme: std::ptr::null_mut(),
    };
    unsafe {
        SystemParametersInfoW(
            0x42,
            value.size,
            (&mut value as *mut HighContrast).cast(),
            0,
        ) != 0
            && value.flags & 1 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_outlines_are_bounded_and_have_theme_variants() {
        for text in ["9:05 AM", "12:59 PM", "23:59", "00:00"] {
            let image = time_visual(text, 56.0).expect("installed Segoe UI Semibold");
            assert!(image.width <= 256 && image.height <= 56);
            let source = STANDARD
                .decode(image.dark.trim_start_matches("data:image/svg+xml;base64,"))
                .unwrap();
            let source = String::from_utf8(source).unwrap();
            assert!(source.contains("<path") && source.contains("#F7FAFF"));
            assert!(!source.contains("<text"));
            assert_ne!(image.light, image.dark);
        }
        assert!(time_visual("", 48.0).is_none());
        assert!(time_visual("\u{10ffff}", 48.0).is_none());
        assert!(time_visual("١٢:٢٩ ص", 48.0).is_none());
        assert_eq!(
            outline_text("\u{200e}12\u{200e}:\u{200e}29\u{200e}\u{202f}AM"),
            Some("12:29 AM".into())
        );
        assert_eq!(
            time_visual("\u{200e}12\u{200e}:\u{200e}29\u{200e} AM", 48.0)
                .unwrap()
                .dark,
            time_visual("12:29 AM", 48.0).unwrap().dark
        );
    }
}
