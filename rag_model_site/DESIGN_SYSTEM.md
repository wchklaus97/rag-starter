# RAG Model Field Guide Design System

This site uses a small token system so the static GitHub Pages project can stay clean as it grows.

## Design Direction

The visual direction is a technical lab notebook:

- Paper-like light background
- Calm blue accent
- Dense but readable cards
- Serif display type for editorial structure
- High-legibility sans body type for data and multilingual text

The site should feel like a field guide for developers, not a SaaS dashboard.

## Token Architecture

Tokens follow three layers:

```text
Primitive values
  ↓
Semantic aliases
  ↓
Component tokens
```

Source files:

- `assets/design-tokens.json` — human-readable token inventory
- `assets/design-tokens.css` — runtime CSS variables imported by `index.html`
- `styles.css` — component/layout rules that consume the tokens

## Primitive Tokens

Primitive tokens are raw values:

- `--primitive-cream-50`
- `--primitive-ink-900`
- `--primitive-blue-650`
- `--primitive-space-1` through `--primitive-space-12`
- `--primitive-radius-sm`, `--primitive-radius-md`, `--primitive-radius-lg`
- `--primitive-font-serif`, `--primitive-font-sans`

Do not use primitive tokens directly inside components unless you are defining semantic tokens.

## Semantic Tokens

Semantic tokens describe purpose:

- `--color-bg`
- `--color-surface`
- `--color-text`
- `--color-text-muted`
- `--color-border`
- `--color-accent`
- `--color-accent-soft`
- `--font-display`
- `--font-body`
- `--space-sm`, `--space-md`, `--space-lg`

Use semantic tokens for layout and page-level styling.

## Component Tokens

Component tokens map the semantic layer into reusable UI pieces:

| Component | Tokens |
| --- | --- |
| Page | `--page-bg`, `--page-text`, `--page-grid-line` |
| Panel | `--panel-bg`, `--panel-border`, `--panel-radius`, `--panel-shadow` |
| Card | `--card-bg`, `--card-hover-shadow` |
| Control | `--control-bg`, `--control-border`, `--control-focus`, `--control-active-bg` |
| Tag / Code | `--tag-bg`, `--tag-radius`, `--code-bg`, `--code-radius` |

Use component tokens when changing the behavior of one UI pattern.

## Component Specs

### Panel

| State | Background | Border | Shadow |
| --- | --- | --- | --- |
| Default | `--panel-bg` | `--panel-border` | `--panel-shadow` |

### Model Card

| State | Background | Border | Shadow |
| --- | --- | --- | --- |
| Default | `--card-bg` | `--panel-border` | none |
| Hover | `--card-bg` | `--panel-border` | `--card-hover-shadow` |

### Filter Chip

| State | Background | Text | Border |
| --- | --- | --- | --- |
| Default | `--control-bg` | `--control-text` | `--control-border` |
| Hover | mixed surface | `--control-text` | accent-mixed border |
| Active | `--control-active-bg` | `--control-text` | accent-mixed border |

### Search Input

| State | Background | Text | Border |
| --- | --- | --- | --- |
| Default | `--control-bg` | `--control-text` | `--control-border` |
| Focus | `--control-bg` | `--control-text` | `--control-focus` outline |

## Rules

1. Put new color, spacing, type, and radius values in `assets/design-tokens.css`.
2. Avoid raw hex values in `styles.css`.
3. Prefer semantic/component tokens over primitives in components.
4. Keep the language toggle and filter controls keyboard accessible.
5. If a future dark theme is added, update semantic tokens first, not every component.
