# Gemma4 Swift Sample Design System

This sample is primarily a SwiftPM executable today. The design system below is for the future SwiftUI/iOS shell around the same on-device Gemma 4 runtime.

## Product Feel

Quiet, local, technical, trustworthy.

The UI should feel like a compact on-device model console, not a cloud chatbot. It should make these facts obvious:

- The model runs on Apple Silicon.
- First run may download weights.
- Prompts use the Gemma 4 formatter, not a generic chat template.
- Generation is local after weights are cached.

## Token Architecture

Use three layers:

1. Primitive tokens: raw color, type, radius, spacing values.
2. Semantic tokens: purpose aliases like `surface`, `primary`, `warning`.
3. Component tokens: specific values for message bubbles, status pills, and controls.

## Primitive Tokens

| Token | Value | Purpose |
| --- | --- | --- |
| `ink950` | `#111827` | Main text |
| `ink700` | `#374151` | Secondary text |
| `ink500` | `#6B7280` | Muted text |
| `paper50` | `#F9FAFB` | App background |
| `paper100` | `#F3F4F6` | Raised surface |
| `line200` | `#E5E7EB` | Borders |
| `blue600` | `#2563EB` | Primary action |
| `blue700` | `#1D4ED8` | Primary pressed |
| `amber600` | `#D97706` | Download/loading warning |
| `green600` | `#16A34A` | Ready/local status |
| `red600` | `#DC2626` | Error status |

## Semantic Tokens

| Token | Primitive | Purpose |
| --- | --- | --- |
| `background` | `paper50` | Main app canvas |
| `surface` | `paper100` | Cards and panels |
| `border` | `line200` | Separators |
| `textPrimary` | `ink950` | Main text |
| `textSecondary` | `ink700` | Secondary text |
| `textMuted` | `ink500` | Helper copy |
| `accent` | `blue600` | Primary actions |
| `accentPressed` | `blue700` | Pressed primary actions |
| `success` | `green600` | Model ready/local |
| `warning` | `amber600` | Download/loading |
| `danger` | `red600` | Failure |

## Component Tokens

| Component | Property | Token |
| --- | --- | --- |
| App shell | Background | `background` |
| Prompt field | Border | `border` |
| Prompt field | Radius | `radiusMedium` |
| User bubble | Background | `accent` |
| User bubble | Text | white |
| Assistant bubble | Background | `surface` |
| Assistant bubble | Text | `textPrimary` |
| Status pill ready | Background | `success` with low opacity |
| Status pill loading | Background | `warning` with low opacity |
| Status pill error | Background | `danger` with low opacity |

## Typography

- Use the system font for native platform feel.
- Use `.title2` or `.headline` for panel titles.
- Use `.body` for generated text.
- Use `.footnote` for status and privacy notes.
- Use `.monospaced()` only for token IDs, model IDs, and logs.

## Spacing

Use a compact 4-point scale:

| Token | Value |
| --- | --- |
| `spaceXS` | `4` |
| `spaceS` | `8` |
| `spaceM` | `12` |
| `spaceL` | `16` |
| `spaceXL` | `24` |

## Interaction States

| State | Rule |
| --- | --- |
| Loading model | Disable send, show amber status |
| Ready | Enable prompt, show green local status |
| Streaming | Append chunks live, keep stop affordance visible |
| Error | Show concise red error and recovery hint |

## Design Rule

Do not hide local-runtime constraints. First-run download, memory pressure, and unsupported devices should be visible and plain.
