---
version: "2.0"
name: Inditex Minimal Editorial
description: A stark, fashion-forward corporate design system with dual-theme support, monochrome restraint, large imagery, and sparse, precise typography.
colors:
  light:
    primary: "#000000"
    secondary: "#121212"
    tertiary: "#374151"
    neutral: "#FFFFFF"
    surface: "#FFFFFF"
    on-surface: "#000000"
    error: "#B91C1C"
    hover-row: "#f5f5f5"
    disabled-border: "#e5e7eb"
    body-background: "#FFFFFF"
  dark:
    primary: "#FFFFFF"
    secondary: "#1a1a1a"
    tertiary: "#6B7280"
    neutral: "#000000"
    surface: "#121212"
    on-surface: "#FFFFFF"
    error: "#B91C1C"
    hover-row: "#1f1f1f"
    disabled-border: "#374151"
    body-background: "#000000"
typography:
  headline-display:
    fontFamily: "abc_monument_grotesk_Lt"
    fontSize: "38px"
    fontWeight: 700
    lineHeight: "46px"
    letterSpacing: "0px"
  headline-lg:
    fontFamily: "abc_monument_grotesk_Lt"
    fontSize: "30px"
    fontWeight: 700
    lineHeight: "36px"
    letterSpacing: "0px"
  headline-md:
    fontFamily: "abc_monument_grotesk_Lt"
    fontSize: "23px"
    fontWeight: 100
    lineHeight: "36px"
    letterSpacing: "0px"
  headline-sm:
    fontFamily: "abc_monument_grotesk_Lt"
    fontSize: "18px"
    fontWeight: 100
    lineHeight: "33px"
    letterSpacing: "0px"
  body-lg:
    fontFamily: "abc_monument_grotesk_Lt"
    fontSize: "16px"
    fontWeight: 400
    lineHeight: "24px"
    letterSpacing: "0.02em"
  body-md:
    fontFamily: "abc_monument_grotesk_Lt"
    fontSize: "14px"
    fontWeight: 100
    lineHeight: "20px"
    letterSpacing: "0.04em"
  body-sm:
    fontFamily: "abc_monument_grotesk_Lt"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: "18px"
    letterSpacing: "0.04em"
  label-lg:
    fontFamily: "abc_monument_grotesk_Rg"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: "16px"
    letterSpacing: "0.04em"
  label-md:
    fontFamily: "abc_monument_grotesk_Rg"
    fontSize: "10px"
    fontWeight: 400
    lineHeight: "14px"
    letterSpacing: "0.04em"
  label-sm:
    fontFamily: "abc_monument_grotesk_Rg"
    fontSize: "10px"
    fontWeight: 400
    lineHeight: "12px"
    letterSpacing: "0.04em"
spacing:
  xs: "6px"
  sm: "16px"
  md: "56px"
  lg: "82px"
  xl: "160px"
  gutter: "24px"
  margin: "24px"
rounded:
  none: "0px"
  sm: "4px"
  md: "8px"
  lg: "12px"
  xl: "16px"
  full: "9999px"
layout:
  root-max-width: "1500px"
  navbar-height: "50px"
  root-margin-top: "70px"
  content-margin: "24px"
  breakpoint-mobile: "800px"
transitions:
  default: "opacity 0.2s ease"
  border: "border-color 0.2s ease"
  hamburger: "transform 0.4s ease-in-out"
  menu-slide: "transform 0.5s ease-in-out"
  button-click: "opacity 0.3s ease"
components:
  navbar:
    height: "50px"
    background: "{colors.surface}"
    borderBottom: "1px solid {colors.tertiary}"
    position: "fixed"
    zIndex: 10
    linkFont: "{typography.label-lg}"
    activeDecoration: "underline"
    activeUnderlineOffset: "4px"
    hoverOpacity: 0.6
  button-primary:
    backgroundColor: "transparent"
    textColor: "{colors.on-surface}"
    typography: "{typography.label-md}"
    border: "1px solid {colors.primary}"
    rounded: "{rounded.sm}"
    padding: "0 24px"
    height: "40px"
    hoverOpacity: 0.7
    clickedOpacity: 0.5
  button-pagination:
    backgroundColor: "transparent"
    textColor: "{colors.on-surface}"
    border: "1px solid {colors.tertiary}"
    rounded: "{rounded.none}"
    padding: "4px 8px"
    minWidth: "40px"
    hoverOpacity: 0.7
    disabledOpacity: 0.5
  card:
    backgroundColor: "{colors.surface}"
    darkBackgroundColor: "#1a1a1a"
    textColor: "{colors.on-surface}"
    border: "1px solid {colors.tertiary}"
    rounded: "{rounded.none}"
    padding: "{spacing.sm}"
    margin: "10px"
    textAlign: "center"
  table-legend:
    fontSize: "{typography.body-sm.fontSize}"
    maxHeight: "600px"
    overflow: "auto"
    headerBorder: "2px solid {colors.tertiary}"
    headerFont: "{typography.label-lg}"
    headerSticky: true
    cellPadding: "1px 6px"
    cellColor: "{colors.tertiary}"
  table-report:
    fontSize: "{typography.body-sm.fontSize}"
    headerBorder: "1px solid {colors.tertiary}"
    headerFont: "{typography.label-lg}"
    headerPadding: "5px 5px"
    cellPadding: "2px 5px"
    overflowMaxWidth: "300px"
  input:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.on-surface}"
    typography: "{typography.body-md}"
    border: "1px solid {colors.tertiary}"
    rounded: "{rounded.sm}"
    padding: "12px"
    focusBorderColor: "{colors.primary}"
  message:
    padding: "{spacing.sm}"
    borderLeft: "2px solid {colors.tertiary}"
    width: "max-content"
  notice:
    backgroundColor: "{colors.surface}"
    border: "1px solid {colors.tertiary}"
    padding: "20px"
    width: "80%"
  error:
    extends: "notice"
    borderLeft: "3px solid {colors.error}"
  pagination:
    display: "flex"
    gap: "{spacing.xs}"
    fontSize: "{typography.body-sm.fontSize}"
    marginTop: "{spacing.sm}"
  loading:
    margin: "{spacing.md}"
    padding: "{spacing.md}"
    fontStyle: "italic"
    color: "{colors.tertiary}"
---

# Inditex Minimal Editorial v2

## Overview

A restrained, premium, and highly corporate design system with a strong fashion/editorial sensibility. The interface is spacious, using minimal chrome to keep attention on content. It speaks with a confident, low-noise tone that feels polished rather than playful.

This version documents the complete implementation including dual-theme support, all UI components, the layout system, and responsive behavior.

---

## Theme System

### Activation

Themes are toggled via a `data-theme` attribute on the root HTML element:

```html
<!-- Light (default) -->
<html>

<!-- Dark -->
<html data-theme="dark">
```

### Token Mapping

Only semantic color tokens change between themes. Typography, spacing, and border-radius remain constant.

| Token | Light | Dark |
|-------|-------|------|
| `--color-primary` | `#000000` | `#000000` (unchanged) |
| `--color-secondary` | `#121212` | `#121212` (unchanged) |
| `--color-tertiary` | `#374151` | `#6B7280` |
| `--color-neutral` | `#FFFFFF` | `#000000` |
| `--color-surface` | `#FFFFFF` | `#121212` |
| `--color-on-surface` | `#000000` | `#FFFFFF` |
| `--body-background-color` | `#FFFFFF` | `#000000` |
| `--root-background-color` | `#FFFFFF` | `#121212` |
| `--root-foreground-color` | `#000000` | `#FFFFFF` |

### Dark Mode Component Overrides

Some components use hardcoded dark values instead of tokens:
- Cards (`div.report`): `background: #1a1a1a`
- Table hover rows: `background: #1f1f1f`

### Chart.js Dark Mode

Charts must adapt dynamically to the active theme:
- Legend text color: use `--color-on-surface`
- Grid line color: use `--color-tertiary`
- Title/axis text: use `--color-on-surface`
- Tooltip background: use `--color-secondary`

---

## Colors

### Philosophy

The palette is intentionally monochromatic. No bright accent colors, no gradients, no decorative hues. Sophistication comes from restraint. The only chromatic color is `error` red, used sparingly for validation states.

### Light Mode

- **Primary (`#000000`)**: Key text, logo, navigation, strongest contrast moments.
- **Secondary (`#121212`)**: Near-black for deep panels and dark UI foundations.
- **Tertiary (`#374151`)**: Muted graphite for borders, separators, secondary text, and subtle structure.
- **Neutral (`#FFFFFF`)**: White for overlays, hero text on dark imagery.
- **Surface (`#FFFFFF`)**: Clean base for components and light content areas.
- **On-surface (`#000000`)**: Default readable text on light surfaces.
- **Error (`#B91C1C`)**: Restrained red for validation and destructive states only.

### Dark Mode

- **Tertiary (`#6B7280`)**: Lighter graphite to maintain contrast on dark surfaces.
- **Neutral (`#000000`)**: Inverted for overlay contexts.
- **Surface (`#121212`)**: Dark base for components.
- **On-surface (`#FFFFFF`)**: Light text for readability on dark backgrounds.
- **Body background (`#000000`)**: Pure black canvas behind the surface.

---

## Typography

### Font Families

- **Body**: `abc_monument_grotesk_Lt`, with fallbacks `Inter`, `Helvetica Neue`, `sans-serif`
- **Labels**: `abc_monument_grotesk_Rg`, with same fallbacks

Fonts are loaded via `@font-face` from `.woff2` files in `src/assets/fonts/`.

### Scale Usage

| Scale | Size | Weight | Use |
|-------|------|--------|-----|
| `headline-display` | 38px | 700 | Page titles, hero text |
| `headline-lg` | 30px | 700 | Major section headers |
| `headline-md` | 23px | 100 | Subsection headers |
| `headline-sm` | 18px | 100 | Card headings (`div.report h3`) |
| `body-lg` | 16px | 400 | Mobile nav items, emphasized body |
| `body-md` | 14px | 100 | Default body text, form inputs |
| `body-sm` | 12px | 400 | Table cells, pagination, small text |
| `label-lg` | 12px | 400 | Table headers, navbar links, totals |
| `label-md` | 10px | 400 | Buttons, metadata |
| `label-sm` | 10px | 400 | Chips, smallest tokens |

### Letter Spacing

Body and label scales use positive tracking (`0.02em`-`0.04em`) for clarity at small sizes. Headlines use `0px`.

---

## Layout & Grid

### Page Structure

```
┌─────────────────────────────────────────┐
│ Navbar (fixed, 50px, z-index:10)        │
├─────────────────────────────────────────┤
│                                         │
│ div#root (margin-top: 70px)             │
│ ┌─────────────────────────────────────┐ │
│ │ div.contents (margin: 0 24px)       │ │
│ │ ┌─────────────────────────────────┐ │ │
│ │ │ div.links (section nav)         │ │ │
│ │ ├─────────────────────────────────┤ │ │
│ │ │ div.flex (card grid)            │ │ │
│ │ │  ┌──────┐ ┌──────┐ ┌──────┐    │ │ │
│ │ │  │ card │ │ card │ │ card │    │ │ │
│ │ │  └──────┘ └──────┘ └──────┘    │ │ │
│ │ └─────────────────────────────────┘ │ │
│ └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

### Layout Mechanism

- **No CSS Grid**: Layout uses `display: flex; flex-wrap: wrap` via `div.flex`
- **Card sizing**: Inline styles set width per card (e.g., 300px for doughnuts, 700px for line charts)
- **Content is fluid**: `--root-max-width: 1500px` is defined but not enforced as a constraint
- **Content margins**: `div.contents` applies `0 24px` horizontal margin

### Spacing Scale Usage

| Token | Value | Where Used |
|-------|-------|-----------|
| `xs` (6px) | Pagination gap, table cell padding, login cell padding |
| `sm` (16px) | Card padding, message padding, pagination margin-top |
| `md` (56px) | Loading state margin/padding |
| `lg` (82px) | Login form margin-top |
| `gutter` (24px) | Button horizontal padding, nav link spacing |
| `margin` (24px) | Content wrapper horizontal margins |

---

## Responsive Design

### Breakpoint

Single breakpoint: `@media (max-width: 800px)`

### Mobile Changes

| Element | Desktop | Mobile (<=800px) |
|---------|---------|-------------------|
| Navbar | Horizontal flex menu | Hamburger + slide-out drawer |
| Navbar opacity | 1.0 | 0.97 |
| Menu items | Row, label-lg font | Column, body-lg font, 1.5rem spacing |
| Menu width | Auto | 300px max, full viewport height |
| Menu animation | None | Slide from left (0.5s ease-in-out) |

### What Does NOT Change at Mobile

- Card layout (no stacking or resizing)
- Table layout (no responsive adaptation)
- Form layout
- Pagination
- Typography scale

---

## Components

### Navbar

Fixed top navigation with hamburger mobile variant.

**Desktop:**
- Position: fixed, top: 0, full width, 50px height, z-index: 10
- Background: `--color-surface`
- Border: bottom `1px solid --color-tertiary`
- Items: horizontal flex row, `--spacing-gutter` gap between items
- Links: font-label, label-lg scale, no decoration
- Active: underline with `text-underline-offset: 4px`
- Hover: opacity 0.6, no underline

**Mobile (<=800px):**
- Hamburger: 3 lines (35x23px), positioned top:17px left:20px
- Toggle: hidden checkbox pattern (no JavaScript required)
- Drawer: slides from left, 300px max-width, full height, right border
- Line animation: line1 rotates 35deg, line2 scales to 0, line3 rotates -35deg
- Transition: `transform 0.5s ease-in-out`

### Buttons

**Submit / Link Buttons:**
- Background: transparent
- Border: `1px solid --color-primary`
- Border-radius: `--rounded-sm` (4px)
- Height: 40px
- Padding: `0 --spacing-gutter`
- Font: label-md scale
- Hover: opacity 0.7
- Clicked: opacity 0.5 (0.3s transition)
- Selected (link variant): `outline: 2px solid --color-primary; outline-offset: -3px`

**Pagination Buttons:**
- Border: `1px solid --color-tertiary`
- Border-radius: 0
- Min-width: 40px
- Padding: 4px 8px
- Disabled: opacity 0.5, `--color-tertiary` text, `#e5e7eb` border

### Tables

**Legend Table (`table.legend`):**
- Display: block (enables scrolling)
- Width: 100%, max-height: 600px, overflow-y: auto
- Font-size: body-sm (12px)
- Header: `border-bottom: 2px solid --color-tertiary`, position sticky top:0
- Footer: `border-top: 2px solid --color-tertiary`, position sticky bottom:0
- Header/footer background: `--body-background-color` (ensures sticky readability)
- Header cells: font-label, label-lg, padding `1px 2px`, nowrap
- First th column: `width: 100%` (stretches to fill)
- Body cells: padding `1px 6px`, nowrap, color `--color-tertiary`
- Row hover: `#f5f5f5` (light) / `#1f1f1f` (dark)

**Report Table (`table.report`):**
- Font-size: body-sm (12px)
- Header: `border-bottom: 1px solid --color-tertiary`
- Header cells: label-lg font, padding `5px 5px`
- Body cells: padding `2px 5px`, nowrap
- Overflow cells (`.overflow`): max-width 300px, `text-overflow: ellipsis`
- Footer: `border-top: 1px solid --color-tertiary`
- Row hover: same as legend
- Column filters (`input.tfilter`): width 100%, 1px tertiary border, no radius, padding `1px 5px`

**Login Table (`table#login`):**
- Width: 250px, centered (margin auto)
- Margin-top: `--spacing-lg` (82px)
- Labels: font-label, label-lg scale
- Cell padding: `--spacing-xs` (6px)
- All inputs/buttons: width 100%, border-radius 0

### Cards (`div.report`)

- Text-align: center
- Padding: `--spacing-sm` (16px)
- Background: `--color-surface` (light) / `#1a1a1a` (dark)
- Border: `1px solid --color-tertiary`
- Border-radius: 0 (sharp corners always)
- Margin: 10px
- Position: relative
- Heading (h3): font-body, headline-sm, `--color-on-surface`
- Total label (p.total): font-label, label-lg, text-align left

### Messages

**Inline Message (`div.message`):**
- Color: `--color-on-surface`
- Padding: `--spacing-sm`
- Margin: 20px auto
- Width: max-content
- Border-left: `2px solid --color-tertiary`

**Notice (`div.notice`):**
- Background: `--color-surface`
- Margin: 5% auto
- Padding: 20px
- Border: `1px solid --color-tertiary`
- Width: 80%

**Error (`div.error`):**
- Same as notice plus `border-left: 3px solid --color-error`

### Form Controls

- Font: body-md scale
- Padding: 12px
- Border: `1px solid --color-tertiary`
- Border-radius: `--rounded-sm` (4px)
- Background: `--color-surface`
- Color: `--color-on-surface`
- Focus: `border-color: --color-primary`, no outline
- Transition: `border-color 0.2s ease`
- Placeholder: `--color-tertiary`, weight 400
- Select: custom SVG chevron arrow, native appearance removed
- Autofill: white box-shadow inset to prevent browser styling

### Links Navigation (`div.links`)

- Padding: `0 --spacing-margin`
- Links: inline-block, padding `10px --spacing-gutter`
- Mini variant: padding `5px 10px`
- Active: underline + font-weight 700
- Hover: underline
- Transition: opacity 0.2s ease

### Pagination (`div.pagination`)

- Display: flex, align-items center
- Font-size: body-sm
- Gap: `--spacing-xs` (6px)
- Margin-top: `--spacing-sm` (16px)
- Page number input: width 65px
- Page info span: margin `auto 5px`

### Loading State

- Margin: `--spacing-md`
- Padding: `--spacing-md`
- Font-style: italic
- Color: `--color-tertiary`

---

## Charts (Chart.js)

### Chart Types

| Type | Use Case | Container Size |
|------|----------|---------------|
| Doughnut | Distribution (environments, OS, hardware) | ~300px wide, 550px tall |
| Line | Trends over time (history charts) | ~700px wide, 500px tall |
| Bar | Comparisons (updated counts, facts by env) | ~700px wide, 500px tall |

### Container Pattern

Charts always live inside `div.report` cards with inline dimension styles:

```html
<div class="report" style="height: 550px; width: 300px;">
  <h3>Chart Title</h3>
  <Doughnut ... />
  <table class="legend">...</table>
</div>
```

### Configuration Defaults

- `Chart.defaults.font.size = 13`
- `responsive: true`
- `maintainAspectRatio: false`
- Plugins: Title, Tooltip, Legend, Colors

### Color Palette

For multi-series charts, use this sequence that works in both themes:

1. `#36a2eb` (blue)
2. `#ff6384` (coral)
3. `#ffce56` (amber)
4. `#4bc0c0` (teal)
5. `#9966ff` (violet)
6. `#ff9f40` (orange)
7. `#c9cbcf` (silver)
8. `#dddddd` (light gray)

For binary/ternary charts (e.g., updated/non-updated): `#36a2eb`, `#ff6384`, `#dddddd`

### Legend Positioning

- Doughnut: bottom, align start (legend doubles as data table via `table.legend`)
- Line/Bar: top or hidden (data labels on axes provide context)

### Theme Adaptation

```javascript
const isDark = document.documentElement.dataset.theme === 'dark';
const textColor = isDark ? '#FFFFFF' : '#000000';
const gridColor = isDark ? '#6B7280' : '#374151';
```

---

## Interaction Patterns

### Transitions

All interactive elements use `opacity 0.2s ease` as the base transition.

### Hover States

| Element | Hover Effect |
|---------|-------------|
| Buttons / link-buttons | Opacity 0.7 |
| Navbar links | Opacity 0.6, remove underline |
| Table rows | Background color change (subtle) |
| Regular links | Add underline |
| Content links (div.links) | Add underline |

### Active / Selected States

| Element | Active State |
|---------|-------------|
| Navbar links | Underline + 4px offset |
| Content links | Underline + bold (700) |
| Link buttons | 2px outline, -3px offset |

### Disabled State

- Opacity: 0.5
- Cursor: default
- Border color: `#e5e7eb` (light) / `#374151` (dark)
- Text color: `--color-tertiary`

### Click Feedback

Submit buttons receive a `.clicked` class on press: opacity drops to 0.5 with a 0.3s transition (slightly longer than hover to feel deliberate).

### Philosophy

- No shadows, no scale transforms, no glow effects
- Depth expressed through tonal contrast and borders only
- Animations are functional (menu slide, button feedback), never decorative

---

## Utility Classes

| Class | Effect |
|-------|--------|
| `.float-right` | `float: right` |
| `.text-align-right` | `text-align: right` |
| `.text-align-left` | `text-align: left` |
| `.cursor-pointer` | `cursor: pointer` |
| `.padding-top-30` | `padding-top: 30px` |
| `.margin-left-20` | `margin-left: 20px` |
| `div.width90` | `width: 90%` |
| `div.flex` | `display: flex; flex-wrap: wrap` |
| `div.initial` | `display: initial` |
| `div.center` | `text-align: center` |
| `div.padding-top-60` | `padding-top: 60px` |

---

## Do's and Don'ts

### Do

- Keep the interface monochromatic and structurally quiet.
- Prioritize typography and whitespace over borders, shadows, or ornamental decoration.
- Use generous vertical spacing between sections (56px minimum).
- Keep labels small, precise, and lightly spaced for a refined corporate tone.
- Use CSS custom properties for all colors — never hardcode hex values in components.
- Keep chart containers inside `div.report` cards for consistent framing.
- Use border-radius 0 on cards — always sharp corners.
- Express depth through tonal layering (black > near-black > gray > white), not shadows.
- Use the single 800px breakpoint consistently.

### Don't

- Introduce bright accent colors or playful gradients.
- Add box-shadows, bevels, or skeuomorphic depth.
- Round card corners (cards are always `border-radius: 0`).
- Crowd content into dense multi-column layouts.
- Add new breakpoints beyond 800px without strong justification.
- Use filled button backgrounds — buttons are transparent with border only.
- Add decorative icons or emoji to UI chrome.
- Use scale transforms or bounce animations — all motion is opacity or translate based.

---

## CSS Architecture

Single file: `frontend/src/index.css`

Section order:
1. Font face declarations (commented, ready to enable)
2. Reset
3. Design tokens (`:root` + `[data-theme="dark"]` override)
4. Base typography
5. Utility classes
6. Links
7. Form controls
8. Buttons
9. Layout
10. Messages
11. Cards
12. Links navigation
13. Loading
14. Tables
15. Login
16. Pagination
17. Totals
18. HR
19. Navbar
20. Responsive media query

No CSS modules, no preprocessors, no CSS-in-JS. Pure vanilla CSS with custom properties.
