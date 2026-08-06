//! Çalışma zamanı çeviri dosyası desteği.
//!
//! `zed.exe` dosyasının yanındaki `translations.json` okunur.
//! Dosya formatı basit bir sözlüktür: anahtar, ekranda gördüğünüz
//! (İngilizce veya Türkçe) metin; değer, görüntülemek istediğiniz metin.
//!
//! ```json
//! {
//!   "Open Threads Sidebar": "İleti dizileri kenar çubuğunu aç"
//! }
//! ```
//!
//! Dosyayı değiştirip uygulamayı yeniden başlatmanız yeterlidir;
//! derleme gerekmez. Dosya yoksa veya eşleşme bulunamazsa metin aynen döner.
//!
//! Öncelik: `translations.json` > yerleşik `builtin()` haritası > orijinal metin.

use std::{collections::HashMap, sync::OnceLock};

use gpui::SharedString;

static OVERRIDES: OnceLock<HashMap<String, String>> = OnceLock::new();

fn overrides() -> &'static HashMap<String, String> {
    OVERRIDES.get_or_init(load_translations)
}

fn load_translations() -> HashMap<String, String> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let path = dir.join("translations.json");
            if let Ok(text) = std::fs::read_to_string(path) {
                if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&text) {
                    return map;
                }
            }
        }
    }
    HashMap::new()
}

/// Kod içinde hâlâ İngilizce duran metinlerin yerleşik Türkçeleri.
/// `translations.json` bu listeyi çalışma zamanında ezebilir.
fn builtin(text: &str) -> Option<&'static str> {
    Some(match text {
        // ── Durum çubuğu / genel ──────────────────────────────────────────
        "Open Threads Sidebar" => "İleti dizileri kenar çubuğunu aç",
        "Project Search" => "Proje Ara",
        "Language Servers" => "Dil Sunucuları",
        "Hide Button" => "Düğmeyi Gizle",
        "New…" => "Yeni…",
        "Split Pane" => "Paneyi Böl",

        // ── Ayarlar: sayfa başlıkları ─────────────────────────────────────
        "General" => "Genel",
        "Appearance" => "Görünüm",
        "Keymap" => "Kısayollar",
        "Editor" => "Editör",
        "Languages & Tools" => "Diller ve Araçlar",
        "Search & Files" => "Arama ve Dosyalar",
        "Window & Layout" => "Pencere ve Düzen",
        "Panels" => "Paneller",
        "Debugger" => "Hata Ayıklayıcı",
        "Terminal" => "Terminal",
        "Version Control" => "Sürüm Kontrolü",
        "Collaboration" => "İşbirliği",
        "AI" => "Yapay Zekâ",
        "Network" => "Ağ",
        "Developer" => "Geliştirici",

        // ── Ayarlar: bölüm başlıkları ─────────────────────────────────────
        "Theme" => "Tema",
        "Keybindings" => "Kısayollar",
        "Auto Save" => "Otomatik Kaydet",
        "Agent Configuration" => "Ajan Yapılandırması",
        "Project Panel" => "Proje Paneli",
        "General Settings" => "Genel Ayarlar",
        "Security" => "Güvenlik",
        "Workspace Restoration" => "Çalışma Alanı Geri Yükleme",
        "Scoped Settings" => "Kapsamlı Ayarlar",
        "Privacy" => "Gizlilik",
        "Auto Update" => "Otomatik Güncelleme",
        "Instrumentation" => "Ölçümleme",
        "Buffer Font" => "Tampon Yazı Tipi",
        "UI Font" => "Arayüz Yazı Tipi",
        "Cursor" => "İmleç",
        "Highlighting" => "Vurgulama",
        "Guides" => "Kılavuzlar",

        // ── Ayarlar: ayar adları ve açıklamaları ──────────────────────────
        "Theme Mode" => "Tema Modu",
        "Font Family" => "Yazı Tipi Ailesi",
        "Font Size" => "Yazı Tipi Boyutu",
        "Line Height" => "Satır Yüksekliği",
        "Accessible Mode" => "Erişilebilir Mod",
        "Trust All Projects By Default" => "Varsayılan Olarak Tüm Projelere Güven",
        "Telemetry Diagnostics" => "Telemetri Tanıları",
        "Telemetry Metrics" => "Telemetri Metrikleri",
        "Auto Save Mode" => "Otomatik Kaydetme Modu",
        "Shell" => "Kabuk",
        "Disable AI" => "Yapay Zekâyı Devre Dışı Bırak",
        "Threads Sidebar Side" => "Konuşma Kenar Çubuğu Tarafı",
        "Single File Review" => "Tek Dosya İncelemesi",
        "Choose a static, fixed theme…" => "Statik, sabit bir tema seçin…",
        "What to do when using the 'close active item' action with no tabs." => {
            "'Etkin öğeyi kapat' eylemi sekme yokken kullanıldığında ne yapılacağı."
        }
        "What to do when the last window is closed." => "Son pencere kapatıldığında ne yapılacağı.",

        // ── Ayarlar: butonlar, placeholders, boş durumlar ─────────────────
        "Edit in settings.json" => "settings.json'da düzenle",
        "Fix in settings.json" => "settings.json'da düzelt",
        "Create Skill" => "Yetenek Oluştur",
        "Manage Trust" => "Güveni Yönet",
        "Configure" => "Yapılandır",
        "Reset to Default" => "Varsayılana Sıfırla",
        "Copy Link" => "Bağlantıyı Kopyala",
        "View Other Projects" => "Diğer Projeleri Görüntüle",
        "Add Server" => "Sunucu Ekle",
        "Add Local Server" => "Yerel Sunucu Ekle",
        "Add Remote Server" => "Uzak Sunucu Ekle",
        "Install from Extensions" => "Uzantılardan Kur",
        "Add Provider" => "Sağlayıcı Ekle",
        "Compatible APIs" => "Uyumlu API'ler",
        "API Key Configured" => "API Anahtarı Yapılandırıldı",
        "Reset Key" => "Anahtarı Sıfırla",
        "Add Agent" => "Ajan Ekle",
        "Install from Registry" => "Kayıt Defterinden Kur",
        "Create a Skill" => "Yetenek Oluştur",
        "Save Skill" => "Yetenek Kaydet",
        "Import from URL" => "URL'den İçe Aktar",
        "Add skill content…" => "Yetenek içeriği ekle…",
        "Enable Sandbox" => "Sandbox'ı Etkinleştir",
        "Add domain (e.g. github.com)…" => "Alan adı ekle (ör. github.com)…",
        "Start Testing" => "Testi Başlat",
        "Stop Testing" => "Testi Durdur",
        "Configured Servers" => "Yapılandırılmış Sunucular",
        "No MCP servers added yet. Click \"Add Server\"…" => {
            "Henüz MCP sunucusu eklenmedi. \"Sunucu Ekle\"ye tıklayın…"
        }
        "Select Theme..." => "Tema Seç...",
        "Select Icon Theme..." => "Simge Teması Seç...",
        "Filter action names…" => "Eylem adlarını filtrele…",
        "Create Keybinding" => "Kısayol Oluştur",
        "Edit Keystroke" => "Tuş Vuruşunu Düzenle",
        "Edit Arguments" => "Argümanları Düzenle",
        "Save" => "Kaydet",
        "Edit Keybinding" => "Kısayolu Düzenle",
        "View conflicts" => "Çakışmaları Görüntüle",
        "Show matching keybinds" => "Eşleşen kısayolları göster",
        "No matches found for the provided query" => "Verilen sorgu için eşleşme bulunamadı",
        "Type an action name" => "Bir eylem adı yazın",
        "Action" => "Eylem",
        "Arguments" => "Argümanlar",
        "Keystrokes" => "Tuş Vuruşları",
        "Context" => "Bağlam",
        "Source" => "Kaynak",
        "Clear Keystrokes" => "Tuş Vuruşlarını Temizle",
        "Start Recording" => "Kaydı Başlat",
        "Stop Recording" => "Kaydı Durdur",
        "Search extensions..." => "Uzantıları ara...",
        "Extensions" => "Uzantılar",
        "All" => "Tümü",
        "Installed" => "Kurulu",
        "Install" => "Kur",
        "Uninstall" => "Kaldır",
        "Upgrade" => "Güncelle",
        "Rebuild" => "Yeniden Derle",
        "Install Dev Extension" => "Geliştirici Uzantısı Kur",
        "No dev extensions are installed." => "Kurulu geliştirici uzantısı yok.",
        "Failed to load extensions. Please check your connection…" => {
            "Uzantılar yüklenemedi. Bağlantınızı kontrol edin…"
        }
        "No extensions that match your search." => "Aramanızla eşleşen uzantı yok.",
        "Visit Extension Repository" => "Uzantı Deposunu Ziyaret Et",
        "View Documentation" => "Belgeleri Görüntüle",
        "Enable Vim mode" => "Vim modunu etkinleştir",
        "Incompatible" => "Uyumsuz",
        "Select extension version..." => "Uzantı sürümü seç...",
        "Do you want to install the recommended extension…" => {
            "Önerilen uzantıyı kurmak istiyor musunuz…"
        }
        "No, don't install it" => "Hayır, kurma",
        "Overridden by dev extension." => "Geliştirici uzantısı tarafından geçersiz kılındı.",
        "Search fonts…" => "Yazı tiplerini ara…",
        "Search icon themes…" => "Simge temalarını ara…",
        "Search theme…" => "Tema ara…",
        "Search models…" => "Modelleri ara…",
        "Select a model…" => "Bir model seç…",
        "Enter to Confirm" => "Onaylamak için Enter",
        "No Results" => "Sonuç Yok",
        "No settings match" => "Eşleşen ayar yok",

        // ── Editör: sağ tık / gutter / git menüleri ───────────────────────
        "Run to Cursor" => "İmlece Kadar Çalıştır",
        "Evaluate Selection" => "Seçimi Değerlendir",
        "Go to Definition" => "Tanıma Git",
        "Go to Declaration" => "Bildirime Git",
        "Go to Type Definition" => "Tür Tanımına Git",
        "Go to Implementation" => "Uygulamaya Git",
        "Find All References" => "Tüm Referansları Bul",
        "Rename Symbol" => "Simgeyi Yeniden Adlandır",
        "Format Buffer" => "Tamponu Biçimlendir",
        "Format Selections" => "Seçimleri Biçimlendir",
        "Show Code Actions" => "Kod Eylemlerini Göster",
        "Add to Agent Thread" => "Ajan Konuşmasına Ekle",
        "Cut" => "Kes",
        "Copy" => "Kopyala",
        "Copy and Trim" => "Kopyala ve Kırp",
        "Copy Selection" => "Seçimi Kopyala",
        "Paste" => "Yapıştır",
        "Paste Text" => "Metin Yapıştır",
        "Open Markdown Preview" => "Markdown Önizlemesini Aç",
        "Open SVG Preview" => "SVG Önizlemesini Aç",
        "Open in Terminal" => "Terminalde Aç",
        "Copy Permalink" => "Kalıcı Bağlantıyı Kopyala",
        "View File History" => "Dosya Geçmişini Görüntüle",
        "Clear Run Status" => "Çalıştırma Durumunu Temizle",
        "Edit Bookmark" => "Yer İmini Düzenle",
        "Copy Path" => "Yolu Kopyala",
        "Copy Relative Path" => "Göreli Yolu Kopyala",
        "Reveal In Project Panel" => "Proje Panelinde Göster",
        "Unfold Excerpt" => "Alıntıyı Aç",
        "Fold Excerpt" => "Alıntıyı Katla",
        "Open File" => "Dosyayı Aç",
        "Show Symbol Outline" => "Simge Anahattını Göster",
        "Expand Excerpt" => "Alıntıyı Genişlet",
        "References" => "Referanslar",
        "Definitions" => "Tanımlar",
        "Implementations" => "Uygulamalar",
        "Declarations" => "Bildirimler",
        "Types" => "Türler",
        "Bookmarks" => "Yer İmleri",
        "Unset breakpoint" => "Kesme noktasını kaldır",
        "No executable code is associated with this line." => {
            "Bu satırla ilişkili çalıştırılabilir kod yok."
        }
        "Right-click for more options" => "Daha fazla seçenek için sağ tıklayın",

        // ── Git paneli ────────────────────────────────────────────────────
        "Stage" => "Hazırla",
        "Stage All" => "Tümünü Hazırla",
        "Stage File" => "Dosyayı Hazırla",
        "Unstage" => "Hazırlamayı Geri Al",
        "Unstage All" => "Tümünün Hazırlamasını Geri Al",
        "Unstage File" => "Dosyanın Hazırlamasını Geri Al",
        "Restore" => "Geri Yükle",
        "Restore All Changes" => "Tüm Değişiklikleri Geri Yükle",
        "Restore Checkpoint" => "Denetim Noktasını Geri Yükle",
        "Staged Changes" => "Hazırlanan Değişiklikler",
        "Unstaged Changes" => "Hazırlanmamış Değişiklikler",
        "Staged & Untracked" => "Hazırlanan ve İzlenmeyen",
        "Tracked & Untracked" => "İzlenen ve İzlenmeyen",
        "Discard Tracked Changes" => "İzlenen Değişiklikleri At",
        "Trash Untracked Files" => "İzlenmeyen Dosyaları Çöpe At",
        "Stash All" => "Tümünü Stashle",
        "Stash Pop" => "Stash'i Uygula",
        "View Stash" => "Stash'i Görüntüle",
        "Select a stash…" => "Bir stash seç…",
        "No stashes found" => "Stash bulunamadı",
        "Fetch" => "Getir",
        "Fetch From" => "Şuradan Getir",
        "Pull" => "Çek",
        "Pull (Rebase)" => "Çek (Rebase)",
        "Push" => "Gönder",
        "Push To" => "Şuraya Gönder",
        "Force Push" => "Zorla Push",
        "Publish" => "Yayınla",
        "Republish" => "Yeniden Yayınla",
        "Fetch updates from remote" => "Uzaktan güncellemeleri getir",
        "Push committed changes to remote" => "Commit edilen değişiklikleri uzaktaki depoya gönder",
        "Initialize Repository" => "Depoyu Başlat",
        "Enter commit message" => "Commit mesajı girin",
        "Enter git ref..." => "Git ref girin...",
        "Enter repository URL…" => "Depo URL'si girin…",
        "Clone a repository from GitHub or other sources." => {
            "GitHub'dan veya diğer kaynaklardan bir depo kopyalayın."
        }
        "Enter a name for this remote…" => "Bu uzak için bir ad girin…",
        "Remote name can't be empty" => "Uzak adı boş olamaz",
        "Switch or type to create a branch…" => "Dal oluşturmak için seç veya yaz…",
        "Local Branches" => "Yerel Dallar",
        "Remote Branches" => "Uzak Dallar",
        "Based off" => "Temel alınan",
        "No commits found" => "Commit bulunamadı",
        "No changes to commit" => "Commit edilecek değişiklik yok",
        "Some branches could not be loaded" => "Bazı dallar yüklenemedi",
        "No Git Repositories" => "Git Deposu Yok",
        "Commit message title exceeds" => "Commit mesajı başlığı sınırı aşıyor",
        "Generating Commit…" => "Commit oluşturuluyor…",
        "View Commit" => "Commit'i Görüntüle",
        "View Diff" => "Farkı Görüntüle",
        "View File" => "Dosyayı Görüntüle",
        "Open Diff" => "Farkı Aç",
        "Open File Diff" => "Dosya Farkını Aç",
        "Copy SHA" => "SHA'yı Kopyala",
        "Copy Ref Name" => "Ref Adını Kopyala",
        "Copy Tag" => "Etiketi Kopyala",
        "Show in Git Graph" => "Git Grafiğinde Göster",
        "Custom Commands" => "Özel Komutlar",
        "Learn More" => "Daha Fazla Bilgi",
        "Rename Branch" => "Dalı Yeniden Adlandır",
        "Force Delete Branch" => "Dalı Zorla Sil",
        "Delete Branch" => "Dalı Sil",
        "Hold alt to force delete" => "Zorla silmek için alt basılı tutun",
        "Use" => "Kullan",
        "Use Both" => "İkisini Kullan",
        "Resolve with Agent" => "Ajanla Çöz",
        "Search commits…" => "Commit'leri ara…",
        "Graph" => "Grafik",
        "Description" => "Açıklama",
        "Date" => "Tarih",
        "Author" => "Yazar",
        "Commit" => "Commit",
        "Columns" => "Sütunlar",
        "View" => "Görünüm",
        "List" => "Liste",
        "Tree" => "Ağaç",
        "Sort By" => "Sırala",
        "Group By" => "Grupla",
        "None" => "Yok",
        "Name" => "Ad",
        "Path" => "Yol",
        "Ref" => "Ref",
        "Switch" => "Değiştir",
        "Create" => "Oluştur",
        "Create New From" => "Şundan Yeni Oluştur",
        "Open Thread as Markdown" => "Konuşmayı Markdown Olarak Aç",
        "Inline Assist" => "Satır İçi Asistan",
        "Spawn Task" => "Görev Başlat",
        "Rerun task" => "Görevi yeniden çalıştır",
        "View on" => "Şurada Görüntüle:",
        "Add to .gitignore" => ".gitignore'a ekle",
        "Add to .git/info/exclude" => ".git/info/exclude dosyasına ekle",
        "Trust Directory" => "Dizine Güven",

        // ── Arama ─────────────────────────────────────────────────────────
        "Search…" => "Ara…",
        "Replace with…" => "Şununla değiştir…",
        "Search all files…" => "Tüm dosyalarda ara…",
        "Replace in project…" => "Projede değiştir…",
        "Include: e.g. src/**/*.rs" => "Dahil et: ör. src/**/*.rs",
        "Exclude: e.g. vendor/*, *.lock" => "Hariç tut: ör. vendor/*, *.lock",
        "Match case" => "Büyük/küçük harf eşleştir",
        "Match whole words" => "Tam sözcük eşleştir",
        "Match with regex" => "Regex ile eşleştir",
        "Also search files ignored by configuration" => {
            "Yapılandırma tarafından yok sayılan dosyalarda da ara"
        }
        "Use Regular Expressions" => "Düzenli İfadeler Kullan",
        "One Match Per Line" => "Satır Başına Bir Eşleşme",
        "Search Backwards" => "Geriye Doğru Ara",
        "Loading project…" => "Proje yükleniyor…",
        "Searching…" => "Aranıyor…",
        "Search All Files" => "Tüm Dosyalarda Ara",
        "No results found in this project for the provided query" => {
            "Verilen sorgu için bu projede sonuç bulunamadı"
        }
        "Hit enter to search. For more options:" => "Aramak için enter'a basın. Diğer seçenekler:",
        "Include/exclude specific paths" => "Belirli yolları dahil et/hariç tut",
        "Find and replace" => "Bul ve değiştir",
        "Find in Results" => "Sonuçlarda Bul",
        "No more matches" => "Başka eşleşme yok",
        "Expand All Files" => "Tüm Dosyaları Genişlet",
        "Collapse All Files" => "Tüm Dosyaları Daralt",
        "Toggle Replace" => "Değiştirmeyi Aç/Kapat",
        "Select Previous Match" => "Önceki Eşleşmeyi Seç",
        "Select Next Match" => "Sonraki Eşleşmeyi Seç",
        "Select All Matches" => "Tüm Eşleşmeleri Seç",
        "Close Search Bar" => "Arama Çubuğunu Kapat",
        "Replace Next Match" => "Sonraki Eşleşmeyi Değiştir",
        "Replace All Matches" => "Tüm Eşleşmeleri Değiştir",
        "Expand All Search Results" => "Tüm Arama Sonuçlarını Genişlet",
        "Collapse All Search Results" => "Tüm Arama Sonuçlarını Daralt",
        "Only Search Open Files" => "Yalnızca Açık Dosyalarda Ara",
        "Toggle Filters" => "Filtreleri Aç/Kapat",
        "Search Limits Reached\nTry narrowing your search" => {
            "Arama Sınırına Ulaşıldı\nAramanızı daraltmayı deneyin"
        }

        // ── Terminal ──────────────────────────────────────────────────────
        "New Terminal" => "Yeni Terminal",
        "New Center Terminal" => "Yeni Orta Terminal",
        "Clear" => "Temizle",
        "Split" => "Böl",
        "Split Right" => "Sağa Böl",
        "Split Left" => "Sola Böl",
        "Split Up" => "Yukarı Böl",
        "Split Down" => "Aşağı Böl",
        "Open Settings" => "Ayarları Aç",
        "Edit Settings" => "Ayarları Düzenle",
        "Edit settings.json" => "settings.json'ı düzenle",
        "Failed to spawn terminal" => "Terminal başlatılamadı",
        "Close Terminal Tab" => "Terminal Sekmesini Kapat",
        "Terminal Panel" => "Terminal Paneli",

        // ── İşbirliği / çağrı ─────────────────────────────────────────────
        "Current Call" => "Geçerli Görüşme",
        "Favorites" => "Sık Kullanılanlar",
        "Requests" => "İstekler",
        "Contacts" => "Kişiler",
        "Channels" => "Kanallar",
        "Invites" => "Davetler",
        "Online" => "Çevrimiçi",
        "Offline" => "Çevrimdışı",
        "Guest" => "Misafir",
        "Member" => "Üye",
        "Admin" => "Yönetici",
        "Invited" => "Davet Edildi",
        "You" => "Sen",
        "Calling" => "Aranıyor",
        "Calling…" => "Aranıyor…",
        "Mic only" => "Yalnızca mikrofon",
        "Screen" => "Ekran",
        "Follow" => "Takip Et",
        "Click to Follow" => "Takip etmek için tıkla",
        "Leave Call" => "Görüşmeden ayrıl",
        "Open Notes" => "Notları Aç",
        "Copy Channel Link" => "Kanal Bağlantısını Kopyala",
        "Copy Channel Notes Link" => "Kanal Notları Bağlantısını Kopyala",
        "Copy Link to Section" => "Bölüm Bağlantısını Kopyala",
        "Link copied to clipboard" => "Bağlantı panoya kopyalandı",
        "Room ID copied to clipboard" => "Oda kimliği panoya kopyalandı",
        "New Subchannel" => "Yeni Alt Kanal",
        "Manage Members" => "Üyeleri Yönet",
        "Move this channel" => "Bu kanalı taşı",
        "Make Channel Private" => "Kanalı Gizli Yap",
        "Make Channel Public" => "Kanalı Herkese Açık Yap",
        "Leave Channel" => "Kanaldan Ayrıl",
        "Delete" => "Sil",
        "Delete Profile" => "Profili Sil",
        "Grant Mic Access" => "Mikrofon Erişimi Ver",
        "Grant Write Access" => "Yazma Erişimi Ver",
        "Mute" => "Sustur",
        "Revoke Access" => "Erişimi Geri Al",
        "Remove Contact" => "Kişiyi Kaldır",
        "Remove from Channel" => "Kanaldan Kaldır",
        "Demote to Guest" => "Misafir Yap",
        "Promote to Member" => "Üye Yap",
        "Demote to Member" => "Üye Yap",
        "Promote to Admin" => "Yönetici Yap",
        "Invite Members" => "Üyeleri Davet Et",
        "Invite new contacts" => "Yeni kişileri davet et",
        "Search channels…" => "Kanalları ara…",
        "Search collaborator by username..." => "Kullanıcı adıyla işbirlikçi ara...",
        "Add a Contact" => "Kişi Ekle",
        "Public" => "Herkese Açık",
        "Collaboration is disabled for this organization." => {
            "Bu kuruluş için işbirliği devre dışı bırakılmış."
        }
        "Call Diagnostics" => "Çağrı Tanıları",
        "Inbound audio" => "Gelen ses",
        "Latency" => "Gecikme",
        "Jitter" => "Titreşim",
        "Packet loss" => "Paket kaybı",
        "Input lag" => "Giriş gecikmesi",
        "Excellent" => "Mükemmel",
        "Good" => "İyi",
        "Poor" => "Zayıf",
        "Lost" => "Kayıp",
        "Normal" => "Normal",
        "High" => "Yüksek",
        "No call diagnostics available" => "Kullanılabilir çağrı tanısı yok",
        "Waiting for inbound audio statistics" => "Gelen ses istatistikleri bekleniyor",
        "Copy Report" => "Raporu Kopyala",
        "Save Report…" => "Raporu Kaydet…",

        // ── Ajan ──────────────────────────────────────────────────────────
        "Agent Profiles" => "Ajan Profilleri",
        "Custom Profiles" => "Özel Profiller",
        "Add New Profile" => "Yeni Profil Ekle",
        "Fork Profile" => "Profili Çatalla",
        "Customize" => "Özelleştir",
        "Configure Default Model" => "Varsayılan Modeli Yapılandır",
        "Configure Built-in Tools" => "Yerleşik Araçları Yapılandır",
        "Configure MCP Tools" => "MCP Araçlarını Yapılandır",
        "Configure Server" => "Sunucuyu Yapılandır",
        "Go Back" => "Geri Dön",
        "Open Repository" => "Depoyu Aç",
        "Authenticate" => "Kimlik Doğrula",
        "Authenticate to connect this server" => "Bu sunucuya bağlanmak için kimlik doğrula",
        "Authenticating…" => "Kimlik doğrulanıyor…",
        "Submit" => "Gönder",
        "Cancel" => "İptal",
        "Decline" => "Reddet",
        "No changes to review" => "İncelenecek değişiklik yok",
        "Continue Iterating" => "Yinelemeye Devam Et",
        "Reject All" => "Tümünü Reddet",
        "Generating Changes…" => "Değişiklikler oluşturuluyor…",
        "Keep All" => "Hepsini Koru",
        "Jump to Edit" => "Düzenlemeye Atla",
        "Jump" => "Atla",
        "Scroll" => "Kaydır",
        "Scroll to Top" => "En Üste Kaydır",
        "Scroll to Bottom" => "En Alta Kaydır",
        "No model is configured for summarizing thread titles." => {
            "Konuşma başlıklarını özetlemek için model yapılandırılmamış."
        }
        "Click to show error." => "Hatayı göstermek için tıkla.",
        "Click to see logs." => "Günlükleri görmek için tıkla.",
        "Failed to run" => "Çalıştırılamadı",
        "Formatting failed:" => "Biçimlendirme başarısız:",
        "Downloading" => "İndiriliyor",
        "Checking for updates to" => "Güncellemeler kontrol ediliyor:",
        "Error loading" => "Yüklenirken hata",
        "Loading" => "Yükleniyor",
        "Process ID (PID)" => "İşlem Kimliği (PID)",
        "References to" => "Referanslar:",

        // ── Yardımcı ──────────────────────────────────────────────────────
        "Copy Description" => "Açıklamayı Kopyala",
        "Copy Message" => "Mesajı Kopyala",
        "Select All" => "Tümünü Seç",
        "Symbolic Link" => "Sembolik Bağlantı",
        _ => return None,
    })
}

/// Ekranda görünen `text` için çeviri arar; bulamazsa aynen döner.
pub fn translate(text: impl Into<SharedString>) -> SharedString {
    let text: SharedString = text.into();
    let text = text.as_ref();
    if let Some(translation) = overrides().get(text) {
        translation.as_str().into()
    } else if let Some(translation) = builtin(text) {
        translation.into()
    } else {
        text.into()
    }
}
