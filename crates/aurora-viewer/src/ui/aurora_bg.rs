//! Animated aurora background (login, loading screens without a last view):
//! night sky gradient, twinkling stars, slowly moving aurora curtains in the
//! brand colors (additive light) and mountain silhouettes on the horizon.
//! Drawn with egui meshes: a few hundred vertices, no textures.

use crate::theme::Palette;
use egui::epaint::{Mesh, Vertex, WHITE_UV};
use egui::{Color32, Pos2, Rect, Shape};

fn v(pos: Pos2, color: Color32) -> Vertex {
    Vertex { pos, uv: WHITE_UV, color }
}

/// Additive light: premultiplied color with zero alpha adds to what is below.
fn glow(c: Color32, k: f32) -> Color32 {
    let k = k.clamp(0.0, 1.0);
    Color32::from_rgba_premultiplied((c.r() as f32 * k) as u8, (c.g() as f32 * k) as u8, (c.b() as f32 * k) as u8, 0)
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t) as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

/// Deterministic pseudo-random in 0..1.
fn hash(i: u32) -> f32 {
    let mut x = i.wrapping_mul(0x9E37_79B9) ^ 0x85EB_CA6B;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    (x & 0xFFFF) as f32 / 65535.0
}

/// Vertical gradient quad.
fn gradient(mesh: &mut Mesh, r: Rect, top: Color32, bottom: Color32) {
    let i = mesh.vertices.len() as u32;
    mesh.vertices.push(v(r.left_top(), top));
    mesh.vertices.push(v(r.right_top(), top));
    mesh.vertices.push(v(r.right_bottom(), bottom));
    mesh.vertices.push(v(r.left_bottom(), bottom));
    mesh.add_triangle(i, i + 1, i + 2);
    mesh.add_triangle(i, i + 2, i + 3);
}

/// HyperGrid Explorer's background picture (branding `assets/Background.png`),
/// in place of Aurora Viewer's drawn sky wherever that sky showed: the login
/// screen, the loading screens without a last view, and blurred behind the
/// glass of the login cards and the startup splash.
const PICTURE: &[u8] = include_bytes!("../../../../assets/branding/hypergrid-explorer-background.png");

/// The picture decoded off the main thread: sharp, and a small blurred copy
/// for the frosted glass (scaled up, it stays smooth).
struct Decoded {
    sharp: egui::ColorImage,
    soft: egui::ColorImage,
}

static DECODE_STARTED: std::sync::Once = std::sync::Once::new();
static DECODED: std::sync::Mutex<Option<Decoded>> = std::sync::Mutex::new(None);
/// The two textures (sharp, soft) and the picture's size, once uploaded. One
/// egui context per process (the viewer, or the splash helper): their handles
/// are kept for the life of the process.
static TEXTURES: std::sync::OnceLock<(egui::TextureId, egui::TextureId, egui::Vec2)> = std::sync::OnceLock::new();

fn decode() -> Option<Decoded> {
    let img = match image::load_from_memory(PICTURE) {
        Ok(img) => img.to_rgba8(),
        Err(e) => {
            log::warn!("background picture not decoded: {e}");
            return None;
        }
    };
    let (w, h) = img.dimensions();
    let small = image::imageops::resize(&img, (w / 6).max(1), (h / 6).max(1), image::imageops::FilterType::Triangle);
    let soft = image::imageops::blur(&small, 2.5);
    let color = |i: &image::RgbaImage| egui::ColorImage::from_rgba_unmultiplied([i.width() as usize, i.height() as usize], i.as_raw());
    Some(Decoded {
        sharp: color(&img),
        soft: color(&soft),
    })
}

/// Starts decoding the picture (once) and uploads it to `ctx` when it is
/// ready; true once it can be drawn. Until then the drawn sky shows.
pub fn prepare(ctx: &egui::Context) -> bool {
    if TEXTURES.get().is_some() {
        return true;
    }
    DECODE_STARTED.call_once(|| {
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let decoded = decode();
            if let Ok(mut slot) = DECODED.lock() {
                *slot = decoded;
            }
            ctx.request_repaint();
        });
    });
    let Some(decoded) = DECODED.lock().ok().and_then(|mut slot| slot.take()) else {
        return false;
    };
    let size = egui::vec2(decoded.sharp.size[0] as f32, decoded.sharp.size[1] as f32);
    let sharp = ctx.load_texture("hge-background", decoded.sharp, egui::TextureOptions::LINEAR);
    let soft = ctx.load_texture("hge-background-soft", decoded.soft, egui::TextureOptions::LINEAR);
    let ids = (sharp.id(), soft.id(), size);
    // the textures live as long as the process (one context per process)
    std::mem::forget(sharp);
    std::mem::forget(soft);
    let _ = TEXTURES.set(ids);
    true
}

/// Whether the background picture is on the GPU (or in the splash's painter).
pub fn picture_ready() -> bool {
    TEXTURES.get().is_some()
}

/// The part of a picture of `size` that covers `rect` without distortion
/// (centred, like a wallpaper set to "fill"): the picture follows every
/// window size, trimming only what the window's shape cannot show.
fn cover_uv(size: egui::Vec2, rect: Rect) -> Rect {
    let picture = size.x / size.y.max(1.0);
    let window = rect.width() / rect.height().max(1.0);
    if window > picture {
        let h = picture / window;
        Rect::from_min_max(Pos2::new(0.0, (1.0 - h) / 2.0), Pos2::new(1.0, (1.0 + h) / 2.0))
    } else {
        let w = window / picture;
        Rect::from_min_max(Pos2::new((1.0 - w) / 2.0, 0.0), Pos2::new((1.0 + w) / 2.0, 1.0))
    }
}

/// The background picture over `rect` (blurred when `soft`), or None before
/// it is ready.
fn picture_shapes(p: &Palette, rect: Rect, alpha: f32, soft: bool) -> Option<Vec<Shape>> {
    let &(sharp, blurred, size) = TEXTURES.get()?;
    let a = alpha.clamp(0.0, 1.0);
    let mut out = vec![Shape::image(
        if soft { blurred } else { sharp },
        rect,
        cover_uv(size, rect),
        crate::ui::widgets::PICTURE_TINT.gamma_multiply(a),
    )];
    // a light midnight-navy veil keeps the text over the picture readable
    let veil = p.navy.gamma_multiply(0.22 * a);
    out.push(Shape::rect_filled(rect, 0.0, veil));
    Some(out)
}

/// Paint the background over `rect`; `t` = time in seconds, `alpha` fades it.
pub fn paint(painter: &egui::Painter, p: &Palette, rect: Rect, t: f32, alpha: f32) {
    prepare(painter.ctx());
    painter.extend(shapes(p, rect, t, alpha, false));
}

/// The background as a frosted pane shows it: the same sky at the same
/// moment, out of focus (no stars, the rays of the curtains melted together,
/// their edges and the ridgelines feathered). The cards of the login screen
/// draw it under their tint: the blur of the in-world windows only knows the
/// 3D scene, and there is none behind them.
pub fn frosted(p: &Palette, rect: Rect, t: f32) -> Vec<Shape> {
    shapes(p, rect, t, 1.0, true)
}

/// Shared login glass: a softened sky under the skin's lighter panel tint.
pub const LOGIN_TINT: f32 = 0.56;

/// The sky shown through a login card, with its own lights and panel tint.
pub fn frosted_pane(p: &Palette, screen: Rect, t: f32, pane: Rect, lights: Vec<Shape>) -> Vec<Shape> {
    let mut out = frosted(p, screen, t);
    out.extend(lights);
    let alpha = (f32::from(p.panel_alpha) * LOGIN_TINT) as u8;
    out.push(Shape::rect_filled(
        pane,
        0.0,
        Color32::from_rgba_unmultiplied(p.panel.r(), p.panel.g(), p.panel.b(), alpha),
    ));
    out
}

fn shapes(p: &Palette, rect: Rect, t: f32, alpha: f32, soft: bool) -> Vec<Shape> {
    // HyperGrid Explorer: the background picture once it is ready; Aurora's
    // drawn sky below only shows for the first frames while it decodes
    if let Some(picture) = picture_shapes(p, rect, alpha, soft) {
        return picture;
    }
    let mut out = Vec::new();
    let a = alpha.clamp(0.0, 1.0);
    let (w, h) = (rect.width(), rect.height());
    let fade = |c: Color32| c.gamma_multiply(a);

    // ---- sky
    let mut sky = Mesh::default();
    let night = p.navy;
    let deep = mix(p.navy, Color32::BLACK, 0.45);
    let horizon = mix(p.navy, p.indigo, 0.22);
    let mid = Rect::from_min_max(rect.min, Pos2::new(rect.right(), rect.top() + h * 0.55));
    gradient(&mut sky, mid, fade(deep), fade(night));
    gradient(
        &mut sky,
        Rect::from_min_max(mid.left_bottom(), rect.max),
        fade(night),
        fade(horizon),
    );
    out.push(Shape::mesh(sky));

    // ---- stars (upper sky), twinkling
    let stars = if soft { 0 } else { 170u32 };
    for i in 0..stars {
        let x = rect.left() + hash(i * 3) * w;
        let y = rect.top() + hash(i * 3 + 1).powf(1.6) * h * 0.72;
        let s = 0.5 + hash(i * 3 + 2) * 1.2;
        let tw = 0.55 + 0.45 * (t * (0.6 + hash(i + 977) * 1.8) + hash(i + 31) * std::f32::consts::TAU).sin();
        let c = if i % 9 == 0 {
            p.teal
        } else if i % 7 == 0 {
            p.violet_pale
        } else {
            Color32::WHITE
        };
        out.push(Shape::circle_filled(Pos2::new(x, y), s, glow(c, 0.55 * tw * a)));
    }

    // ---- aurora curtains (additive)
    let mut aur = Mesh::default();
    let curtains: [(f32, f32, Color32, Color32, f32, f32); 4] = [
        // base y, height, top color, bottom color, speed, phase
        (0.30, 0.26, p.violet, p.teal, 0.050, 0.0),
        (0.38, 0.20, p.indigo, p.violet, -0.035, 1.7),
        (0.24, 0.16, p.teal, p.indigo, 0.028, 3.1),
        (0.46, 0.14, p.violet_light, p.teal, -0.020, 4.6),
    ];
    const N: usize = 120;
    for (k, (base, height, top_c, low_c, speed, phase)) in curtains.iter().enumerate() {
        let start = aur.vertices.len() as u32;
        for s in 0..=N {
            let u = s as f32 / N as f32;
            let x = rect.left() + u * w;
            let wave = (u * 5.3 + t * speed * 6.0 + phase).sin() * 0.045
                + (u * 11.7 - t * speed * 9.0 + phase * 2.0).sin() * 0.018
                + (u * 2.1 + t * 0.03 + phase).sin() * 0.03;
            let y = rect.top() + (base + wave) * h;
            // vertical "rays": brightness varies along the curtain
            let rays = if soft {
                0.74
            } else {
                0.55 + 0.45 * ((u * 38.0 + t * 0.35 + phase * 3.0).sin() * 0.5 + 0.5).powf(1.5)
            };
            let ends = (u * std::f32::consts::PI).sin().powf(0.6); // dims at the edges
            let breathe = 0.8 + 0.2 * (t * 0.25 + phase + u * 3.0).sin();
            let strength = (0.34 - k as f32 * 0.05) * rays * ends * breathe * a;
            let tint = mix(*top_c, *low_c, 0.5 + 0.5 * (u * 3.0 + phase + t * 0.05).sin());
            let ht = height * h * (0.75 + 0.25 * (u * 7.0 + phase + t * 0.1).sin());
            // out of focus, the lower edge spreads and loses its brightness
            let (below, peak) = if soft { (0.11, 0.8) } else { (0.035, 1.0) };
            // top (faint, tall), bright lower edge, quick fade below
            aur.vertices.push(v(Pos2::new(x, y - ht), glow(tint, 0.0)));
            aur.vertices.push(v(Pos2::new(x, y - ht * 0.35), glow(tint, strength * 0.55)));
            aur.vertices.push(v(Pos2::new(x, y), glow(*low_c, strength * peak)));
            aur.vertices.push(v(Pos2::new(x, y + h * below), glow(*low_c, 0.0)));
        }
        for s in 0..N as u32 {
            let i = start + s * 4;
            for r in 0..3 {
                let (a0, a1, b0, b1) = (i + r, i + r + 1, i + 4 + r, i + 4 + r + 1);
                aur.add_triangle(a0, b0, b1);
                aur.add_triangle(a0, b1, a1);
            }
        }
    }
    out.push(Shape::mesh(aur));

    // ---- horizon glow
    let mut haze = Mesh::default();
    let hz = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + h * 0.62),
        Pos2::new(rect.right(), rect.top() + h * 0.86),
    );
    let i = haze.vertices.len() as u32;
    haze.vertices.push(v(hz.left_top(), glow(p.violet, 0.0)));
    haze.vertices.push(v(hz.right_top(), glow(p.violet, 0.0)));
    haze.vertices.push(v(hz.right_bottom(), glow(p.violet, 0.10 * a)));
    haze.vertices.push(v(hz.left_bottom(), glow(p.indigo, 0.10 * a)));
    haze.add_triangle(i, i + 1, i + 2);
    haze.add_triangle(i, i + 2, i + 3);
    out.push(Shape::mesh(haze));

    // ---- mountains: two ridgelines (far lighter, near darker)
    let mut ridge = |seed: f32, base: f32, amp: f32, color: Color32| {
        let mut m = Mesh::default();
        const R: usize = 96;
        // out of focus, the crest fades in over this height
        let feather = if soft { h * 0.035 } else { 0.0 };
        for s in 0..=R {
            let u = s as f32 / R as f32;
            let x = rect.left() + u * w;
            let n = (u * 3.1 + seed).sin() * 0.5
                + (u * 7.3 + seed * 2.0).sin() * 0.28
                + (u * 17.0 + seed * 3.0).sin() * 0.12
                + ((u * 41.0 + seed).sin() * 0.06);
            let y = rect.top() + (base - amp * (0.5 + 0.5 * n)) * h;
            m.vertices.push(v(Pos2::new(x, y - feather), Color32::TRANSPARENT));
            m.vertices.push(v(Pos2::new(x, y + feather), color));
            m.vertices.push(v(Pos2::new(x, rect.bottom().max(y + feather)), color));
        }
        for s in 0..R as u32 {
            let i = s * 3;
            for r in 0..2 {
                m.add_triangle(i + r, i + 3 + r, i + r + 1);
                m.add_triangle(i + r + 1, i + 3 + r, i + 4 + r);
            }
        }
        out.push(Shape::mesh(m));
    };
    ridge(0.7, 0.86, 0.16, fade(mix(p.navy, p.indigo, 0.16)));
    ridge(2.9, 0.93, 0.13, fade(mix(p.navy, Color32::BLACK, 0.35)));
    out
}

/// The background colour of HyperGrid Explorer's approved horizontal logo (its
/// edges read #021322 to #031424): the logo is an opaque picture, so the
/// screens lay it on a pool of its own colour instead of cutting it out.
pub const LOGO_BACKDROP: Color32 = Color32::from_rgb(0x02, 0x13, 0x22);

/// HyperGrid Explorer: a pool of [`LOGO_BACKDROP`] under the logo's `rect`,
/// opaque over the picture and fading to nothing `feather` points beyond its
/// edges, with rounded outer corners. The logo's rectangle then melts into
/// whatever gradient or sky is behind it, and the picture itself stays whole
/// (never cropped, recoloured or masked).
pub fn logo_pool_shape(rect: Rect, feather: f32) -> Shape {
    const STEPS: usize = 10;
    let solid = LOGO_BACKDROP;
    let clear = Color32::TRANSPARENT;
    let mut m = Mesh::default();
    // the inner rectangle's corners, clockwise from top left, each with the
    // quarter circle its outer corner sweeps
    let corners = [
        (rect.left_top(), std::f32::consts::PI),
        (rect.right_top(), 1.5 * std::f32::consts::PI),
        (rect.right_bottom(), 0.0),
        (rect.left_bottom(), 0.5 * std::f32::consts::PI),
    ];
    for (corner, start) in corners {
        for k in 0..=STEPS {
            let a = start + k as f32 / STEPS as f32 * std::f32::consts::FRAC_PI_2;
            m.vertices.push(v(corner, solid));
            m.vertices.push(v(corner + egui::vec2(a.cos(), a.sin()) * feather, clear));
        }
    }
    let n = (m.vertices.len() / 2) as u32;
    for i in 0..n {
        let j = (i + 1) % n;
        let (ai, ao, bi, bo) = (2 * i, 2 * i + 1, 2 * j, 2 * j + 1);
        m.add_triangle(ai, ao, bo);
        m.add_triangle(ai, bo, bi);
    }
    // the opaque middle
    let base = m.vertices.len() as u32;
    for c in [rect.left_top(), rect.right_top(), rect.right_bottom(), rect.left_bottom()] {
        m.vertices.push(v(c, solid));
    }
    m.add_triangle(base, base + 1, base + 2);
    m.add_triangle(base, base + 2, base + 3);
    Shape::mesh(m)
}

/// Soft radial light (additive): `intensity` at the center fading to zero
/// at `radius`, smooth (no visible rings).
pub fn radial_glow(painter: &egui::Painter, center: Pos2, radius: f32, color: Color32, intensity: f32) {
    painter.add(radial_glow_shape(center, radius, color, intensity));
}

/// `radial_glow` as a shape, for a caller that orders its own shapes.
pub fn radial_glow_shape(center: Pos2, radius: f32, color: Color32, intensity: f32) -> Shape {
    let mut m = Mesh::default();
    const SEG: usize = 48;
    const RINGS: [(f32, f32); 4] = [(0.0, 1.0), (0.35, 0.62), (0.7, 0.22), (1.0, 0.0)];
    for (r, k) in RINGS {
        for s in 0..SEG {
            let a = s as f32 / SEG as f32 * std::f32::consts::TAU;
            let pos = center + egui::vec2(a.cos(), a.sin()) * radius * r;
            m.vertices.push(v(pos, glow(color, intensity * k)));
        }
    }
    for ring in 0..RINGS.len() as u32 - 1 {
        for s in 0..SEG as u32 {
            let (a0, a1) = (ring * SEG as u32 + s, ring * SEG as u32 + (s + 1) % SEG as u32);
            let (b0, b1) = (a0 + SEG as u32, a1 + SEG as u32);
            m.add_triangle(a0, b0, b1);
            m.add_triangle(a0, b1, a1);
        }
    }
    Shape::mesh(m)
}
