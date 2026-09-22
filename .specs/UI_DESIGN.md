# UI & UX Specifications

## Theme ("Sabor & Raíz")
- Background: `#0D0D0D`
- Cards/Containers: `#141414`
- Accent: `#DEFF9A` (Lime Green)

## POS Order View Layout Rules
- Viewport: Fixed height non-scrolling station (`100vh`).
- Left Panel & Top Bars: Order cart, subtotals, category selectors, and search bars MUST remain strictly fixed in place.
- Scrollable Container: ONLY the product cards grid within the menu section is vertically scrollable.
- Aspect Ratios: Product image containers must enforce `aspect-square` / `aspect-[4/3]` with `object-cover` to prevent distortion on tablet viewports (768px–1024px).

## Interaction Rules
- **No Browser Alerts**: `window.alert()` and `window.confirm()` are forbidden. Use the global `ConfirmationModal` component.