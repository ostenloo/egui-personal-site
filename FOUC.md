# FOUC Investigation: egui Personal Site Canvas Resize

## Problem
On page refresh, the canvas briefly appears as a narrow vertical strip on the left side of the viewport with black bars on both sides. Content inside the canvas is shifted right. Then after ~1 frame, the canvas expands to fill the full viewport and content centers correctly.

## Screenshots
- **FOUC state**: [Screenshot 2026-05-12 at 1.28.06 AM](./Screenshot%202026-05-12%20at%201.28.06%20AM.png) — Canvas is ~200px wide, positioned left, black bars on left and right
- **Final state**: [Screenshot 2026-05-12 at 1.27.37 AM](./Screenshot%202026-05-12%20at%201.27.37%20AM.png) — Canvas fills full viewport, content centered

## Root Cause
eframe's `resize_canvas_to_screen_size()` is called on every frame in `app_runner.rs:187`:

```rust
pub fn logic(&mut self) {
    super::resize_canvas_to_screen_size(self.canvas(), self.web_options.max_size_points);
    let canvas_size = super::canvas_size_in_points(self.canvas(), self.egui_ctx());
    // ...
}
```

This function (in `eframe/src/web/mod.rs:127`) reads the parent element's dimensions:

```rust
let parent = canvas.parent_element()?;
let parent_size_points = Vec2 {
    x: parent.client_width() as f32,
    y: parent.client_height() as f32,
};
let canvas_size_pixels = pixels_per_point * parent_size_points.min(max_size_points);
canvas.set_width((canvas_size_pixels.x / pixels_per_point) as u32);
canvas.set_height((canvas_size_pixels.y / pixels_per_point) as u32);
```

On the first frame, `parent.client_width()` may return an incorrect value (small or 0) because the CSS layout hasn't been fully computed yet. This causes the canvas to render at the wrong size for one frame before the next frame reads the correct dimensions.

## Key Insight: Two Independent Dimensions

eframe's `resize_canvas_to_screen_size()` (eframe 0.27, `src/web/mod.rs:127`) sets **both**:
1. `canvas.style.width/height` — CSS display size (inline style, high specificity)
2. `canvas.width/height` — WebGL rendering buffer size

The visual FOUC (black bars on sides) is caused by #1: the CSS display size being set to a wrong pixel value on the first frame, combined with the canvas being absolutely positioned / centered. This is a **CSS layout flash**, not a rendering resolution issue.

eframe's `text_agent.rs` also explicitly notes `// Canvas is translated 50% horizontally in html`, confirming the library expects `position: absolute; left: 50%; transform: translate(-50%, 0%)` as the canvas layout.

## Attempted Fixes

### 1. Pre-size canvas via JavaScript
Set canvas `width`/`height` attributes before egui loads using `document.write()`.
**Result**: Failed — egui overrides the attributes on the first frame via `resize_canvas_to_screen_size()`.

### 2. CSS `!important` override
Applied `width: 100vw !important; height: 100vh !important;` to the canvas.
**Result**: Failed — HTML canvas attribute `width`/`height` controls the rendering buffer size independently of CSS display size. egui sets the attributes, so the buffer is sized wrong even if CSS displays it full-width.

### 3. Opacity delay
Set `opacity: 0` on canvas, transition to `opacity: 1` after 2 `requestAnimationFrame` cycles (~33ms).
**Result**: Works visually but is a hack — hides the problem rather than fixing it.

### 4. Flexbox layout
Changed `body { display: flex }` and `canvas { flex: 1 }` to let CSS control canvas size natively. Removed `position: fixed`.
**Result**: Cleaner layout but FOUC persists — egui still resizes the canvas attributes based on parent `client_width()`.

### 5. Position absolute with centering
Original approach: `position: absolute; left: 50%; transform: translate(-50%, 0%);`
**Result**: FOUC still occurs because canvas attribute width changes, affecting the transform center point.

## Why It's Hard to Fix
- eframe unconditionally calls `resize_canvas_to_screen_size()` on every frame
- The function reads `parent.client_width()` which depends on CSS layout being complete
- CSS layout completion timing is not predictable in the browser
- Canvas attribute dimensions and CSS display dimensions are independent — setting one doesn't prevent egui from overriding the other

## Potential Proper Fixes
1. **Patch eframe**: Modify `resize_canvas_to_screen_size()` to skip resizing if parent dimensions are 0 or clearly incorrect, and defer to the next frame.
2. **Force synchronous layout**: Use `getBoundingClientRect()` or `offsetWidth` in a blocking manner before egui initializes (not possible from Rust/WASM without JS interop).
3. **Custom WebRunner**: Fork eframe's web runner and add a minimum delay before the first resize.
4. **CSS containment**: Use `contain: layout` on the body to force the browser to compute layout synchronously (experimental, browser-dependent).

### 6. `position: fixed` wrapper div
Wrapped canvas in `<div id="canvas-wrapper" style="position: fixed; width: 100%; height: 100%">` so `canvas.parent_element().clientWidth` reads from a viewport-relative fixed element rather than body.
**Result**: Failed — FOUC persists. Either `clientWidth` is still unreliable at WASM init time, or the root cause is not `clientWidth` at all.

### 7. JS MutationObserver + opacity hide
Hid canvas with `opacity: 0`, watched for eframe to set `canvas.style.width` via `MutationObserver`, revealed canvas once width ≥ 90% of `window.innerWidth`.
**Result**: Failed — FOUC persists. The observer either fires too late or the FOUC is not originating from the canvas element.

### 8. CSS `width: 100vw !important`
Used `!important` on canvas `width`/`height` to beat eframe's inline style in the CSS cascade, so the canvas always *displays* at full viewport regardless of what eframe writes.
**Result**: Failed — FOUC persists. The display size may be forced correct, but the rendering content (egui layout at wrong `screen_rect`) still causes a visible jump.

## Current State
Unresolved. FOUC persists on refresh. All JS-side workarounds have failed. The issue is in eframe's canvas resize logic (`resize_canvas_to_screen_size` in `eframe/src/web/mod.rs`) and likely requires a library-level fix: either patching eframe to skip/defer the first resize when dimensions appear wrong, or forking the WebRunner to add a minimum-size guard before the first render.
