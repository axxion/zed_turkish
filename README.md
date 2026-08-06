# AYA Vakfı Editörü — Türkçe Zed Çatalı

[Zed](https://zed.dev) editörünün (taban: **v1.14.2**) tamamen **Türkçe arayüzlü** ve **AYA Vakfı** markalı özel derlemesi.

> Bu proje, [zed-industries/zed](https://github.com/zed-industries/zed) kaynak kodunun bağımsız bir çatalıdır.
> Zed Industries ile bağlantılı, onun onaylı veya sponsorlu bir ürünü değildir.

## Bu sürümde neler var

- **Türkçe arayüz**: menü çubuğu, komut paleti, which-key, sekme ve panel menüleri, karşılama ekranı, proje/terminal/işbirliği panelleri, Hakkında ekranı
- **AYA Vakfı markası**: uygulama adı, pencere başlığı, sürüm kanalları (`org.ayavakfi.Aya`)
- **Onboarding kapalı**: ilk açılışta tanıtım turu açılmaz, doğrudan boş editör açılır
- **Ön tanımlı varsayılan ayarlar** (`assets/settings/default.json`):
  - Proje / outline / işbirliği / git panelleri **solda**, agent paneli **sağda**
  - Varsayılan model: **DeepSeek V4 Flash** (`enable_thinking: false`)
  - Agent için yaygın terminal komutları önceden onaylı, sandbox geniş izinli
  - **Telemetri kapalı** (metrics + diagnostics)
  - `buffer_font_size` / `ui_font_size`: 16

## Derleme (Windows)

Gereksinimler:

- Rust toolchain (`rust-toolchain.toml` içindeki sürüm; `rustup` ile kurulur)
- Visual Studio 2022 Build Tools (MSVC + Windows SDK + CMake)

Ortamı hazırlayıp derleyin (`build_zed.bat` bunların hepsini ayarlar):

```bat
build_zed.bat cargo build --release -p zed
```

Çıktı: `target\release\zed.exe`

Not: Windows'ta derlerken **Smart App Control (SAC)** kapalı olmalıdır; aksi hâlde imzasız ara çalıştırılabilirler engellenir (os error 4551).

## Lisans

Bu çatal **GPL-3.0-or-later** ([LICENSE-GPL](LICENSE-GPL)) lisanslıdır; Apache-2.0 işaretli bileşenlerde [LICENSE-APACHE](LICENSE-APACHE) geçerlidir.

Orijinal kaynak: [zed-industries/zed](https://github.com/zed-industries/zed) — telif hakkı Zed Industries, Inc. Tüm orijinal telif ve lisans bildirimleri korunmuştur.

"Zed" adı ve logosu Zed Industries'in ticari markasıdır; bu çatalda kullanılmaz, çatal Zed Industries tarafından onaylanmamıştır.

## Dil dosyası ile çeviri güncelleme (derlemesiz)

Uygulama, `zed.exe` dosyasının **yanındaki** `translations.json` dosyasını çalışma zamanında okur.
Bir metni değiştirmek için dosyaya **ekranda gördüğünüz metni anahtar olarak** ekleyin ve uygulamayı yeniden başlatın:

```json
{
  "Open Threads Sidebar": "İleti dizileri kenar çubuğunu aç",
  "General": "Genel",
  "Project Search": "Proje Ara"
}
```

- **Tooltip'ler, komut paleti, ayarlar modalı başlıkları, editör/git/arama menüleri** bu mekanizmadan geçer — derleme gerekmez.
- Eşleşme bulunamazsa metin aynen gösterilir.
- Yerleşik varsayılan çeviriler `crates/ui/src/tr.rs` içindeki `builtin()` haritasındadır; `translations.json` onları geçersiz kılar.
- Çeviri kuralları ve durum için: [`TRANSLATION_GUIDE.md`](TRANSLATION_GUIDE.md)
