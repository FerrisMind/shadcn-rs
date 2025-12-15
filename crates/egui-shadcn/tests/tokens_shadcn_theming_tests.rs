use egui::Color32;
use egui_shadcn::{ColorPalette, ShadcnBaseColor};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[derive(Clone, Copy, Debug)]
struct Oklch {
    l: f32,
    c: f32,
    h_deg: f32,
    alpha: f32,
}

impl Oklch {
    const fn new(l: f32, c: f32, h_deg: f32) -> Self {
        Self {
            l,
            c,
            h_deg,
            alpha: 1.0,
        }
    }

    const fn with_alpha(l: f32, c: f32, h_deg: f32, alpha: f32) -> Self {
        Self { l, c, h_deg, alpha }
    }
}

fn oklch_to_color32(oklch: Oklch) -> Color32 {
    let h_rad = oklch.h_deg.to_radians();
    let a = oklch.c * h_rad.cos();
    let b = oklch.c * h_rad.sin();

    // OKLab -> LMS (non-linear)
    let l_ = oklch.l + 0.396_337_777_4 * a + 0.215_803_757_3 * b;
    let m_ = oklch.l - 0.105_561_345_8 * a - 0.063_854_172_8 * b;
    let s_ = oklch.l - 0.089_484_177_5 * a - 1.291_485_548_0 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    // LMS -> linear sRGB
    let r_lin = 4.076_741_662_1 * l - 3.307_711_591_3 * m + 0.230_969_929_2 * s;
    let g_lin = -1.268_438_004_6 * l + 2.609_757_401_1 * m - 0.341_319_396_5 * s;
    let b_lin = -0.004_196_086_3 * l - 0.703_418_614_7 * m + 1.707_614_701_0 * s;

    fn to_srgb_u8(linear: f32) -> u8 {
        let clamped = linear.clamp(0.0, 1.0);
        let srgb = if clamped <= 0.003_130_8 {
            12.92 * clamped
        } else {
            1.055 * clamped.powf(1.0 / 2.4) - 0.055
        };
        (srgb.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    let r = to_srgb_u8(r_lin);
    let g = to_srgb_u8(g_lin);
    let b = to_srgb_u8(b_lin);
    let a = (oklch.alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

fn assert_color_close(actual: Color32, expected: Color32, label: &str) {
    let [ar, ag, ab, aa] = actual.to_array();
    let [er, eg, eb, ea] = expected.to_array();
    let channel_close = |a: u8, e: u8| (a as i16 - e as i16).abs() <= 1;
    assert!(
        channel_close(ar, er)
            && channel_close(ag, eg)
            && channel_close(ab, eb)
            && channel_close(aa, ea),
        "{label}: expected rgba({er},{eg},{eb},{ea}), got rgba({ar},{ag},{ab},{aa})",
    );
}

fn assert_palette_matches_doc(
    actual: &ColorPalette,
    background: Oklch,
    foreground: Oklch,
    card: Oklch,
    card_foreground: Oklch,
    popover: Oklch,
    popover_foreground: Oklch,
    primary: Oklch,
    primary_foreground: Oklch,
    secondary: Oklch,
    secondary_foreground: Oklch,
    muted: Oklch,
    muted_foreground: Oklch,
    accent: Oklch,
    accent_foreground: Oklch,
    destructive: Oklch,
    border: Oklch,
    input: Oklch,
    ring: Oklch,
    chart_1: Oklch,
    chart_2: Oklch,
    chart_3: Oklch,
    chart_4: Oklch,
    chart_5: Oklch,
    sidebar: Oklch,
    sidebar_foreground: Oklch,
    sidebar_primary: Oklch,
    sidebar_primary_foreground: Oklch,
    sidebar_accent: Oklch,
    sidebar_accent_foreground: Oklch,
    sidebar_border: Oklch,
    sidebar_ring: Oklch,
    prefix: &str,
) {
    assert_color_close(
        actual.background,
        oklch_to_color32(background),
        &format!("{prefix}.background"),
    );
    assert_color_close(
        actual.foreground,
        oklch_to_color32(foreground),
        &format!("{prefix}.foreground"),
    );
    assert_color_close(
        actual.card,
        oklch_to_color32(card),
        &format!("{prefix}.card"),
    );
    assert_color_close(
        actual.card_foreground,
        oklch_to_color32(card_foreground),
        &format!("{prefix}.card_foreground"),
    );
    assert_color_close(
        actual.popover,
        oklch_to_color32(popover),
        &format!("{prefix}.popover"),
    );
    assert_color_close(
        actual.popover_foreground,
        oklch_to_color32(popover_foreground),
        &format!("{prefix}.popover_foreground"),
    );
    assert_color_close(
        actual.primary,
        oklch_to_color32(primary),
        &format!("{prefix}.primary"),
    );
    assert_color_close(
        actual.primary_foreground,
        oklch_to_color32(primary_foreground),
        &format!("{prefix}.primary_foreground"),
    );
    assert_color_close(
        actual.secondary,
        oklch_to_color32(secondary),
        &format!("{prefix}.secondary"),
    );
    assert_color_close(
        actual.secondary_foreground,
        oklch_to_color32(secondary_foreground),
        &format!("{prefix}.secondary_foreground"),
    );
    assert_color_close(
        actual.muted,
        oklch_to_color32(muted),
        &format!("{prefix}.muted"),
    );
    assert_color_close(
        actual.muted_foreground,
        oklch_to_color32(muted_foreground),
        &format!("{prefix}.muted_foreground"),
    );
    assert_color_close(
        actual.accent,
        oklch_to_color32(accent),
        &format!("{prefix}.accent"),
    );
    assert_color_close(
        actual.accent_foreground,
        oklch_to_color32(accent_foreground),
        &format!("{prefix}.accent_foreground"),
    );
    assert_color_close(
        actual.destructive,
        oklch_to_color32(destructive),
        &format!("{prefix}.destructive"),
    );
    assert_color_close(
        actual.border,
        oklch_to_color32(border),
        &format!("{prefix}.border"),
    );
    assert_color_close(
        actual.input,
        oklch_to_color32(input),
        &format!("{prefix}.input"),
    );
    assert_color_close(
        actual.ring,
        oklch_to_color32(ring),
        &format!("{prefix}.ring"),
    );
    assert_color_close(
        actual.chart_1,
        oklch_to_color32(chart_1),
        &format!("{prefix}.chart_1"),
    );
    assert_color_close(
        actual.chart_2,
        oklch_to_color32(chart_2),
        &format!("{prefix}.chart_2"),
    );
    assert_color_close(
        actual.chart_3,
        oklch_to_color32(chart_3),
        &format!("{prefix}.chart_3"),
    );
    assert_color_close(
        actual.chart_4,
        oklch_to_color32(chart_4),
        &format!("{prefix}.chart_4"),
    );
    assert_color_close(
        actual.chart_5,
        oklch_to_color32(chart_5),
        &format!("{prefix}.chart_5"),
    );
    assert_color_close(
        actual.sidebar,
        oklch_to_color32(sidebar),
        &format!("{prefix}.sidebar"),
    );
    assert_color_close(
        actual.sidebar_foreground,
        oklch_to_color32(sidebar_foreground),
        &format!("{prefix}.sidebar_foreground"),
    );
    assert_color_close(
        actual.sidebar_primary,
        oklch_to_color32(sidebar_primary),
        &format!("{prefix}.sidebar_primary"),
    );
    assert_color_close(
        actual.sidebar_primary_foreground,
        oklch_to_color32(sidebar_primary_foreground),
        &format!("{prefix}.sidebar_primary_foreground"),
    );
    assert_color_close(
        actual.sidebar_accent,
        oklch_to_color32(sidebar_accent),
        &format!("{prefix}.sidebar_accent"),
    );
    assert_color_close(
        actual.sidebar_accent_foreground,
        oklch_to_color32(sidebar_accent_foreground),
        &format!("{prefix}.sidebar_accent_foreground"),
    );
    assert_color_close(
        actual.sidebar_border,
        oklch_to_color32(sidebar_border),
        &format!("{prefix}.sidebar_border"),
    );
    assert_color_close(
        actual.sidebar_ring,
        oklch_to_color32(sidebar_ring),
        &format!("{prefix}.sidebar_ring"),
    );

    // Extension (not in shadcn docs): we keep `destructive_foreground` for compatibility.
    assert_eq!(
        actual.destructive_foreground.a(),
        255,
        "{prefix}.destructive_foreground should be opaque",
    );
}

#[test]
fn shadcn_neutral_matches_doc_light_and_dark() {
    init_logger();

    let chart_light = [
        Oklch::new(0.646, 0.222, 41.116),
        Oklch::new(0.6, 0.118, 184.704),
        Oklch::new(0.398, 0.07, 227.392),
        Oklch::new(0.828, 0.189, 84.429),
        Oklch::new(0.769, 0.188, 70.08),
    ];
    let chart_dark = [
        Oklch::new(0.488, 0.243, 264.376),
        Oklch::new(0.696, 0.17, 162.48),
        Oklch::new(0.769, 0.188, 70.08),
        Oklch::new(0.627, 0.265, 303.9),
        Oklch::new(0.645, 0.246, 16.439),
    ];

    let light = ColorPalette::shadcn_light(ShadcnBaseColor::Neutral);
    assert_palette_matches_doc(
        &light,
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.145, 0.0, 0.0),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.145, 0.0, 0.0),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.145, 0.0, 0.0),
        Oklch::new(0.205, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.97, 0.0, 0.0),
        Oklch::new(0.205, 0.0, 0.0),
        Oklch::new(0.97, 0.0, 0.0),
        Oklch::new(0.556, 0.0, 0.0),
        Oklch::new(0.97, 0.0, 0.0),
        Oklch::new(0.205, 0.0, 0.0),
        Oklch::new(0.577, 0.245, 27.325),
        Oklch::new(0.922, 0.0, 0.0),
        Oklch::new(0.922, 0.0, 0.0),
        Oklch::new(0.708, 0.0, 0.0),
        chart_light[0],
        chart_light[1],
        chart_light[2],
        chart_light[3],
        chart_light[4],
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.145, 0.0, 0.0),
        Oklch::new(0.205, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.97, 0.0, 0.0),
        Oklch::new(0.205, 0.0, 0.0),
        Oklch::new(0.922, 0.0, 0.0),
        Oklch::new(0.708, 0.0, 0.0),
        "neutral.light",
    );

    let dark = ColorPalette::shadcn_dark(ShadcnBaseColor::Neutral);
    assert_palette_matches_doc(
        &dark,
        Oklch::new(0.145, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.205, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.269, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.922, 0.0, 0.0),
        Oklch::new(0.205, 0.0, 0.0),
        Oklch::new(0.269, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.269, 0.0, 0.0),
        Oklch::new(0.708, 0.0, 0.0),
        Oklch::new(0.371, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.704, 0.191, 22.216),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.15),
        Oklch::new(0.556, 0.0, 0.0),
        chart_dark[0],
        chart_dark[1],
        chart_dark[2],
        chart_dark[3],
        chart_dark[4],
        Oklch::new(0.205, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.488, 0.243, 264.376),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.269, 0.0, 0.0),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::new(0.439, 0.0, 0.0),
        "neutral.dark",
    );

    // Also validate the shorthand constructors align to Neutral.
    assert_eq!(ColorPalette::default(), dark);
    assert_eq!(ColorPalette::dark(), dark);
    assert_eq!(ColorPalette::light(), light);
}

#[test]
fn shadcn_other_base_colors_match_doc_light_and_dark() {
    init_logger();

    let chart_light = [
        Oklch::new(0.646, 0.222, 41.116),
        Oklch::new(0.6, 0.118, 184.704),
        Oklch::new(0.398, 0.07, 227.392),
        Oklch::new(0.828, 0.189, 84.429),
        Oklch::new(0.769, 0.188, 70.08),
    ];
    let chart_dark = [
        Oklch::new(0.488, 0.243, 264.376),
        Oklch::new(0.696, 0.17, 162.48),
        Oklch::new(0.769, 0.188, 70.08),
        Oklch::new(0.627, 0.265, 303.9),
        Oklch::new(0.645, 0.246, 16.439),
    ];

    // Stone
    let stone_light = ColorPalette::shadcn_light(ShadcnBaseColor::Stone);
    assert_palette_matches_doc(
        &stone_light,
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.147, 0.004, 49.25),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.147, 0.004, 49.25),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.147, 0.004, 49.25),
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.97, 0.001, 106.424),
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.97, 0.001, 106.424),
        Oklch::new(0.553, 0.013, 58.071),
        Oklch::new(0.97, 0.001, 106.424),
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.577, 0.245, 27.325),
        Oklch::new(0.923, 0.003, 48.717),
        Oklch::new(0.923, 0.003, 48.717),
        Oklch::new(0.709, 0.01, 56.259),
        chart_light[0],
        chart_light[1],
        chart_light[2],
        chart_light[3],
        chart_light[4],
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.147, 0.004, 49.25),
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.97, 0.001, 106.424),
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.923, 0.003, 48.717),
        Oklch::new(0.709, 0.01, 56.259),
        "stone.light",
    );

    let stone_dark = ColorPalette::shadcn_dark(ShadcnBaseColor::Stone);
    assert_palette_matches_doc(
        &stone_dark,
        Oklch::new(0.147, 0.004, 49.25),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.923, 0.003, 48.717),
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.268, 0.007, 34.298),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.268, 0.007, 34.298),
        Oklch::new(0.709, 0.01, 56.259),
        Oklch::new(0.268, 0.007, 34.298),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.704, 0.191, 22.216),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.15),
        Oklch::new(0.553, 0.013, 58.071),
        chart_dark[0],
        chart_dark[1],
        chart_dark[2],
        chart_dark[3],
        chart_dark[4],
        Oklch::new(0.216, 0.006, 56.043),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.488, 0.243, 264.376),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::new(0.268, 0.007, 34.298),
        Oklch::new(0.985, 0.001, 106.423),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::new(0.553, 0.013, 58.071),
        "stone.dark",
    );

    // Zinc
    let zinc_light = ColorPalette::shadcn_light(ShadcnBaseColor::Zinc);
    assert_palette_matches_doc(
        &zinc_light,
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.141, 0.005, 285.823),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.141, 0.005, 285.823),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.141, 0.005, 285.823),
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.967, 0.001, 286.375),
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.967, 0.001, 286.375),
        Oklch::new(0.552, 0.016, 285.938),
        Oklch::new(0.967, 0.001, 286.375),
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.577, 0.245, 27.325),
        Oklch::new(0.92, 0.004, 286.32),
        Oklch::new(0.92, 0.004, 286.32),
        Oklch::new(0.705, 0.015, 286.067),
        chart_light[0],
        chart_light[1],
        chart_light[2],
        chart_light[3],
        chart_light[4],
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.141, 0.005, 285.823),
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.967, 0.001, 286.375),
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.92, 0.004, 286.32),
        Oklch::new(0.705, 0.015, 286.067),
        "zinc.light",
    );

    let zinc_dark = ColorPalette::shadcn_dark(ShadcnBaseColor::Zinc);
    assert_palette_matches_doc(
        &zinc_dark,
        Oklch::new(0.141, 0.005, 285.823),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.92, 0.004, 286.32),
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.274, 0.006, 286.033),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.274, 0.006, 286.033),
        Oklch::new(0.705, 0.015, 286.067),
        Oklch::new(0.274, 0.006, 286.033),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.704, 0.191, 22.216),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.15),
        Oklch::new(0.552, 0.016, 285.938),
        chart_dark[0],
        chart_dark[1],
        chart_dark[2],
        chart_dark[3],
        chart_dark[4],
        Oklch::new(0.21, 0.006, 285.885),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.488, 0.243, 264.376),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::new(0.274, 0.006, 286.033),
        Oklch::new(0.985, 0.0, 0.0),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::new(0.552, 0.016, 285.938),
        "zinc.dark",
    );

    // Gray
    let gray_light = ColorPalette::shadcn_light(ShadcnBaseColor::Gray);
    assert_palette_matches_doc(
        &gray_light,
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.13, 0.028, 261.692),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.13, 0.028, 261.692),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.13, 0.028, 261.692),
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.967, 0.003, 264.542),
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.967, 0.003, 264.542),
        Oklch::new(0.551, 0.027, 264.364),
        Oklch::new(0.967, 0.003, 264.542),
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.577, 0.245, 27.325),
        Oklch::new(0.928, 0.006, 264.531),
        Oklch::new(0.928, 0.006, 264.531),
        Oklch::new(0.707, 0.022, 261.325),
        chart_light[0],
        chart_light[1],
        chart_light[2],
        chart_light[3],
        chart_light[4],
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.13, 0.028, 261.692),
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.967, 0.003, 264.542),
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.928, 0.006, 264.531),
        Oklch::new(0.707, 0.022, 261.325),
        "gray.light",
    );

    let gray_dark = ColorPalette::shadcn_dark(ShadcnBaseColor::Gray);
    assert_palette_matches_doc(
        &gray_dark,
        Oklch::new(0.13, 0.028, 261.692),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.928, 0.006, 264.531),
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.278, 0.033, 256.848),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.278, 0.033, 256.848),
        Oklch::new(0.707, 0.022, 261.325),
        Oklch::new(0.278, 0.033, 256.848),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.704, 0.191, 22.216),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.15),
        Oklch::new(0.551, 0.027, 264.364),
        chart_dark[0],
        chart_dark[1],
        chart_dark[2],
        chart_dark[3],
        chart_dark[4],
        Oklch::new(0.21, 0.034, 264.665),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.488, 0.243, 264.376),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::new(0.278, 0.033, 256.848),
        Oklch::new(0.985, 0.002, 247.839),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::new(0.551, 0.027, 264.364),
        "gray.dark",
    );

    // Slate
    let slate_light = ColorPalette::shadcn_light(ShadcnBaseColor::Slate);
    assert_palette_matches_doc(
        &slate_light,
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.129, 0.042, 264.695),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.129, 0.042, 264.695),
        Oklch::new(1.0, 0.0, 0.0),
        Oklch::new(0.129, 0.042, 264.695),
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.968, 0.007, 247.896),
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.968, 0.007, 247.896),
        Oklch::new(0.554, 0.046, 257.417),
        Oklch::new(0.968, 0.007, 247.896),
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.577, 0.245, 27.325),
        Oklch::new(0.929, 0.013, 255.508),
        Oklch::new(0.929, 0.013, 255.508),
        Oklch::new(0.704, 0.04, 256.788),
        chart_light[0],
        chart_light[1],
        chart_light[2],
        chart_light[3],
        chart_light[4],
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.129, 0.042, 264.695),
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.968, 0.007, 247.896),
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.929, 0.013, 255.508),
        Oklch::new(0.704, 0.04, 256.788),
        "slate.light",
    );

    let slate_dark = ColorPalette::shadcn_dark(ShadcnBaseColor::Slate);
    assert_palette_matches_doc(
        &slate_dark,
        Oklch::new(0.129, 0.042, 264.695),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.929, 0.013, 255.508),
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.279, 0.041, 260.031),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.279, 0.041, 260.031),
        Oklch::new(0.704, 0.04, 256.788),
        Oklch::new(0.279, 0.041, 260.031),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.704, 0.191, 22.216),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.15),
        Oklch::new(0.551, 0.027, 264.364),
        chart_dark[0],
        chart_dark[1],
        chart_dark[2],
        chart_dark[3],
        chart_dark[4],
        Oklch::new(0.208, 0.042, 265.755),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.488, 0.243, 264.376),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::new(0.279, 0.041, 260.031),
        Oklch::new(0.984, 0.003, 247.858),
        Oklch::with_alpha(1.0, 0.0, 0.0, 0.10),
        Oklch::new(0.551, 0.027, 264.364),
        "slate.dark",
    );
}
