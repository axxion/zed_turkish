# AYA Vakfı — Türkçe Çeviri Kılavuzu ve Durum Defteri

> Bu dosya projenin kalıcı hafızasıdır. Yeni çeviri işi yapmadan ÖNCE oku.
> Alt ajan (sub-agent) görevlendirirken bu dosyaya atıfta bulun.

## Mimari (2 katman)

1. **Derleme zamanı çeviriler** — koddaki doğrudan Türkçe literal'ler
   (menüler: `app_menus.rs`, paneller, karşılama, vs.)
2. **Çalışma zamanı çeviriler** — `ui::tr::translate()`:
   - `zed.exe`'nin yanındaki `translations.json` okunur (JSON[görünen metin] = yeni metin)
   - Eşleşme yoksa `builtin()` haritası (ui/src/tr.rs içindeki yerleşik Türkçeler)
   - Eşleşme yoksa metin aynen döner
   - **Kullanıcı dosyayı düzenleyip uygulamayı yeniden başlatınca çeviri değişir — derleme gerekmez**

### Kritik dosyalar

| Dosya | Görev |
|---|---|
| `crates/ui/src/tr.rs` | `translate()` + `builtin()` + JSON yükleyici. **TEK YAZAR dosyası** (alt ajanlar dokunmaz, sadece ana ajan) |
| `crates/ui/src/components/tooltip.rs` | TÜM tooltip'ler `translate()`'ten geçer — tooltip için KOD DEĞİŞMEDEN sadece builtin/JSON yeterli |
| `crates/command_palette/src/command_palette.rs` | `humanize_action_name()` → tr_labels + translate() sarmalı |
| `crates/command_palette/src/tr_labels.rs` | ~490 aksiyon adı → Türkçe (anahtar: aksiyon adı, camelCase) |
| `translations.json` (exe yanında + repo kökünde) | Kullanıcı düzenleyebilir örnek/geçersiz kılma dosyası |

## Çeviri kuralları (hatan yapılanlar!)

1. **Tooltip metinleri**: koddaki İngilizce literal'i BIRAK; Türkçeyi `builtin()` veya `translations.json`'a ekle. Kod değişikliği gerekmez.
2. **Diğer görünür metinler** (buton, label, modal başlığı, bölüm başlığı, placeholder, diyalog):
   `ui::tr::translate("...")` ile sar — **İngilizceyi anahtar olarak bırak**, Türkçeyi `builtin()`'e ekle.
   Böylece kullanıcı JSON ile geçersiz kılabilir (derlemesiz).
3. **`format!` / interpolasyon**: ASLA parçalama. Statik kısmı sar, `{}`/`{var}` yerinde kalsın.
   Örn: `format!("Open {}", name)` → `format!("{}", ui::tr::translate("Open {}", name))` DEĞİL;
   `format!("{}", ui::tr::translate("Open"))` + `name` şeklinde parçala veya bırak.
4. **Tuple/vec yapısını bozma** — GERÇEK HATA (terminal_view): 
   `vec![("Rename".into(), Box::new(RenameTerminal))]` → parantezler düşünce derleme kırıldı.
   Menü/aksiyon tuple'larında `("label", action)` çiftinin parantezleri ŞART.
5. Aksiyon adlarını (camelCase, `tr_labels` anahtarları) çevirme.
6. Kısayol adları, kod tanımlayıcıları, URL'ler, dosya yolları çevrilmez.
7. UTF-8: Türkçe karakterler sorunsuz; `translations.json` UTF-8 olmalı.
8. Testleri kırma: `simulate_prompt_answer("...")` gibi İngilizce bekleyen testlerin cevaplarını değiştirme.
9. `aria_label` gibi erişilebilirlik etiketleri opsiyonel — öncelik görsel tooltip/label.

## Durum (checklist)

- [x] Komut paleti + which-key + keymap editörü (tr_labels ~490)
- [x] Menü çubuğu (app_menus.rs — Dosya/Düzen/Seçim/Görünüm/Git/Çalıştır/Pencere/Yardım)
- [x] Pane sekme sağ tık menüsü + tooltip'ler
- [x] Karşılama ekranı, proje paneli, terminal görünümü, collab paneli parçaları
- [x] Tooltip runtime mekanizması (tooltip.rs → translate) — TÜM tooltip'ler JSON ile değiştirilebilir
- [x] builtin haritası: ~300 giriş (durum çubuğu, ayarlar modalı, editör/git/arama/terminal/collab/ajan metinleri)
- [x] Ayarlar modalı: render noktaları sarmalandı (settings_ui.rs) — sayfa/bölüm/ayar başlıkları runtime çevrilebilir
- [x] Editör sağ tık/gutter menüleri, git paneli, arama, terminal, collab, agent_ui sarmalandı (~330 nokta)
- [x] `cargo check -p zed` temiz
- [ ] Onay diyalogları (Don't Save vb.) — test bağımlılığı nedeniyle BİLİNÇLİ ertelendi (kural 8)
- [ ] `page_data.rs` ayar açıklamalarının tümü (uzun metinler; render noktası sarmalı çalışıyor, builtin'e eklenecek)
- [ ] settings_ui `Focus Content`/`Focus Navbar` kısayol etiketleri (&str tipi — ertelendi)
- [ ] Aria-label'ler (kural 9, opsiyonel)
- [ ] Kalan İngilizce yerler için backlog: TRANSLATION katalogları (3 tarama ajanı çıktısı) — yeni oturumda bu listeden devam et

## Ortam

- Derleme: `build_zed.bat cargo build --release -p zed` (incremental ~5-15 dk; ilk tam derleme 32 dk)
- Smart App Control KAPALI (reboot ile) — açılırsa build-script'ler engellenir (os error 4551)
- Yayın akışı: `zedturkce/zed` geliştirme → `zedturkce/zed-turkish` senkron + commit + push (axxion/zed_turkish)
- Çeviriler hem `zed/` hem `zed-turkish/`'e işlenmeli; `translations.json` repo kökünde durur, exe yanına kopyalanır
