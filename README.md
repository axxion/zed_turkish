# Zed L10n — Zed Localization Support

**Zed, çevrilebilir hâliyle. / Zed, made localizable.**

[Zed](https://zed.dev) editörünün (taban: **v1.14.2**), arayüzünün tamamı bir JSON
dosyasıyla **herhangi bir dile** çevrilebilen çatalı. Türkçe hazır gelir.

A fork of the [Zed](https://zed.dev) editor (based on **v1.14.2**) whose entire UI
can be translated into **any language** through a single JSON file — no rebuild,
no toolchain. Turkish ships as the reference locale.

> Bu proje, [zed-industries/zed](https://github.com/zed-industries/zed) kaynak
> kodunun bağımsız bir çatalıdır. Zed Industries ile bağlantılı, onun onaylı veya
> sponsorlu bir ürünü **değildir**.
>
> This is an independent fork of [zed-industries/zed](https://github.com/zed-industries/zed).
> It is **not** affiliated with, endorsed by, or sponsored by Zed Industries.

---

## Neden / Why

Zed'in yerelleştirme altyapısı yok — bütün arayüz metinleri koda gömülü dize
sabitleri. Bir eklenti de bunu çözemez: Zed'in WASM eklenti API'si yalnızca dil
sunucusu, tema, slash komut ve hata ayıklama bağdaştırıcısı sunar; arayüz
metinlerine hiç erişemez.

Zed L10n bu boşluğu doldurur: çeviri kancası **gpui** katmanına, yani metnin ekrana
çizildiği yere yerleştirilmiştir. Çizilen her metin buradan geçer.

Zed has no i18n layer — every UI string is a hardcoded literal, and extensions
cannot reach them (Zed's WASM extension API only covers language servers, themes,
slash commands and debug adapters). Zed L10n puts the translation hook at the
**gpui** layer, where text is actually laid out, so every rendered string passes
through it.

---

## İndir / Download

**[→ Son sürüm / Latest release](https://github.com/axxion/Zed-L10n/releases/latest)**

ZIP'i indirip **klasörün tamamını** bir yere çıkarın (örneğin `%LOCALAPPDATA%\ZedL10n`)
ve `zed-l10n.exe` dosyasını çalıştırın. Kurulum sihirbazı yoktur.

Extract the **whole folder** somewhere (e.g. `%LOCALAPPDATA%\ZedL10n`) and run
`zed-l10n.exe`. There is no installer.

| Dosya / File | Ne işe yarar / Purpose |
|---|---|
| `zed-l10n.exe` | Uygulama. Türkçe sözlük içine gömülüdür / The app; Turkish dictionary is built in |
| `OpenConsole.exe` + `conpty.dll` | Terminal için gerekli (Microsoft ConPTY) / Required for the terminal |
| `locales/tr.json` | Türkçe dil dosyası; yeni diller için şablon / Turkish locale, template for others |

### Windows uyarısı / Windows warning

İkili **imzalanmamıştır**; SmartScreen "Bilinmeyen yayımcı" uyarısı verir. Bu bir
virüs uyarısı değildir. Geçmek için: **Ek bilgi → Yine de çalıştır**.

The binary is **unsigned**, so SmartScreen shows an "unknown publisher" prompt.
Click **More info → Run anyway**.

### Gereksinimler / Requirements

Windows 10 (1809+) veya Windows 11, 64 bit. Ek çalışma zamanı gerekmez —
Visual C++ Redistributable kurmanıza gerek yoktur.

---

## Dil seçme / Choosing a language

`settings.json` dosyanıza `locale` yazın; değişiklik **yeniden başlatmada**
etkinleşir.

Set `locale` in your `settings.json`; the change takes effect **after a restart**.

```jsonc
{
  "locale": "en"   // system | en | tr | de | ...
}
```

| Değer / Value | Sonuç / Effect |
|---|---|
| `system` | İşletim sistemi dili, yoksa İngilizce / OS language, falls back to English |
| `en` | Çeviri kapalı, özgün İngilizce / Translation off, original English |
| `tr`, `de`, … | `locales/<kod>.json` okunur / reads `locales/<code>.json` |

Arama sırası / Lookup order:

```
%APPDATA%\Zed-L10n\locales\<kod>.json    ← kendi dosyanız / your own file
<exe dizini>\locales\<kod>.json          ← ZIP'ten gelen / shipped
ikiliye gömülü Türkçe / built-in Turkish ← yalnızca "tr" / only for "tr"
özgün İngilizce / original English
```

## Kendi dilinize çevirin / Translate into your language

`locales/tr.json` dosyasını kopyalayıp kendi dil kodunuzla kaydedin, değerleri
çevirin, `locale` ayarını o koda getirin. Anahtar, uygulamanın **özgün İngilizce
metnidir**; değer, görmek istediğiniz metin.

Copy `locales/tr.json`, rename it to your language code, translate the values and
point `locale` at it. The key is the app's **original English string**; the value
is what you want displayed.

```json
{
  "New File": "Neue Datei",
  "Open Project": "Projekt öffnen",
  "Close {} Dock": "{} Dock schließen"
}
```

Dosyayı düzenleyip uygulamayı yeniden başlatın — **derleme gerekmez.**
Eşleşme bulunamayan metinler İngilizce kalır, yani sözlüğü adım adım
tamamlayabilirsiniz.

Edit, restart, done — **no rebuild.** Anything without a match stays in English,
so you can fill the dictionary incrementally.

### Yer tutucular / Placeholders

Bazı metinler çalışma zamanında kurulur. Bunların anahtarı şablonun kendisidir ve
Türkçe/Almanca gibi dillerde **sözcük sırası değişebildiği** için karşılık konumlu
yer tutucu kullanabilir:

Some strings are built at runtime. Their key is the template itself, and because
word order differs between languages the value may use positional placeholders:

```json
{
  "Changes since {}": "{} sonrasındaki değişiklikler",
  "Base: {0} — {1}": "{1} — temel: {0}"
}
```

### Yeni bir dil paketi başlatmak / Starting a new locale

Depodaki [`locales/tr.json`](locales/tr.json) tam Türkçe sözlüktür (2500+ giriş)
ve **anahtar listesi olarak** kullanılabilir: kopyalayın, değerleri kendi dilinize
çevirin, `locales/` klasörüne koyun.

The [`locales/tr.json`](locales/tr.json) in this repo is the complete Turkish
dictionary (2500+ entries) and doubles as **the list of translatable keys**: copy
it, translate the values, drop it into `locales/`.

**Önemli:** Seçtiğiniz dilde bir metnin karşılığı yoksa o metin İngilizce kalır,
Türkçeye düşmez — yarım çeviri karışımı olmasın diye. / **Note:** missing entries
fall back to English, never to Turkish.

### Sınırlar / Limitations

- Editör içeriği ve terminal çıktısı **hiçbir zaman** çevrilmez — kodunuz kodunuz
  olarak kalır. / Editor buffer content and terminal output are **never**
  translated.
- Seçici listelerinde çeviri uygulanan öğelerde bulanık arama vurgusu düşürülür;
  vurgu indeksleri özgün metne aittir. / In pickers, fuzzy-match highlighting is
  dropped for translated entries, since highlight indices refer to the source string.
- `format!` ile kurulan bazı metinler henüz şablona bağlanmamıştır; bunlar
  İngilizce kalır. / Some `format!`-built strings are not yet templated and remain
  in English.
- Ayarlarda henüz açılır dil menüsü yok; dil `settings.json` üzerinden seçilir. /
  No language dropdown in the settings UI yet; use `settings.json`.
- `"system"` Windows'ta çoğunlukla İngilizceye düşer (sistem dilini okumak için
  platform çağrısı gerekiyor). / `"system"` usually falls back to English on
  Windows.

---

## Bu sürümde neler var / What's in this build

- **Türkçe arayüz**: 2500+ giriş. Menü çubuğu, komut paleti, ayarların tamamı
  (başlıklar ve açıklamalar), ajan paneli, git paneli, editör menüleri, hata
  ayıklayıcı, terminal, arama, tooltip'ler, seçiciler, placeholder'lar.
- **Ayarlar resmi Zed'den ayrı** (`%APPDATA%\Zed-L10n`) — ikisi yan yana kurulabilir.
- **Otomatik güncelleme kapalı**: güncelleme akışı upstream sunucularına baktığı
  için kapatıldı; sürümler buradaki Releases üzerinden gelir.
- Pastel yeşil uygulama ikonu, `zed-l10n.exe` adı.

---

## Nasıl çalışıyor / How it works

| Katman / Layer | Dosya / File |
|---|---|
| Evrensel metin kancası — çizilen her `SharedString`/`&str` | `crates/gpui/src/elements/text.rs` (`set_text_translator`) |
| Motor: JSON yükleyici + yerleşik sözlük | `crates/ui/src/tr.rs`, `crates/ui/src/tr_more.rs` |
| Kancanın kurulumu | `crates/zed/src/main.rs` → `ui::tr::init()` |
| Seçici (picker) etiketleri | `crates/ui/src/components/label/highlighted_label.rs` |
| Editör placeholder'ları | `crates/editor/src/editor.rs` → `set_placeholder_text` |
| Dinamik metinler | `tr_format!` makrosu + `tr::format_translated` |

Öncelik / Precedence: `translations.json` → yerleşik sözlük → özgün metin.

---

## Derleme / Building (Windows)

Gereksinimler: Rust toolchain (`rust-toolchain.toml`), Visual Studio 2022 Build
Tools (MSVC + Windows SDK + CMake).

```bat
build_zed.bat cargo build --release -p zed
```

Çıktı / Output: `target\release\zed-l10n.exe`

Notlar / Notes:

- Çalıştırılabilir adı `zed-l10n` olduğu için `crates/paths/src/paths.rs` içindeki
  `APP_NAME` de `Zed-L10n` olmalıdır; `crates/zed/src/main.rs` içindeki bir assert
  bunu derleme zamanında denetler.
- Windows'ta **Smart App Control kapalı** olmalıdır, aksi hâlde imzasız ara
  çalıştırılabilirler engellenir (os error 4551).

### Araçlar / Tooling

| Betik / Script | İş / Purpose |
|---|---|
| `tools/tara-eksik-ceviri.ps1` | Çevrilmemiş görünür metinleri tarar |
| `tools/disa-aktar-translations.ps1` | Yerleşik sözlüğü `translations.json`'a aktarır |

---

## Lisans / License

**GPL-3.0-or-later** ([LICENSE-GPL](LICENSE-GPL)); Apache-2.0 işaretli bileşenlerde
[LICENSE-APACHE](LICENSE-APACHE) geçerlidir.

Orijinal kaynak / Original source: [zed-industries/zed](https://github.com/zed-industries/zed),
© Zed Industries, Inc. Tüm telif ve lisans bildirimleri korunmuştur.

"Zed" adı ve logosu Zed Industries'in ticari markalarıdır. "Zed L10n" adı yalnızca
çatalın hangi projeye dayandığını belirten tanımlayıcı bir kullanımdır; bu çatal
Zed Industries tarafından onaylanmamıştır.

"Zed" and its logo are trademarks of Zed Industries. "Zed L10n" is descriptive use
indicating the upstream project; this fork is not endorsed by Zed Industries.

Yayımlayan / Published by **AYA Vakfı**.
