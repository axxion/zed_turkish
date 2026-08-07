# AYA Vakfı Editörü — Türkçe Zed Çatalı

[Zed](https://zed.dev) editörünün (taban: **v1.14.2**) tamamen **Türkçe arayüzlü** ve **AYA Vakfı** markalı özel derlemesi.

> Bu proje, [zed-industries/zed](https://github.com/zed-industries/zed) kaynak kodunun bağımsız bir çatalıdır.
> Zed Industries ile bağlantılı, onun onaylı veya sponsorlu bir ürünü değildir.

## İndir (Windows x64)

Derlemeye uğraşmak istemiyorsanız hazır sürümü indirin:

**[→ Son sürümü indir (Releases)](https://github.com/axxion/zed_turkish/releases/latest)**

`Zed-Turkce-<sürüm>.zip` dosyasını indirip **klasörün tamamını** bir yere çıkarın
(örneğin `C:\Program Files\ZedTurkce` veya `%LOCALAPPDATA%\ZedTurkce`), sonra
`zed.exe` dosyasını çalıştırın. Kurulum sihirbazı yoktur; çıkarıp çalıştırmanız yeterli.

ZIP içindekiler:

| Dosya | Ne işe yarar |
|---|---|
| `zed.exe` | Uygulamanın kendisi. Türkçe sözlük ikilinin içine derlenmiştir |
| `OpenConsole.exe` + `conpty.dll` | Windows'ta terminalin çalışması için gerekli (Microsoft ConPTY). **Silmeyin** |
| `translations.json` | İsteğe bağlı. Çevirileri derlemeden değiştirmek isterseniz (aşağıya bakın) |

### Windows "Bilinmeyen yayımcı" uyarısı

İkili dosya **kod imzalama sertifikasıyla imzalanmamıştır**, bu yüzden Windows
SmartScreen ilk çalıştırmada uyarı verir. Bu bir virüs uyarısı değildir; Windows
tanımadığı yayımcıların programlarına verdiği standart uyarıdır.

Geçmek için: **Ek bilgi** → **Yine de çalıştır**.

Güvenmek istemiyorsanız kaynak kod bu depoda açıktır; aşağıdaki adımlarla kendiniz
derleyip aynı sonucu elde edebilirsiniz.

### Gereksinimler

- Windows 10 (1809+) veya Windows 11, 64 bit
- Ek çalışma zamanı **gerekmez** — Visual C++ Redistributable kurmanıza gerek yoktur

## Bu sürümde neler var

- **Türkçe arayüz**: 2440 girişlik sözlük. Menü çubuğu, komut paleti, which-key,
  ayarların tamamı (sayfa başlıkları ve açıklamalar dahil), ajan paneli, git paneli,
  editör menüleri, hata ayıklayıcı, terminal, arama, işbirliği panelleri, tooltip'ler,
  seçici listeleri, arama kutusu placeholder'ları, Hakkında ekranı
- **Kapsama gpui seviyesinde**: çizilen her metin çeviri kancasından geçer, bu yüzden
  tek tek sarmalanmamış metinler de Türkçeleşir. Editör içeriği ve terminal çıktısı
  bu yoldan geçmez — yani kodunuz ve terminal çıktınız asla çevrilmez
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

- Arayüzde çizilen **her metin** bu mekanizmadan geçer — derleme gerekmez.
- Eşleşme bulunamazsa metin aynen gösterilir.
- Yerleşik varsayılan çeviriler `crates/ui/src/tr.rs` ve `crates/ui/src/tr_more.rs`
  içindedir; depodaki `translations.json` bunların dışa aktarılmış hâlidir ve
  exe'nin yanındaki kopya onları geçersiz kılar.
- **`translations.json` zorunlu değildir.** Sözlük ikilinin içine derlenmiştir;
  dosya olmasa da arayüz tam Türkçedir. Dosya yalnızca bir çeviriyi kendinize göre
  değiştirmek isterseniz gerekir.
- Çeviri kuralları ve durum için: [`TRANSLATION_GUIDE.md`](TRANSLATION_GUIDE.md)
