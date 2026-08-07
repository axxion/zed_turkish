# Zed L10n — Zed Localization Support

**Zed, made localizable.**

A fork of the [Zed](https://zed.dev) editor (based on **v1.14.2**) whose entire UI
can be translated into **any language** by editing a single JSON file — no
rebuild, no toolchain. Turkish ships as the reference locale.

> This is an independent fork of [zed-industries/zed](https://github.com/zed-industries/zed).
> It is **not** affiliated with, endorsed by, or sponsored by Zed Industries.
> "Zed" is a trademark of Zed Industries; "Zed L10n" is descriptive use
> indicating the upstream project this fork is based on.

---

## Why

Zed has no i18n layer — every UI string is a hardcoded literal. Extensions can't
reach them either: Zed's WASM extension API only covers language servers, themes,
slash commands, context servers and debug adapters. There is no seam for UI text.

Zed L10n adds one. The translation hook sits in **gpui**, at the point where text
is laid out, so every rendered string passes through it.

---

## Download (Windows x64)

**[→ Latest release](https://github.com/axxion/Zed-L10n/releases/latest)**

Extract the **whole folder** somewhere (e.g. `%LOCALAPPDATA%\ZedL10n`) and run
`zed-l10n.exe`. There is no installer.

| File | Purpose |
|---|---|
| `zed-l10n.exe` | The app. Turkish dictionary is built in |
| `OpenConsole.exe` + `conpty.dll` | Required for the terminal (Microsoft ConPTY) — don't delete |
| `locales/tr.json` | Turkish locale; also the template for other languages |

### Windows "unknown publisher" warning

The binary is **not code-signed**, so Windows SmartScreen shows a warning on first
run. It is not a virus warning — it's the standard prompt for publishers Windows
doesn't recognise.

To proceed: **More info → Run anyway**

Each release publishes a SHA-256 checksum so you can verify the download matches
what was published. If you'd rather not trust a binary at all, the source is here
and builds reproducibly.

### Requirements

Windows 10 (1809+) or Windows 11, 64-bit. No additional runtime needed — you do
**not** need the Visual C++ Redistributable.

---

## Choosing a language

Set `locale` in your `settings.json`. The change takes effect **after a restart**.

```jsonc
{
  "locale": "en"   // system | en | tr | de | ...
}
```

| Value | Effect |
|---|---|
| `system` | Follow the OS language, falling back to English |
| `en` | Translation off — original English strings |
| `tr`, `de`, … | Reads `locales/<code>.json` |

Lookup order:

```
%APPDATA%\Zed-L10n\locales\<code>.json    ← your own file (wins)
<exe directory>\locales\<code>.json       ← shipped with the release
built-in Turkish dictionary               ← only when "tr" is selected
original English
```

---

## Translate it into your language

Copy [`locales/tr.json`](locales/tr.json), rename it to your language code,
translate the values, and point `locale` at it. Keys are the app's **original
English strings**; values are what gets displayed.

```json
{
  "New File": "Neue Datei",
  "Open Project": "Projekt öffnen",
  "Close {} Dock": "{} Dock schließen"
}
```

Drop the file in `locales/` next to the executable (or in
`%APPDATA%\Zed-L10n\locales\`), restart, done — **no rebuild.**

`locales/tr.json` is the complete dictionary (2,500+ entries) and doubles as
**the list of translatable strings**, so it's also the checklist for a new locale.

**Partial files are fine.** Anything you leave out stays in English, so you can
fill the dictionary in incrementally. Missing entries always fall back to
English, never to another locale — a half-German/half-Turkish UI would be worse
than a half-translated one.

### Placeholders

Some strings are built at runtime. Their key is the template itself, and the
value may reorder positional placeholders, because word order differs between
languages:

```json
{
  "Changes since {}": "Änderungen seit {}",
  "Base: {0} — {1}": "{1} — base: {0}"
}
```

### Requesting a language

Don't want to do it yourself? **[Tell us which language you need in issue
#1](https://github.com/axxion/Zed-L10n/issues/1)** — we'll translate it, add it to
the repo and ship it in a release. No Rust, no build step required. Corrections to
existing translations are welcome there too; the file is plain JSON, so a one-line
fix is a trivial PR.

---

## Limitations

- **Editor buffer content and terminal output are never translated.** They use
  `shape_line` directly and bypass the hook entirely — your code stays your code.
- **No plural or gender forms.** A flat key/value dictionary is enough for
  Turkish, but languages with complex plural rules (Russian, Arabic, Polish)
  would need something like [Fluent](https://projectfluent.org/) or ICU.
- **Keys are English source strings**, so a wording change upstream orphans the
  entry. There is no message-ID indirection.
- **In pickers, fuzzy-match highlighting is dropped for translated entries** —
  highlight indices are byte offsets into the source string. Untranslated entries
  (filenames, branches) keep their highlighting.
- **Some `format!`-built strings aren't templated yet** and remain in English.
- **`thiserror` messages** (`#[error("missing {provider} API key")]`) interpolate
  before anything can key them. Unsolved.
- **No language dropdown in the settings UI yet** — `locale` is set in
  `settings.json`.
- **`"system"` usually falls back to English on Windows**; reading the OS
  language needs a platform call that hasn't been added.
- **Extensions are not localized.**

---

## What's in this build

- **Turkish UI**: 2,500+ entries — menu bar, command palette, the entire settings
  UI (titles and descriptions), agent panel, git panel, editor menus, debugger,
  terminal, search, tooltips, pickers, placeholders
- **User data is separate from official Zed** (`%APPDATA%\Zed-L10n`), so both can
  be installed side by side
- **Auto-update is off**: the update flow points at upstream servers, so leaving
  it on could replace this build with English Zed. Releases come from this repo

---

## How it works

| Layer | File |
|---|---|
| Universal text hook — every `SharedString`/`&str` that gets drawn | `crates/gpui/src/elements/text.rs` (`set_text_translator`) |
| Engine: locale file loader + built-in dictionary | `crates/ui/src/tr.rs`, `crates/ui/src/tr_more.rs` |
| Hook registration | `crates/zed/src/main.rs` → `ui::tr::init()` |
| Picker labels | `crates/ui/src/components/label/highlighted_label.rs` |
| Editor placeholders | `crates/editor/src/editor.rs` → `set_placeholder_text` |
| Runtime-built strings | `tr_format!` macro + `tr::format_translated` |

Precedence: locale file → built-in dictionary → original string.

The gpui side is ~31 lines and is a complete no-op unless a translator is
registered.

---

## Building (Windows)

Requirements: Rust toolchain (see `rust-toolchain.toml`), Visual Studio 2022
Build Tools (MSVC + Windows SDK + CMake).

```bat
build_zed.bat cargo build --release -p zed
```

Output: `target\release\zed-l10n.exe`

Notes:

- The binary is named `zed-l10n`, so `APP_NAME` in `crates/paths/src/paths.rs`
  must be `Zed-L10n` — a compile-time assert in `crates/zed/src/main.rs` enforces
  this. Change both together.
- **Smart App Control must be off** on Windows, otherwise unsigned intermediate
  executables are blocked (os error 4551).

### Tooling

| Script | Purpose |
|---|---|
| `tools/tara-eksik-ceviri.ps1` | Scans for untranslated visible strings |
| `tools/disa-aktar-translations.ps1` | Exports the built-in dictionary to a locale file |

---

## License

**GPL-3.0-or-later** ([LICENSE-GPL](LICENSE-GPL)); Apache-2.0 applies to
components marked as such ([LICENSE-APACHE](LICENSE-APACHE)).

Original source: [zed-industries/zed](https://github.com/zed-industries/zed),
© Zed Industries, Inc. All original copyright and license notices are preserved.

Published by **AYA Vakfı**.
