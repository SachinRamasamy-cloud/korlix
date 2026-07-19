#!/usr/bin/env python3
"""Static coverage verification for the complete Korlix showcase."""

from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"

EXPECTED_ROUTES = {
    "/",
    "/language",
    "/native-elements",
    "/styling",
    "/components",
    "/forms",
    "/state-events",
    "/api",
    "/overlays",
    "/pagination",
    "/themes",
}

EXPECTED_COMPONENTS = {
    "accordion",
    "accordion-item",
    "alert",
    "article-card",
    "audio-player",
    "avatar",
    "avatar-group",
    "badge",
    "bottom-navigation",
    "breadcrumb",
    "btn",
    "button",
    "calendar",
    "card",
    "carousel",
    "chart-container",
    "checkbox",
    "checkbox-group",
    "chip",
    "code-block",
    "color-picker",
    "column",
    "command-menu",
    "confirm-dialog",
    "container",
    "context-menu",
    "data-grid",
    "data-table",
    "date-picker",
    "divider",
    "drawer",
    "dropdown",
    "dropzone",
    "email-field",
    "empty-state",
    "feature-card",
    "field-error",
    "file-upload",
    "footer",
    "form-field",
    "form-group",
    "gallery",
    "grid",
    "hero",
    "icon",
    "image",
    "image-card",
    "image-viewer",
    "input",
    "key-value-list",
    "lightbox",
    "link",
    "list-group",
    "list-item",
    "map-container",
    "metric",
    "mobile-menu",
    "modal",
    "multi-select",
    "nav-group",
    "nav-item",
    "nav-list",
    "navbar",
    "notification",
    "number-field",
    "otp-input",
    "pagination",
    "password-field",
    "popover",
    "pricing-card",
    "product-card",
    "profile-card",
    "progress",
    "progress-bar",
    "progress-circle",
    "radio",
    "radio-group",
    "range-slider",
    "result-state",
    "row",
    "scroll-area",
    "search-box",
    "section",
    "select",
    "select-field",
    "sheet",
    "sidebar",
    "skeleton",
    "skeleton-card",
    "skeleton-grid",
    "slider",
    "spacer",
    "spinner",
    "split",
    "stack",
    "stat-card",
    "status-indicator",
    "step",
    "stepper",
    "switch",
    "table",
    "tabs",
    "tag",
    "testimonial-card",
    "text-field",
    "textarea",
    "textarea-field",
    "theme-toggle",
    "time-picker",
    "timeline",
    "timeline-item",
    "toast",
    "tooltip",
    "tree-view",
    "video-player",
}

COLOR_FAMILIES = {
    "neutral", "ash", "slate", "stone", "sand", "red", "coral",
    "orange", "amber", "yellow", "lime", "green", "emerald", "mint",
    "teal", "cyan", "sky", "blue", "indigo", "violet", "purple",
    "magenta", "pink", "rose", "gray", "zinc",
}

EVENTS = {
    "click", "double-click", "input", "change", "submit", "focus", "blur",
    "keydown", "keyup", "mouseenter", "mouseleave", "scroll", "load",
    "error", "drag", "drop", "touch-start", "touch-end",
}

RUNTIME_CALLS = {
    "toast", "showToast", "openModal", "closeModal", "openDrawer",
    "closeDrawer", "navigate", "goBack", "toggleTheme", "scrollTo",
    "copyToClipboard", "log",
}

API_MARKERS = {"get ", "post ", "put ", "patch ", "delete ", "reload ", "api.get("}


def fail(message: str) -> None:
    print(f"FAIL: {message}")
    raise SystemExit(1)


def unescaped_quote_count(line: str) -> int:
    count = 0
    escaped = False
    for char in line:
        if escaped:
            escaped = False
        elif char == "\\":
            escaped = True
        elif char == '"':
            count += 1
    return count


files = sorted(SRC.rglob("*.klx"))
if not files:
    fail("No KLX source files found")

source = "\n".join(path.read_text(encoding="utf-8") for path in files)

for path in files:
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if "\t" in line:
            fail(f"Tab indentation in {path.relative_to(ROOT)}:{number}")
        indent = len(line) - len(line.lstrip(" "))
        if line.strip() and indent % 2:
            fail(f"Indentation is not a multiple of two in {path.relative_to(ROOT)}:{number}")
        if unescaped_quote_count(line) % 2:
            fail(f"Unbalanced string quotes in {path.relative_to(ROOT)}:{number}")

routes = set(re.findall(r'^\s*page\s+[\w-]+\s+(?:at|route)\s+"([^"]+)"', source, re.M))
if routes != EXPECTED_ROUTES:
    fail(f"Route mismatch. Expected {sorted(EXPECTED_ROUTES)}, found {sorted(routes)}")

for path in files:
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        match = re.match(r'\s*import(?:\s+[\w-]+\s+from)?\s+"([^"]+)"', line)
        if match:
            target = (path.parent / match.group(1)).resolve()
            if not target.exists():
                fail(f"Missing import {match.group(1)} in {path.relative_to(ROOT)}:{number}")


used_components = set()
for path in files:
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped:
            first = stripped.split()[0].rstrip(":")
            if first in EXPECTED_COMPONENTS:
                used_components.add(first)
missing_components = sorted(EXPECTED_COMPONENTS - used_components)
if missing_components:
    fail(f"Missing registered component examples: {missing_components}")

missing_colors = sorted(
    family for family in COLOR_FAMILIES
    if not re.search(rf'-(?:{re.escape(family)})-', source)
)
if missing_colors:
    fail(f"Missing color families: {missing_colors}")

missing_levels = [level for level in range(13) if f"surface-violet-{level}" not in source]
if missing_levels:
    fail(f"Missing violet levels: {missing_levels}")

missing_events = sorted(
    event for event in EVENTS
    if f"{event}=" not in source and f"on:{event}:" not in source
)
if missing_events:
    fail(f"Missing event examples: {missing_events}")

missing_calls = sorted(call for call in RUNTIME_CALLS if f"{call}(" not in source)
if missing_calls:
    fail(f"Missing runtime call examples: {missing_calls}")

missing_api = sorted(marker.strip() for marker in API_MARKERS if marker not in source)
if missing_api:
    fail(f"Missing API examples: {missing_api}")

print(f"PASS: {len(files)} KLX files checked")
print(f"PASS: {len(routes)} routes covered")
print(f"PASS: {len(EXPECTED_COMPONENTS)} registered component names covered")
print(f"PASS: {len(COLOR_FAMILIES)} color families and 13 violet levels covered")
print(f"PASS: {len(EVENTS)} browser events covered")
print(f"PASS: {len(RUNTIME_CALLS)} runtime calls covered")
print("PASS: GET, POST, PUT, PATCH, DELETE, reload and api.get covered")
