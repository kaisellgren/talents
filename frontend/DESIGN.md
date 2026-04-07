# Design System Strategy: The Digital Curator

## 1. Overview & Creative North Star
This design system is built on the Creative North Star of **"The Digital Curator."** In the context of high-end talent recruitment, we are not just building a database; we are building an intelligent, authoritative editorial experience. 

To move beyond the "standard SaaS" look, this system rejects the rigid, boxy constraints of traditional templates. Instead, it utilizes **intentional asymmetry**, high-contrast typography scales, and overlapping surface layers. The goal is a UI that feels like a premium digital workspace—one that conveys intelligence through breathing room and precision rather than clutter.

## 2. Colors & Visual Soul
The color palette is rooted in deep, nocturnal foundations to minimize eye strain and maximize the "pop" of AI-driven insights.

*   **Primary Foundation:** The background uses `surface` (`#060e20`), providing a depth that feels more sophisticated than flat charcoal.
*   **The "No-Line" Rule:** To achieve a truly premium feel, **1px solid borders are prohibited for sectioning.** Boundaries must be defined solely through background color shifts. For example, a main content area using `surface-container-low` should sit against the `surface` background without a stroke.
*   **Surface Hierarchy & Nesting:** We treat the UI as a series of physical layers. Use the `surface-container` tiers (Lowest to Highest) to create nested depth. An inner card should be `surface-container-high` placed upon a `surface-container` section.
*   **The "Glass & Gradient" Rule:** Floating elements (like modals or navigation bars) must use Glassmorphism. Utilize semi-transparent `surface_bright` with a `backdrop-blur` of 12px-20px. 
*   **Signature Textures:** Main CTAs and Hero backgrounds should not be flat. Use a subtle linear gradient transitioning from `primary` (`#a3a6ff`) to `primary_container` (`#9396ff`) at a 135-degree angle to provide a sense of "energy" and professional polish.

## 3. Typography: Editorial Authority
We use a dual-font strategy to balance character with utility.

*   **Display & Headlines (Manrope):** Chosen for its geometric modernism. Use `display-lg` through `headline-sm` to create an editorial feel. Bold weight should be used for candidate names and key data points to establish instant hierarchy.
*   **Functional Text (Inter):** The "workhorse." Use `title-md` down to `label-sm` for all interactive elements, body copy, and metadata. Inter’s high x-height ensures readability in dark mode.
*   **Intentional Contrast:** Pair a `display-sm` headline with a `label-md` uppercase subtitle (using `on_surface_variant`) to create a sophisticated, "magazine-style" layout.

## 4. Elevation & Depth: Tonal Layering
Traditional drop shadows are often a crutch for poor layout. This system prioritizes **Tonal Layering**.

*   **The Layering Principle:** Depth is achieved by "stacking" surface tokens. Place a `surface_container_lowest` element on a `surface_container_low` background to create a soft, natural lift.
*   **Ambient Shadows:** When a floating effect is required (e.g., a candidate profile modal), use an extra-diffused shadow: `blur: 40px`, `spread: -5px`. The shadow color must be a 15% opacity version of `surface_container_lowest`, never pure black.
*   **The "Ghost Border" Fallback:** If a border is required for accessibility in input fields, use a "Ghost Border": the `outline_variant` token at 20% opacity. 100% opaque borders are strictly forbidden as they "trap" the eye and break the flow.

## 5. Components
Primitive components must feel like bespoke tools, not default widgets.

*   **Buttons:** 
    *   *Primary:* Uses the `primary` gradient with `on_primary` text. Corners set to `DEFAULT` (0.5rem).
    *   *Secondary:* No fill. Use a Ghost Border (`outline_variant` @ 20%) and `primary` text color.
*   **Input Fields:** Use `surface_variant` for the field background with no border. On focus, transition the background to `surface_bright` and add a subtle 2px outer glow using `primary` at 30% opacity.
*   **Cards:** Forbid the use of divider lines. Separate content using vertical white space (8px, 16px, or 24px increments) or subtle background shifts between `surface_container_low` and `surface_container_high`.
*   **Recruitment-Specific Components:**
    *   *Match Score Chip:* Use `tertiary_container` (`#ff8ed2`) with `on_tertiary_container` text to highlight AI-driven candidate matches.
    *   *Status Micro-Indicators:* Use `primary_dim` for "Active" and `error_dim` for "Inactive," utilizing a soft pulse animation to indicate "live" AI processing.

## 6. Do’s and Don’ts

### Do:
*   **Do** use asymmetrical padding (e.g., more top padding than bottom) in Hero sections to create a sense of movement.
*   **Do** use `on_surface_variant` (`#a3aac4`) for all secondary labels to ensure the hierarchy doesn't feel "loud."
*   **Do** leverage the `xl` (1.5rem) roundedness for large-scale imagery or "Featured Candidate" banners to soften the professional tone.

### Don’t:
*   **Don’t** use pure white text. Always use `on_surface` (`#dee5ff`) to prevent "halation" (the glowing effect of white text on dark backgrounds).
*   **Don’t** use dividers or lines to separate list items. Use the `surface-container` shifts or 12px-16px of negative space.
*   **Don’t** use standard "drop shadows" on buttons. If a button needs to feel elevated, use a subtle `primary` glow or a slight scale-up (1.02x) on hover.
