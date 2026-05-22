---
version: alpha
name: Inditex Minimal Editorial
description: A stark, fashion-forward corporate system that pairs monochrome restraint with large imagery and sparse, precise typography.
colors:
  primary: "#000000"
  secondary: "#121212"
  tertiary: "#374151"
  neutral: "#FFFFFF"
  surface: "#FFFFFF"
  on-surface: "#000000"
  error: "#B91C1C"
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
rounded:
  none: 0px
  sm: 4px
  md: 8px
  lg: 12px
  xl: 16px
  full: 9999px
spacing:
  xs: 6px
  sm: 16px
  md: 56px
  lg: 82px
  xl: 160px
  gutter: 24px
  margin: 24px
components:
  button-primary:
    backgroundColor: "transparent"
    textColor: "{colors.on-surface}"
    typography: "{typography.label-md}"
    rounded: "{rounded.sm}"
    padding: "0px"
    height: "40px"
    width: "546px"
  button-secondary:
    backgroundColor: "transparent"
    textColor: "{colors.neutral}"
    typography: "{typography.label-md}"
    rounded: "{rounded.sm}"
    padding: "0px"
    height: "40px"
    width: "546px"
  button-tertiary:
    backgroundColor: "transparent"
    textColor: "{colors.on-surface}"
    typography: "{typography.label-md}"
    rounded: "{rounded.none}"
    padding: "0px"
    height: "auto"
    width: "auto"
  card:
    backgroundColor: "{colors.secondary}"
    textColor: "{colors.neutral}"
    rounded: "{rounded.md}"
    padding: "16px"
  input:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.on-surface}"
    typography: "{typography.body-md}"
    rounded: "{rounded.sm}"
    padding: "12px"
  chip:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.on-surface}"
    typography: "{typography.label-sm}"
    rounded: "{rounded.full}"
    padding: "6px"
---

# Inditex Minimal Editorial

## Overview
This system feels restrained, premium, and highly corporate, with a strong fashion/editorial sensibility. The interface is spacious and image-led, using minimal text chrome to keep attention on large visuals and brand messaging. It is designed for a broad public audience, but it speaks with a confident, low-noise tone that feels polished rather than playful.

## Colors
- **Primary (#000000):** Pure black used for key text, logo marks, navigation, and the strongest contrast moments. It establishes the brand’s crisp, monochrome identity.
- **Secondary (#121212):** A near-black used for deep panels, cards, and dark UI foundations. It keeps surfaces tonal rather than flat.
- **Tertiary (#374151):** A muted graphite used for borders, separators, and secondary structure when subtle contrast is needed.
- **Neutral (#FFFFFF):** White used for hero text over dark imagery, overlays, and the overall page canvas when light surfaces appear.
- **Surface (#FFFFFF):** The clean base surface for utility UI and light components, especially where content must feel detached from the immersive hero.
- **On-surface (#000000):** The default readable text color on light surfaces, keeping body copy sharp and highly legible.
- **Error (#B91C1C):** A restrained red reserved for validation or destructive states; it should remain rare so the palette stays disciplined.

## Typography
The system is built on abc_monument_grotesk_Lt and abc_monument_grotesk_Rg, giving the interface a modern grotesk voice with a fashion-publishing feel. Headlines are strong but not decorative: `headline-display` and `headline-lg` carry the boldest hierarchy, while `headline-md` and `headline-sm` soften into lighter weights for secondary story framing. Body text is compact and highly controlled, with slight positive letter-spacing that improves clarity in small sizes; labels and navigation use the 10px–12px range and should remain clean, minimally styled, and mostly sentence case rather than decorative uppercase.

## Layout
The layout is built around wide, full-bleed moments and a fixed, highly controlled top navigation. Content spacing is generous, with large vertical rhythm steps reflected in the `spacing` scale from `xs` to `xl`, so sections should breathe and avoid dense stacking. Utility controls, cookies, and small actions sit at the edges with tight padding, while primary editorial blocks should align to broad margins and maintain strong horizontal simplicity.

## Elevation & Depth
The design is intentionally flat and low-shadow, relying on contrast, borders, and imagery rather than raised surfaces. Depth comes from tonal layering between black, near-black, and white, plus the occasional subtle border in graphite for separation. When hierarchy is needed, use size, whitespace, and image treatment before introducing shadow.

## Shapes
The shape language is minimal and disciplined, with small radii on interactive elements and slightly softer corners on surfaces. `rounded.sm` at 4px is the dominant interactive corner treatment, while `rounded.md` at 8px is suitable for cards and content containers. Fully rounded shapes should be reserved for chips or small utility indicators only.

## Components
Buttons are understated and text-first. `button-primary` and `button-secondary` should feel nearly invisible structurally, with transparent backgrounds, 40px height, and wide minimum width only when used in broad hero or CTA contexts; the key distinction is text color, not fill color. `button-tertiary` should be a bare text action with no chrome. Hover and active states should stay subtle, using contrast shifts or underlines rather than filled backgrounds.

Cards should use the dark `card` treatment: near-black background, white text, a 1px graphite border, and `rounded.md`. They should feel like contained editorial modules, not shadowed app panels.

Inputs should remain clean and rectangular, with light surfaces, `rounded.sm`, and compact padding. Borders should be thin and neutral, with focus states expressed by contrast rather than elaborate glow.

Chips should be small, pill-shaped filters or metadata tokens with `rounded.full`, tight padding, and label-sized text. They should not become loud badges.

Navigation and utility links should use the smallest label scale, with generous spacing between items and no heavy decoration. Icons should remain thin, monochrome, and secondary to text.

## Do's and Don'ts
- Do keep the interface monochrome and structurally quiet.
- Do prioritize imagery and typography over borders, shadows, or ornamental decoration.
- Do use generous whitespace and broad alignment to support an editorial feel.
- Do keep labels small, precise, and lightly spaced for a refined corporate tone.
- Don't introduce bright accent colors or playful gradients.
- Don't add heavy shadows, bevels, or skeuomorphic depth.
- Don't round everything uniformly; preserve the sharp, controlled geometry of the system.
- Don't crowd content into dense cards or multi-column clutter unless the layout explicitly requires it.
