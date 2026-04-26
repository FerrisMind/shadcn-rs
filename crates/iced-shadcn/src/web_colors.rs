//! Helpers for keeping iced surface formats aligned with web-colors mode.
//!
//! `iced` switches image atlas texture formats and shader color packing when
//! `web-colors` is enabled. Applications that own their own `wgpu::Surface`
//! should use the same color mode when choosing the swapchain format.

use iced_wgpu::graphics;

pub use iced_wgpu::wgpu;

/// Returns whether the current iced renderer build performs gamma correction.
pub fn gamma_correction_enabled() -> bool {
    graphics::color::GAMMA_CORRECTION
}

/// Selects a surface format compatible with the current iced color mode.
pub fn select_surface_format_for_iced_color_mode(
    formats: &[wgpu::TextureFormat],
) -> Option<wgpu::TextureFormat> {
    select_surface_format_for_gamma_correction(formats, gamma_correction_enabled())
}

/// Selects a surface format for an explicit gamma-correction mode.
pub fn select_surface_format_for_gamma_correction(
    formats: &[wgpu::TextureFormat],
    gamma_correction: bool,
) -> Option<wgpu::TextureFormat> {
    let preferred = if gamma_correction {
        formats.iter().copied().find(wgpu::TextureFormat::is_srgb)
    } else {
        formats.iter().copied().find(|format| !format.is_srgb())
    };

    preferred.or_else(|| formats.first().copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gamma_corrected_mode_prefers_srgb_surface() {
        let selected = select_surface_format_for_gamma_correction(
            &[
                wgpu::TextureFormat::Bgra8Unorm,
                wgpu::TextureFormat::Bgra8UnormSrgb,
            ],
            true,
        );

        assert_eq!(selected, Some(wgpu::TextureFormat::Bgra8UnormSrgb));
    }

    #[test]
    fn web_colors_mode_prefers_non_srgb_surface() {
        let selected = select_surface_format_for_gamma_correction(
            &[
                wgpu::TextureFormat::Bgra8UnormSrgb,
                wgpu::TextureFormat::Bgra8Unorm,
            ],
            false,
        );

        assert_eq!(selected, Some(wgpu::TextureFormat::Bgra8Unorm));
    }

    #[test]
    fn falls_back_to_first_surface_format_when_preferred_mode_is_unavailable() {
        let selected = select_surface_format_for_gamma_correction(
            &[wgpu::TextureFormat::Bgra8UnormSrgb],
            false,
        );

        assert_eq!(selected, Some(wgpu::TextureFormat::Bgra8UnormSrgb));
    }

    #[cfg(feature = "web-colors")]
    #[test]
    fn web_colors_feature_disables_gamma_correction_and_selects_non_srgb_surface() {
        assert!(!gamma_correction_enabled());

        let selected = select_surface_format_for_iced_color_mode(&[
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureFormat::Bgra8Unorm,
        ]);

        assert_eq!(selected, Some(wgpu::TextureFormat::Bgra8Unorm));
    }
}
