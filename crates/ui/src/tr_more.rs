//! Ek Türkçe sözlük — ağırlıklı olarak Ayarlar arayüzü (`settings_ui`) metinleri.
//!
//! `tr.rs` içindeki çekirdek `builtin()` haritası büyüdüğü için ek girişler
//! buraya alındı. `tr::translate()` önce `translations.json`'a, sonra
//! `tr::builtin()`'e, sonra buraya bakar.
//!
//! Kural: anahtar özgün İngilizce metnin **birebir** kendisidir; kaynaktaki
//! kaçış dizileri (`\"`) aynen korunur.

/// Ayarlar arayüzü ve diğer ek metinlerin Türkçe karşılıkları.
pub(crate) fn builtin_more(text: &str) -> Option<&'static str> {
    Some(match text {
        // ── Ayarlar: başlıklar ────────────────────────────────────────────
        "Activate On Close" => "Kapatıldığında Etkinleştir",
        "Active Encoding Button" => "Etkin Kodlama Düğmesi",
        "Active File Name" => "Etkin Dosya Adı",
        "Active Language Button" => "Etkin Dil Düğmesi",
        "Active Line Width" => "Etkin Satır Genişliği",
        "Agent Panel Button" => "Ajan Paneli Düğmesi",
        "Agent Panel Default Height" => "Ajan Paneli Varsayılan Yüksekliği",
        "Agent Panel Default Width" => "Ajan Paneli Varsayılan Genişliği",
        "Agent Panel Dock" => "Ajan Paneli Yerleşimi",
        "Agent Panel Flexible Sizing" => "Ajan Paneli Esnek Boyutlandırma",
        "Agent Review" => "Ajan İncelemesi",
        "Allow Rewrap" => "Yeniden Sarmaya İzin Ver",
        "Allowed" => "İzin Verildi",
        "Alternate Scroll" => "Alternatif Kaydırma",
        "Always Treat Brackets As Autoclosed" => "Parantezleri Her Zaman Otomatik Kapatılmış Say",
        "Anthropic Data Retention" => "Anthropic Veri Saklama",
        "Audible Bell" => "Sesli Zil",
        "Auto Compact" => "Otomatik Sıkıştırma",
        "Auto Compact Threshold" => "Otomatik Sıkıştırma Eşiği",
        "Auto Fold Directories" => "Dizinleri Otomatik Katla",
        "Auto Indent" => "Otomatik Girinti",
        "Auto Indent On Paste" => "Yapıştırmada Otomatik Girinti",
        "Auto Open Files On Create" => "Oluşturulunca Dosyaları Otomatik Aç",
        "Auto Open Files On Drop" => "Sürükleyip Bırakınca Dosyaları Otomatik Aç",
        "Auto Open Files On Paste" => "Yapıştırınca Dosyaları Otomatik Aç",
        "Auto Reveal Entries" => "Öğeleri Otomatik Göster",
        "Autoscroll On Clicks" => "Tıklamada Otomatik Kaydır",
        "Background Coloring" => "Arka Plan Renklendirme",
        "Bold Folder Labels" => "Kalın Klasör Etiketleri",
        "Border Size" => "Kenarlık Boyutu",
        "Bottom Dock Layout" => "Alt Yerleşim Düzeni",
        "Breadcrumbs" => "Gezinti Yolu",
        "Buffer Font Family" => "Tampon Yazı Tipi Ailesi",
        "Buffer Font Size" => "Tampon Yazı Tipi Boyutu",
        "Button Layout" => "Düğme Düzeni",
        "Cancel Generation On Terminal Stop" => "Terminal Durunca Üretimi İptal Et",
        "Case Sensitive" => "Büyük/Küçük Harf Duyarlı",
        "Center on Match" => "Eşleşmede Ortala",
        "Centered Layout Left Padding" => "Ortalanmış Düzen Sol Boşluğu",
        "Centered Layout Right Padding" => "Ortalanmış Düzen Sağ Boşluğu",
        "CLI Default Open Behavior" => "CLI Varsayılan Açma Davranışı",
        "Close on File Delete" => "Dosya Silinince Kapat",
        "Code Actions" => "Kod Eylemleri",
        "Code Actions On Format" => "Biçimlendirmede Kod Eylemleri",
        "Code Font Family" => "Kod Yazı Tipi Ailesi",
        "Collaboration Panel Button" => "İşbirliği Paneli Düğmesi",
        "Collaboration Panel Default Width" => "İşbirliği Paneli Varsayılan Genişliği",
        "Collaboration Panel Dock" => "İşbirliği Paneli Yerleşimi",
        "Collapse Untracked Diff" => "İzlenmeyen Farkı Daralt",
        "Coloring" => "Renklendirme",
        "Colorize Brackets" => "Parantezleri Renklendir",
        "Commit Title Max Length" => "Commit Başlığı Azami Uzunluğu",
        "Completion Detail Alignment" => "Tamamlama Ayrıntısı Hizalaması",
        "Completion Menu Item Kind" => "Tamamlama Menüsü Öğe Türü",
        "Completion Menu Scrollbar" => "Tamamlama Menüsü Kaydırma Çubuğu",
        "Copy On Select" => "Seçince Kopyala",
        "Current Line Highlight" => "Geçerli Satır Vurgusu",
        "Cursor Blink" => "İmleç Yanıp Sönmesi",
        "Cursor Blinking" => "İmleç Yanıp Sönmesi",
        "Cursor Position Button" => "İmleç Konumu Düğmesi",
        "Cursor Shape" => "İmleç Şekli",
        "Cursor Shape - Insert Mode" => "İmleç Şekli — Ekleme Modu",
        "Cursor Shape - Normal Mode" => "İmleç Şekli — Normal Mod",
        "Cursor Shape - Replace Mode" => "İmleç Şekli — Değiştirme Modu",
        "Cursor Shape - Visual Mode" => "İmleç Şekli — Görsel Mod",
        "Cursors" => "İmleçler",
        "Custom Button Layout" => "Özel Düğme Düzeni",
        "Custom Digraphs" => "Özel İkili Karakterler",
        "Custom Line Height" => "Özel Satır Yüksekliği",
        "Dark Icon Theme" => "Koyu Simge Teması",
        "Dark Theme" => "Koyu Tema",
        "Data Collection" => "Veri Toplama",
        "Debounce" => "Gecikmeli Tetikleme",
        "Debugger Button" => "Hata Ayıklayıcı Düğmesi",
        "Debugger Panel Dock" => "Hata Ayıklayıcı Paneli Yerleşimi",
        "Debuggers" => "Hata Ayıklayıcılar",
        "Default Height" => "Varsayılan Yükseklik",
        "Default Mode" => "Varsayılan Mod",
        "Default Open Behavior" => "Varsayılan Açma Davranışı",
        "Default Width" => "Varsayılan Genişlik",
        "Delay" => "Gecikme",
        "Delay (milliseconds)" => "Gecikme (milisaniye)",
        "Detect Virtual Environment" => "Sanal Ortamı Algıla",
        "Diagnostic Badges" => "Tanı Rozetleri",
        "Diagnostics Button" => "Tanılar Düğmesi",
        "Diff Stats" => "Fark İstatistikleri",
        "Diff View Style" => "Fark Görünümü Biçimi",
        "Directory" => "Dizin",
        "Disable Git Integration" => "Git Tümleşmesini Devre Dışı Bırak",
        "Disable in Language Scopes" => "Dil Kapsamlarında Devre Dışı Bırak",
        "Display In" => "Şurada Göster",
        "Display Mode" => "Görüntüleme Modu",
        "Double Click In Multibuffer" => "Çoklu Tamponda Çift Tıklama",
        "Drag and Drop" => "Sürükle ve Bırak",
        "Drop Size Target" => "Bırakma Boyutu Hedefi",
        "Edit Debounce Ms" => "Düzenleme Gecikmesi (ms)",
        "Edit Keybindings" => "Kısayolları Düzenle",
        "Enable Feedback" => "Geri Bildirimi Etkinleştir",
        "Enable Git Diff" => "Git Farkını Etkinleştir",
        "Enable Git Status" => "Git Durumunu Etkinleştir",
        "Enable Keep Preview On Code Navigation" => "Kod Gezinmesinde Önizlemeyi Koru",
        "Enable Language Server" => "Dil Sunucusunu Etkinleştir",
        "Enable Preview File From Code Navigation" => "Kod Gezinmesinden Dosya Önizlemesi",
        "Enable Preview From File Finder" => "Dosya Bulucudan Önizleme",
        "Enable Preview From Multibuffer" => "Çoklu Tampondan Önizleme",
        "Enable Preview From Project Panel" => "Proje Panelinden Önizleme",
        "Enable Preview Multibuffer From Code Navigation" => "Kod Gezinmesinden Çoklu Tampon Önizlemesi",
        "Enabled" => "Etkin",
        "Ensure Final Newline On Save" => "Kaydederken Sondaki Satır Sonunu Garantile",
        "Entry Spacing" => "Öğe Aralığı",
        "Environment Variables" => "Ortam Değişkenleri",
        "Excerpt Context Lines" => "Alıntı Bağlam Satırları",
        "Expand Edit Card" => "Düzenleme Kartını Genişlet",
        "Expand Excerpt Lines" => "Alıntı Satırlarını Genişlet",
        "Expand Outlines With Depth" => "Anahatları Derinlikle Genişlet",
        "Expand Terminal Card" => "Terminal Kartını Genişlet",
        "Extend Comment On Newline" => "Yeni Satırda Yorumu Sürdür",
        "Fallback Branch Name" => "Yedek Dal Adı",
        "Fast Scroll Sensitivity" => "Hızlı Kaydırma Duyarlılığı",
        "Feature Flags" => "Özellik Bayrakları",
        "Fetch Timeout (milliseconds)" => "Getirme Zaman Aşımı (milisaniye)",
        "File Icons" => "Dosya Simgeleri",
        "File Scan Exclusions" => "Dosya Taramasından Hariç Tutulanlar",
        "File Scan Inclusions" => "Dosya Taramasına Dahil Edilenler",
        "File Type Associations" => "Dosya Türü İlişkilendirmeleri",
        "Focus Follows Mouse" => "Odak Fareyi İzler",
        "Focus Follows Mouse Debounce ms" => "Odak Fareyi İzleme Gecikmesi (ms)",
        "Folder Icons" => "Klasör Simgeleri",
        "Font Fallbacks" => "Yedek Yazı Tipleri",

        // ── Ayarlar: açıklamalar ──────────────────────────────────────────
        "(Linux only) choose how window control buttons are laid out in the titlebar." => {
            "(Yalnızca Linux) Pencere denetim düğmelerinin başlık çubuğunda nasıl dizileceğini seçin."
        }
        "(Linux only) whether Zed or your compositor should draw window decorations." => {
            "(Yalnızca Linux) Pencere süslemelerini Zed'in mi yoksa bileştiricinizin mi çizeceği."
        }
        "(macOS only) whether to allow Windows to tab together." => {
            "(Yalnızca macOS) Pencerelerin sekme hâlinde birleşmesine izin verilip verilmeyeceği."
        }
        "A mapping from languages to files and file extensions that should be treated as that language." => {
            "Hangi dosya ve dosya uzantılarının hangi dil sayılacağını belirleyen eşleme."
        }
        "Activates the Python virtual environment, if one is found, in the terminal's working directory." => {
            "Terminalin çalışma dizininde bir Python sanal ortamı bulunursa etkinleştirir."
        }
        "Additional code actions to run when formatting." => {
            "Biçimlendirme sırasında çalıştırılacak ek kod eylemleri."
        }
        "Allow sending requests to Anthropic models that cannot be offered with Zero Data Retention." => {
            "Sıfır Veri Saklama ile sunulamayan Anthropic modellerine istek gönderilmesine izin ver."
        }
        "Amount of indentation for nested items." => "İç içe öğeler için girinti miktarı.",
        "Amount of time to wait before changing focus." => "Odak değiştirmeden önce beklenecek süre.",
        "An optional string to override the title of the terminal tab." => {
            "Terminal sekmesinin başlığını geçersiz kılan isteğe bağlı metin."
        }
        "Any number of settings profiles that are temporarily applied on top of your existing user settings." => {
            "Mevcut kullanıcı ayarlarınızın üzerine geçici olarak uygulanan istediğiniz sayıda ayar profili."
        }
        "Automatically close files that have been deleted." => "Silinen dosyaları otomatik olarak kapat.",
        "Automatically compact the agent's context when it grows too large, summarizing earlier messages to free up room in the model's context window." => {
            "Ajanın bağlamı çok büyüdüğünde önceki mesajları özetleyerek modelin bağlam penceresinde yer açar."
        }
        "Automatically show a signature help pop-up." => "İmza yardımı açılır kutusunu otomatik göster.",
        "Border style for the minimap's scrollbar thumb." => {
            "Mini haritanın kaydırma çubuğu tutamacının kenarlık biçimi."
        }
        "Character counts at which to show wrap guides in the editor." => {
            "Editörde sarma kılavuzlarının gösterileceği karakter sayıları."
        }
        "Character counts at which to show wrap guides." => {
            "Sarma kılavuzlarının gösterileceği karakter sayıları."
        }
        "Choose a static, fixed theme or dynamically select themes based on appearance and light/dark modes." => {
            "Sabit bir tema seçin ya da temaları görünüme ve açık/koyu moda göre dinamik olarak belirleyin."
        }
        "Choose whether to use the selected light or dark icon theme or to follow your OS appearance configuration." => {
            "Seçili açık ya da koyu simge temasının mı kullanılacağını, yoksa işletim sistemi görünümünüzün mü izleneceğini seçin."
        }
        "Choose whether to use the selected light or dark theme or to follow your OS appearance configuration." => {
            "Seçili açık ya da koyu temanın mı kullanılacağını, yoksa işletim sistemi görünümünüzün mü izleneceğini seçin."
        }
        "Collect timing data for foreground and background executor tasks so they can be inspected via `zed: open performance profiler`. May lead to increased memory usage." => {
            "Ön plan ve arka plan görevleri için zamanlama verisi toplar; `zed: open performance profiler` ile incelenebilir. Bellek kullanımını artırabilir."
        }
        "Command to automatically run when Zed creates a Terminal Thread shell in the agent panel. Runs in your configured shell." => {
            "Zed, ajan panelinde bir Terminal Konuşması kabuğu oluşturduğunda otomatik çalıştırılacak komut. Yapılandırdığınız kabukta çalışır."
        }
        "Control when to show the active encoding in the status bar." => {
            "Etkin kodlamanın durum çubuğunda ne zaman gösterileceğini denetler."
        }
        "Control whether Git status is shown in the editor's gutter." => {
            "Git durumunun editörün kenar sütununda gösterilip gösterilmeyeceğini denetler."
        }
        "Controls automatic indentation behavior when typing." => {
            "Yazarken otomatik girinti davranışını denetler."
        }
        "Controls how LSP completions are inserted." => "LSP tamamlamalarının nasıl eklendiğini denetler.",
        "Controls how words are completed." => "Sözcüklerin nasıl tamamlandığını denetler.",
        "Controls line number display in the editor's gutter. \"disabled\" shows absolute line numbers, \"enabled\" shows relative line numbers for each absolute line, and \"wrapped\" shows relative line numbers for every line, absolute or wrapped." => {
            "Editörün kenar sütunundaki satır numarası gösterimini denetler. \"disabled\" mutlak satır numaralarını, \"enabled\" her mutlak satır için göreli numaraları, \"wrapped\" ise mutlak veya sarılmış her satır için göreli numaraları gösterir."
        }
        "Controls the appearance behavior of the tab's close button." => {
            "Sekmenin kapatma düğmesinin görünüm davranışını denetler."
        }
        "Controls when to use system clipboard in Vim mode." => {
            "Vim modunda sistem panosunun ne zaman kullanılacağını denetler."
        }
        "Controls where the `editor::rewrap` action is allowed for this language." => {
            "Bu dil için `editor::rewrap` eyleminin nerede geçerli olduğunu denetler."
        }
        "Controls whether edit predictions are shown immediately or manually." => {
            "Düzenleme tahminlerinin hemen mi yoksa elle mi gösterileceğini denetler."
        }
        "Controls whether edit predictions are shown in the given language scopes." => {
            "Düzenleme tahminlerinin belirtilen dil kapsamlarında gösterilip gösterilmeyeceğini denetler."
        }
        "Controls whether the closing characters are always skipped over and auto-removed no matter how they were inserted." => {
            "Kapatma karakterlerinin, nasıl eklenmiş olurlarsa olsunlar her zaman atlanıp otomatik kaldırılıp kaldırılmayacağını denetler."
        }
        "Controls whether Zed may collect training data when using Zed's Edit Predictions. Data is only collected for files in projects detected as open source. The default value uses the preference previously set via the status-bar toggle, or false if no preference has been stored." => {
            "Zed'in Düzenleme Tahminleri kullanılırken eğitim verisi toplayıp toplayamayacağını denetler. Veri yalnızca açık kaynak olarak algılanan projelerdeki dosyalar için toplanır. Varsayılan değer, daha önce durum çubuğundaki anahtarla belirlenen tercihi kullanır; tercih kaydedilmemişse false olur."
        }
        "Custom digraph mappings for Vim mode." => "Vim modu için özel ikili karakter eşlemeleri.",
        "Custom line height value (must be at least 1.0)." => {
            "Özel satır yüksekliği değeri (en az 1.0 olmalı)."
        }
        "Cursor shape for insert mode. Inherit uses the editor's cursor shape." => {
            "Ekleme modu için imleç şekli. Devral seçeneği editörün imleç şeklini kullanır."
        }
        "Cursor shape for normal mode." => "Normal mod için imleç şekli.",
        "Cursor shape for replace mode." => "Değiştirme modu için imleç şekli.",
        "Cursor shape for the editor." => "Editör için imleç şekli.",
        "Cursor shape for visual mode." => "Görsel mod için imleç şekli.",
        "Debounce threshold in milliseconds after which changes are reflected in the Git gutter." => {
            "Değişikliklerin Git kenar sütununa yansıması için milisaniye cinsinden bekleme eşiği."
        }
        "Default action when clicking a changed file in the Git panel." => {
            "Git panelinde değişmiş bir dosyaya tıklandığındaki varsayılan eylem."
        }
        "Default branch name will be when init.defaultbranch is not set in Git." => {
            "Git'te init.defaultbranch ayarlı değilken kullanılacak varsayılan dal adı."
        }
        "Default cursor shape for the terminal (bar, block, underline, or hollow)." => {
            "Terminal için varsayılan imleç şekli (çubuk, blok, alt çizgi veya içi boş)."
        }
        "Default depth to expand outline items in the current file." => {
            "Geçerli dosyada anahat öğelerinin genişletileceği varsayılan derinlik."
        }
        "Default height when the agent panel is docked to the bottom." => {
            "Ajan paneli alta yerleştirildiğindeki varsayılan yükseklik."
        }
        "Default height when the terminal is docked to the bottom (in pixels)." => {
            "Terminal alta yerleştirildiğindeki varsayılan yükseklik (piksel)."
        }
        "Default Prettier options, in the format as in package.json section for Prettier." => {
            "Varsayılan Prettier seçenekleri; package.json'daki Prettier bölümüyle aynı biçimde."
        }
        "Default width of the collaboration panel in pixels." => {
            "İşbirliği panelinin piksel cinsinden varsayılan genişliği."
        }
        "Default width of the Git panel in pixels." => "Git panelinin piksel cinsinden varsayılan genişliği.",
        "Default width of the outline panel in pixels." => {
            "Anahat panelinin piksel cinsinden varsayılan genişliği."
        }
        "Default width of the project panel in pixels." => {
            "Proje panelinin piksel cinsinden varsayılan genişliği."
        }
        "Default width when the agent panel is docked to the left or right." => {
            "Ajan paneli sola veya sağa yerleştirildiğindeki varsayılan genişlik."
        }
        "Default width when the terminal is docked to the left or right (in pixels)." => {
            "Terminal sola veya sağa yerleştirildiğindeki varsayılan genişlik (piksel)."
        }
        "Delay in milliseconds before drag and drop selection starts." => {
            "Sürükle-bırak seçimi başlamadan önceki milisaniye cinsinden gecikme."
        }
        "Delay in milliseconds before the which-key menu appears." => {
            "which-key menüsü belirmeden önceki milisaniye cinsinden gecikme."
        }
        "Determines how indent guide backgrounds are colored." => {
            "Girinti kılavuzu arka planlarının nasıl renklendirileceğini belirler."
        }
        "Determines how indent guides are colored." => {
            "Girinti kılavuzlarının nasıl renklendirileceğini belirler."
        }
        "Determines how snippets are sorted relative to other completion items." => {
            "Parçacıkların diğer tamamlama öğelerine göre nasıl sıralanacağını belirler."
        }
        "Determines the stepping granularity for debug operations." => {
            "Hata ayıklama işlemlerinde adımlama ayrıntı düzeyini belirler."
        }
        "Direction to split horizontally." => "Yatay bölme yönü.",
        "Direction to split vertically." => "Dikey bölme yönü.",
        "Disable all Git integration features in Zed." => {
            "Zed'deki tüm Git tümleşme özelliklerini devre dışı bırak."
        }
        "Display indent guides in the editor." => "Editörde girinti kılavuzlarını göster.",
        "Display the terminal title in breadcrumbs inside the terminal pane." => {
            "Terminal başlığını terminal panesindeki gezinti yolunda göster."
        }
        "Display the which-key menu with matching bindings while a multi-stroke binding is pending." => {
            "Çok tuşlu bir kısayol beklenirken eşleşen kısayolları which-key menüsünde göster."
        }
        "Duration in milliseconds to highlight yanked text in Vim mode." => {
            "Vim modunda kopyalanan metnin vurgulanacağı milisaniye cinsinden süre."
        }
        "Enable drag and drop selection." => "Sürükle-bırak seçimini etkinleştir.",
        "Enable Helix mode and key bindings." => "Helix modunu ve kısayollarını etkinleştir.",
        "Enable middle-click paste on Linux." => "Linux'ta orta tık ile yapıştırmayı etkinleştir.",
        "Enable smartcase searching in Vim mode." => {
            "Vim modunda akıllı büyük/küçük harf aramasını etkinleştir."
        }
        "Enable to show entries in tree view list, disable to show in flat view list." => {
            "Öğeleri ağaç görünümünde listelemek için etkinleştirin, düz listede görmek için kapatın."
        }
        "Enable Vim mode and key bindings." => "Vim modunu ve kısayollarını etkinleştir.",
        "Enables or disables formatting with Prettier for a given language." => {
            "Belirtilen dil için Prettier ile biçimlendirmeyi açar veya kapatır."
        }
        "Extra task variables to set for a particular language." => {
            "Belirli bir dil için tanımlanacak ek görev değişkenleri."
        }
        "Fast scroll sensitivity multiplier for both horizontal and vertical scrolling." => {
            "Yatay ve dikey kaydırma için hızlı kaydırma duyarlılık çarpanı."
        }
        "Files or globs of files that will be excluded by Zed entirely. They will be skipped during file scans, file searches, and not be displayed in the project file tree. Takes precedence over \"File Scan Inclusions\"" => {
            "Zed'in tamamen hariç tutacağı dosyalar veya dosya kalıpları. Dosya taramalarında ve aramalarda atlanır, proje dosya ağacında görünmez. \"Dosya Taramasına Dahil Edilenler\" ayarına göre önceliklidir."
        }
        "Files or globs of files that will be included by Zed, even when ignored by git. This is useful for files that are not tracked by git, but are still important to your project. Note that globs that are overly broad can slow down Zed's file scanning. \"File Scan Exclusions\" takes precedence over these inclusions" => {
            "Git tarafından yok sayılsa bile Zed'in dahil edeceği dosyalar veya dosya kalıpları. Git ile izlenmeyen ama projeniz için önemli dosyalar için kullanışlıdır. Çok geniş kalıpların Zed'in dosya taramasını yavaşlatabileceğini unutmayın. \"Dosya Taramasından Hariç Tutulanlar\" bu dahil etmelere göre önceliklidir."
        }
        "Font fallbacks for terminal text. If not set, defaults to buffer font fallbacks." => {
            "Terminal metni için yedek yazı tipleri. Ayarlanmazsa tampon yedek yazı tipleri kullanılır."
        }
        // ── Ayarlar: yazı tipi, biçimlendirme, Git ────────────────────────
        "Font Features" => "Yazı Tipi Özellikleri",
        "Font Weight" => "Yazı Tipi Kalınlığı",
        "Format DAP Log Messages" => "DAP Günlük Mesajlarını Biçimlendir",
        "Format On Save" => "Kaydederken Biçimlendir",
        "Formatter" => "Biçimlendirici",
        "Git Diff" => "Git Farkı",
        "Git Panel Button" => "Git Paneli Düğmesi",
        "Git Panel Default Width" => "Git Paneli Varsayılan Genişliği",
        "Git Panel Dock" => "Git Paneli Yerleşimi",
        "Git Panel Status Style" => "Git Paneli Durum Biçimi",
        "Git Status" => "Git Durumu",
        "Git Status Indicator" => "Git Durum Göstergesi",
        "Global Substitution Default" => "Genel Değiştirme Varsayılanı",
        "Go To Definition Fallback" => "Tanıma Gitme Yedeği",
        "Go To Definition Scroll Strategy" => "Tanıma Gitme Kaydırma Stratejisi",
        "Hard Tabs" => "Sekme Karakteri",
        "Hidden Files" => "Gizli Dosyalar",
        "Hide .gitignore" => ".gitignore'u Gizle",
        "Hide Hidden" => "Gizlileri Gizle",
        "Hide Mouse" => "Fareyi Gizle",
        "Hide Root" => "Kökü Gizle",
        "Hiding Delay" => "Gizleme Gecikmesi",
        "Highlight on Yank Duration" => "Kopyalamada Vurgu Süresi",
        "Horizontal Scroll" => "Yatay Kaydırma",
        "Horizontal Scroll Margin" => "Yatay Kaydırma Kenar Boşluğu",
        "Horizontal Scrollbar" => "Yatay Kaydırma Çubuğu",
        "Horizontal Split Direction" => "Yatay Bölme Yönü",
        "Hunk Style" => "Parça Biçimi",
        "Icon Theme" => "Simge Teması",
        "Icon Theme Name" => "Simge Teması Adı",
        "Image Viewer" => "Görsel Görüntüleyici",
        "Inactive Opacity" => "Etkin Olmayan Saydamlık",
        "Include Ignored" => "Yok Sayılanları Dahil Et",
        "Include Ignored in Search" => "Aramaya Yok Sayılanları Dahil Et",
        "Include Warnings" => "Uyarıları Dahil Et",
        "Indent Size" => "Girinti Boyutu",
        "Inline Code Actions" => "Satır İçi Kod Eylemleri",
        "Input Audio Device" => "Ses Giriş Aygıtı",
        "Insert Mode" => "Ekleme Modu",
        "JSX Tag Auto Close" => "JSX Etiketini Otomatik Kapat",
        "Keep Selection On Copy" => "Kopyalarken Seçimi Koru",
        "Left padding for centered layout." => "Ortalanmış düzen için sol boşluk.",
        "Light Icon Theme" => "Açık Simge Teması",
        "Light Theme" => "Açık Tema",
        "Limit Content Width" => "İçerik Genişliğini Sınırla",
        "Limit Markdown Preview Width" => "Markdown Önizleme Genişliğini Sınırla",
        "Line Ending" => "Satır Sonu",
        "Line Endings Button" => "Satır Sonları Düğmesi",
        "Line Width" => "Satır Genişliği",
        "Linked Edits" => "Bağlı Düzenlemeler",
        "LLM Providers" => "LLM Sağlayıcıları",
        "Location" => "Konum",
        "Log DAP Communications" => "DAP İletişimini Günlüğe Yaz",
        "LSP Document Colors" => "LSP Belge Renkleri",
        "LSP Document Symbols" => "LSP Belge Simgeleri",
        "LSP Folding Ranges" => "LSP Katlama Aralıkları",
        "LSP Results Location" => "LSP Sonuçları Konumu",
        "Max Content Width" => "Azami İçerik Genişliği",
        "Max Scroll History Lines" => "Azami Kaydırma Geçmişi Satırı",
        "Max Severity" => "Azami Önem Derecesi",
        "Max Width" => "Azami Genişlik",
        "Max Width Columns" => "Azami Genişlik Sütunu",
        "Maximum Tabs" => "Azami Sekme",
        "Menu Delay" => "Menü Gecikmesi",
        "Message Editor Min Lines" => "Mesaj Editörü Asgari Satır",
        "Middle Click Paste" => "Orta Tık ile Yapıştır",
        "Min Line Number Digits" => "Asgari Satır Numarası Basamağı",
        "Minimum Column" => "Asgari Sütun",
        "Minimum Contrast" => "Asgari Karşıtlık",
        "Minimum Contrast For Highlights" => "Vurgular İçin Asgari Karşıtlık",
        "Minimum Split Diff Width" => "Asgari Bölünmüş Fark Genişliği",
        "Mode" => "Mod",
        "Mouse Wheel Zoom" => "Fare Tekerleğiyle Yakınlaştır",
        "Multi Cursor Modifier" => "Çoklu İmleç Değiştiricisi",
        "Mute On Join" => "Katılınca Sustur",
        "Notify When Agent Waiting" => "Ajan Beklerken Bildir",
        "On Last Window Closed" => "Son Pencere Kapatıldığında",
        "Open Keymap" => "Kısayolları Aç",
        "Open Links In Mouse Mode" => "Bağlantıları Fare Modunda Aç",
        "Option As Meta" => "Option Tuşunu Meta Yap",
        "Options" => "Seçenekler",
        "Outline Panel Button" => "Anahat Paneli Düğmesi",
        "Outline Panel Default Width" => "Anahat Paneli Varsayılan Genişliği",
        "Outline Panel Dock" => "Anahat Paneli Yerleşimi",
        "Output Audio Device" => "Ses Çıkış Aygıtı",
        "Padding" => "İç Boşluk",
        "Parser" => "Ayrıştırıcı",
        "Path Style" => "Yol Biçimi",
        "Performance Profiler" => "Performans Profilleyici",
        "Pinned Tabs Layout" => "Sabitlenmiş Sekme Düzeni",
        "Play Sound When Agent Done" => "Ajan Bitirince Ses Çal",
        "Plugins" => "Eklentiler",
        "Prefer LSP" => "LSP'yi Tercih Et",
        "Preferred Line Length" => "Tercih Edilen Satır Uzunluğu",
        "Preview Channel" => "Önizleme Kanalı",
        "Preview Tabs Enabled" => "Önizleme Sekmeleri Etkin",
        "Primary Click Behavior" => "Birincil Tıklama Davranışı",
        "Private Files" => "Özel Dosyalar",
        "Program" => "Program",
        "Project Panel Button" => "Proje Paneli Düğmesi",
        "Project Panel Default Width" => "Proje Paneli Varsayılan Genişliği",
        "Project Panel Dock" => "Proje Paneli Yerleşimi",
        "Project Search Button" => "Proje Arama Düğmesi",
        "Proxy" => "Vekil Sunucu",
        "Quick Actions" => "Hızlı Eylemler",
        "Redact Private Values" => "Özel Değerleri Gizle",
        "Reduce Motion" => "Hareketi Azalt",
        "Regex" => "Düzenli İfade",
        "Regex Search" => "Düzenli İfade Araması",
        "Relative Line Numbers" => "Göreli Satır Numaraları",
        "Remove Trailing Whitespace On Save" => "Kaydederken Sondaki Boşlukları Sil",
        "Restore File State" => "Dosya Durumunu Geri Yükle",
        "Restore On Startup" => "Açılışta Geri Yükle",
        "Restore Unsaved Buffers" => "Kaydedilmemiş Tamponları Geri Yükle",
        "Right padding for centered layout." => "Ortalanmış düzen için sağ boşluk.",
        "Rounded Selection" => "Yuvarlatılmış Seçim",
        "Sandbox" => "Sandbox",
        "Save Breakpoints" => "Kesme Noktalarını Kaydet",
        "Scan Symbolic Links" => "Sembolik Bağlantıları Tara",
        "Scroll Bar" => "Kaydırma Çubuğu",
        "Scroll Beyond Last Line" => "Son Satırın Ötesine Kaydır",
        "Scroll Debounce Ms" => "Kaydırma Gecikmesi (ms)",
        "Scroll Multiplier" => "Kaydırma Çarpanı",
        "Scroll Sensitivity" => "Kaydırma Duyarlılığı",

        // ── Ayarlar: açıklamalar (2. parti) ───────────────────────────────
        "Font family for agent response text in the agent panel. Falls back to the regular UI font family." => {
            "Ajan panelindeki ajan yanıtı metni için yazı tipi ailesi. Ayarlanmazsa normal arayüz yazı tipi kullanılır."
        }
        "Font family for code blocks in the markdown preview. Falls back to the editor font family." => {
            "Markdown önizlemesindeki kod blokları için yazı tipi ailesi. Ayarlanmazsa editör yazı tipi kullanılır."
        }
        "Font family for editor text." => "Editör metni için yazı tipi ailesi.",
        "Font family for terminal text. If not set, defaults to buffer font family." => {
            "Terminal metni için yazı tipi ailesi. Ayarlanmazsa tampon yazı tipi kullanılır."
        }
        "Font family for the markdown preview. Falls back to the UI font family." => {
            "Markdown önizlemesi için yazı tipi ailesi. Ayarlanmazsa arayüz yazı tipi kullanılır."
        }
        "Font family for UI elements." => "Arayüz öğeleri için yazı tipi ailesi.",
        "Font family for user messages in the agent panel. Falls back to the regular buffer font family." => {
            "Ajan panelindeki kullanıcı mesajları için yazı tipi ailesi. Ayarlanmazsa normal tampon yazı tipi kullanılır."
        }
        "Font features for terminal text." => "Terminal metni için yazı tipi özellikleri.",
        "Font size for agent response text in the agent panel. Falls back to the regular UI font size." => {
            "Ajan panelindeki ajan yanıtı metni için yazı tipi boyutu. Ayarlanmazsa normal arayüz boyutu kullanılır."
        }
        "Font size for editor text." => "Editör metni için yazı tipi boyutu.",
        "Font size for terminal text. If not set, defaults to buffer font size." => {
            "Terminal metni için yazı tipi boyutu. Ayarlanmazsa tampon yazı tipi boyutu kullanılır."
        }
        "Font size for the markdown preview. Falls back to the editor font size." => {
            "Markdown önizlemesi için yazı tipi boyutu. Ayarlanmazsa editör boyutu kullanılır."
        }
        "Font size for UI elements." => "Arayüz öğeleri için yazı tipi boyutu.",
        "Font size for user messages text in the agent panel." => {
            "Ajan panelindeki kullanıcı mesajı metni için yazı tipi boyutu."
        }
        "Font weight for editor text (100-900)." => "Editör metni için yazı tipi kalınlığı (100-900).",
        "Font weight for terminal text in CSS weight units (100-900)." => {
            "Terminal metni için CSS kalınlık birimiyle yazı tipi kalınlığı (100-900)."
        }
        "Font weight for UI elements (100-900)." => "Arayüz öğeleri için yazı tipi kalınlığı (100-900).",
        "Forces Prettier integration to use a specific parser name when formatting files with the language." => {
            "Bu dildeki dosyalar biçimlendirilirken Prettier'ın belirli bir ayrıştırıcıyı kullanmasını zorlar."
        }
        "Forces Prettier integration to use specific plugins when formatting files with the language." => {
            "Bu dildeki dosyalar biçimlendirilirken Prettier'ın belirli eklentileri kullanmasını zorlar."
        }
        "Global switch to toggle hints on and off." => "İpuçlarını topluca açıp kapatan genel anahtar.",
        "Global switch to toggle inline values on and off when debugging." => {
            "Hata ayıklarken satır içi değerleri topluca açıp kapatan genel anahtar."
        }
        "Globs to match against file paths to determine if a file is private." => {
            "Bir dosyanın özel sayılıp sayılmayacağını belirlemek için dosya yollarıyla eşleştirilecek kalıplar."
        }
        "Globs to match files that will be considered \"hidden\" and can be hidden from the project panel." => {
            "\"Gizli\" sayılacak ve proje panelinde gizlenebilecek dosyaları eşleştiren kalıplar."
        }
        "GNOME-style layout string such as \"close:minimize,maximize\"." => {
            "\"close:minimize,maximize\" gibi GNOME biçiminde düzen metni."
        }
        "Hide the values of variables in private files." => "Özel dosyalardaki değişken değerlerini gizle.",
        "Highlight all occurrences of selected text." => "Seçili metnin tüm geçtiği yerleri vurgula.",
        "How `zed <path>` opens directories when no flag is specified." => {
            "Bayrak verilmediğinde `zed <yol>` komutunun dizinleri nasıl açacağı."
        }
        "How and when the scrollbar should be displayed." => {
            "Kaydırma çubuğunun nasıl ve ne zaman gösterileceği."
        }
        "How entry statuses are displayed." => "Öğe durumlarının nasıl gösterileceği.",
        "How Git hunks are displayed visually in the editor." => {
            "Git parçalarının editörde görsel olarak nasıl gösterileceği."
        }
        "How line endings should be handled for new files and during format and save operations." => {
            "Yeni dosyalarda ve biçimlendirme/kaydetme işlemlerinde satır sonlarının nasıl ele alınacağı."
        }
        "How many characters has to be in the completions query to automatically show the words-based completions." => {
            "Sözcük tabanlı tamamlamaların otomatik gösterilmesi için sorguda bulunması gereken karakter sayısı."
        }
        "How many columns a tab should occupy." => "Bir sekmenin kaç sütun kaplayacağı.",
        "How many lines of context to provide in multibuffer excerpts by default." => {
            "Çoklu tampon alıntılarında varsayılan olarak kaç satır bağlam sunulacağı."
        }
        "How many lines to expand the multibuffer excerpts by default." => {
            "Çoklu tampon alıntılarının varsayılan olarak kaç satır genişletileceği."
        }
        "How much to fade out unused code (0.0 - 0.9)." => {
            "Kullanılmayan kodun ne kadar soluklaştırılacağı (0.0 - 0.9)."
        }
        "How projects open from the UI by default." => {
            "Projelerin arayüzden varsayılan olarak nasıl açılacağı."
        }
        "How thinking blocks should be displayed by default. 'Auto' fully expands during streaming, then auto-collapses when done. 'Preview' auto-expands with a height constraint during streaming. 'Always Expanded' shows full content. 'Always Collapsed' keeps them collapsed." => {
            "Düşünme bloklarının varsayılan gösterimi. 'Auto' akış sırasında tamamen açılır, bitince kendiliğinden kapanır. 'Preview' akış sırasında yükseklik sınırıyla açılır. 'Always Expanded' tüm içeriği gösterir. 'Always Collapsed' kapalı tutar."
        }
        "How to display diffs in the editor." => "Farkların editörde nasıl gösterileceği.",
        "How to display the LSP item kind (function, method, variable, etc.) of each entry in the completions menu." => {
            "Tamamlama menüsündeki her öğenin LSP türünün (işlev, yöntem, değişken vb.) nasıl gösterileceği."
        }
        "How to group entries in the git panel." => "Git panelindeki öğelerin nasıl gruplanacağı.",
        "How to highlight the current line in the minimap." => {
            "Mini haritada geçerli satırın nasıl vurgulanacağı."
        }
        "How to highlight the current line." => "Geçerli satırın nasıl vurgulanacağı.",
        "How to perform a buffer format." => "Tampon biçimlendirmesinin nasıl yapılacağı.",
        "How to render LSP color previews in the editor." => {
            "LSP renk önizlemelerinin editörde nasıl çizileceği."
        }
        "How to scroll the target into view when navigating to a definition or reference." => {
            "Bir tanıma veya referansa giderken hedefin görünüme nasıl kaydırılacağı."
        }
        "How to soft-wrap long lines of text." => "Uzun metin satırlarının nasıl yumuşak sarılacağı.",
        "How to sort entries in the git panel." => "Git panelindeki öğelerin nasıl sıralanacağı.",
        "Include ignored files in search results by default." => {
            "Arama sonuçlarına varsayılan olarak yok sayılan dosyaları da dahil et."
        }
        "Key-value pairs to add to the terminal's environment." => {
            "Terminalin ortamına eklenecek anahtar-değer çiftleri."
        }
        "Layout mode for the bottom dock." => "Alt yerleşim için düzen modu.",
        "Line height for editor text." => "Editör metni için satır yüksekliği.",
        "Line height for terminal text." => "Terminal metni için satır yüksekliği.",
        "Maximum content width in pixels. Content will be centered when the pane is wider than this value." => {
            "Piksel cinsinden azami içerik genişliği. Pane bu değerden genişse içerik ortalanır."
        }
        "Maximum content width in pixels. Content will be centered when the panel is wider than this value." => {
            "Piksel cinsinden azami içerik genişliği. Panel bu değerden genişse içerik ortalanır."
        }
        "Maximum length of the commit message title before a warning is shown. Set to 0 to disable." => {
            "Uyarı gösterilmeden önce commit mesajı başlığının azami uzunluğu. Devre dışı bırakmak için 0 yapın."
        }
        "Maximum number of columns to display in the minimap." => {
            "Mini haritada gösterilecek azami sütun sayısı."
        }
        "Maximum number of lines to keep in scrollback history (max: 100,000; 0 disables scrolling)." => {
            "Kaydırma geçmişinde tutulacak azami satır sayısı (en fazla 100.000; 0 kaydırmayı kapatır)."
        }
        "Maximum open tabs in a pane. Will not close an unsaved tab." => {
            "Bir panedeki azami açık sekme sayısı. Kaydedilmemiş sekme kapatılmaz."
        }
        "Minimum number of characters to reserve space for in the gutter." => {
            "Kenar sütununda yer ayrılacak asgari karakter sayısı."
        }
        "Minimum number of lines to display in the agent message editor." => {
            "Ajan mesaj editöründe gösterilecek asgari satır sayısı."
        }
        "Minimum time to wait before pulling diagnostics from the language server(s)." => {
            "Dil sunucularından tanı çekmeden önce beklenecek asgari süre."
        }
        "Modifier key for adding multiple cursors." => "Birden fazla imleç eklemek için değiştirici tuş.",
        "Number of lines to search for modelines (set to 0 to disable)." => {
            "Modeline aranacak satır sayısı (devre dışı bırakmak için 0 yapın)."
        }
        "On: format the whole buffer.\nOff: do not format.\nModifications: format only lines with unstaged changes; skips formatting when a git diff or LSP range formatting is unavailable.\nModifications If Available: same, but falls back to formatting the whole buffer." => {
            "On: tüm tamponu biçimlendirir.\nOff: biçimlendirmez.\nModifications: yalnızca hazırlanmamış değişikliği olan satırları biçimlendirir; git farkı veya LSP aralık biçimlendirmesi yoksa atlar.\nModifications If Available: aynısı, ancak yoksa tüm tamponu biçimlendirir."
        }
        "Opacity of inactive panels (0.0 - 1.0)." => "Etkin olmayan panellerin saydamlığı (0.0 - 1.0).",
        "Optimize Zed's interface for assistive technology such as screen readers. When enabled, otherwise-collapsed controls stay expanded and keyboard-reachable." => {
            "Zed arayüzünü ekran okuyucu gibi yardımcı teknolojiler için iyileştirir. Etkinken normalde daraltılmış denetimler açık ve klavyeyle erişilebilir kalır."
        }
        "Padding between the end of the source line and the start of the inline blame in columns." => {
            "Kaynak satırın sonu ile satır içi blame'in başlangıcı arasındaki sütun cinsinden boşluk."
        }
        "Position of the close button in a tab." => "Sekmedeki kapatma düğmesinin konumu.",
        "Preferred debuggers for this language." => "Bu dil için tercih edilen hata ayıklayıcılar.",
        "Relative size of the drop target in the editor that will open dropped file as a split pane." => {
            "Bırakılan dosyayı bölünmüş pane olarak açacak bırakma hedefinin editördeki göreli boyutu."
        }
        "Restore previous file state when reopening." => "Yeniden açarken önceki dosya durumunu geri yükle.",
        "Save after inactivity period (in milliseconds)." => {
            "Belirtilen hareketsizlik süresinden sonra kaydet (milisaniye)."
        }
        "Scroll sensitivity multiplier for both horizontal and vertical scrolling." => {
            "Yatay ve dikey kaydırma için kaydırma duyarlılık çarpanı."
        }
        "Search case-sensitively by default." => "Varsayılan olarak büyük/küçük harfe duyarlı ara.",
        "Search for whole words by default." => "Varsayılan olarak tam sözcük ara.",

        // ── Ayarlar: başlıklar (3. parti) ─────────────────────────────────
        "Search Results" => "Arama Sonuçları",
        "Search Wrap" => "Arama Sarması",
        "Seed Search Query From Cursor" => "Arama Sorgusunu İmleçten Doldur",
        "Selected Symbol" => "Seçili Simge",
        "Selected Text" => "Seçili Metin",
        "Selection Highlight" => "Seçim Vurgusu",
        "Selections Menu" => "Seçimler Menüsü",
        "Server URL" => "Sunucu URL'si",
        "Settings Profiles" => "Ayar Profilleri",
        "Share On Join" => "Katılınca Paylaş",
        "Show" => "Göster",
        "Show Author Name" => "Yazar Adını Göster",
        "Show Avatar" => "Avatarı Göster",
        "Show Background" => "Arka Planı Göster",
        "Show Bookmarks" => "Yer İmlerini Göster",
        "Show Branch Name" => "Dal Adını Göster",
        "Show Branch Status Icon" => "Dal Durum Simgesini Göster",
        "Show Breakpoints" => "Kesme Noktalarını Göster",
        "Show Close Button" => "Kapatma Düğmesini Göster",
        "Show Commit Summary" => "Commit Özetini Göster",
        "Show Completion Documentation" => "Tamamlama Belgelerini Göster",
        "Show Completions On Input" => "Yazarken Tamamlamaları Göster",
        "Show Count Badge" => "Sayı Rozetini Göster",
        "Show Diagnostics" => "Tanıları Göster",
        "Show Edit Predictions" => "Düzenleme Tahminlerini Göster",
        "Show Edit Predictions in Normal Mode" => "Normal Modda Düzenleme Tahminlerini Göster",
        "Show File Icons In Tabs" => "Sekmelerde Dosya Simgelerini Göster",
        "Show Folds" => "Katlamaları Göster",
        "Show Full File by Default" => "Varsayılan Olarak Tüm Dosyayı Göster",
        "Show Git Status In Tabs" => "Sekmelerde Git Durumunu Göster",
        "Show Indent Guides" => "Girinti Kılavuzlarını Göster",
        "Show Line Numbers" => "Satır Numaralarını Göster",
        "Show Menus" => "Menüleri Göster",
        "Show Merge Conflict Indicator" => "Birleştirme Çakışması Göstergesini Göster",
        "Show Navigation History Buttons" => "Gezinti Geçmişi Düğmelerini Göster",
        "Show Onboarding Banner" => "Tanıtım Afişini Göster",
        "Show Other Hints" => "Diğer İpuçlarını Göster",
        "Show Parameter Hints" => "Parametre İpuçlarını Göster",
        "Show Project Items" => "Proje Öğelerini Göster",
        "Show Runnables" => "Çalıştırılabilirleri Göster",
        "Show Scrollbar" => "Kaydırma Çubuğunu Göster",
        "Show Sign In" => "Giriş Düğmesini Göster",
        "Show Signature Help After Edits" => "Düzenlemeden Sonra İmza Yardımını Göster",
        "Show Stage/Restore Buttons" => "Hazırla/Geri Yükle Düğmelerini Göster",
        "Show Tab Bar" => "Sekme Çubuğunu Göster",
        "Show Tab Bar Buttons" => "Sekme Çubuğu Düğmelerini Göster",
        "Show Turn Stats" => "Tur İstatistiklerini Göster",
        "Show Type Hints" => "Tür İpuçlarını Göster",
        "Show User Menu" => "Kullanıcı Menüsünü Göster",
        "Show User Picture" => "Kullanıcı Resmini Göster",
        "Show Value Hints" => "Değer İpuçlarını Göster",
        "Show Which-key Menu" => "which-key Menüsünü Göster",
        "Show Whitespaces" => "Boşlukları Göster",
        "Show Worktree Name" => "Worktree Adını Göster",
        "Show Wrap Guides" => "Sarma Kılavuzlarını Göster",
        "Skip Focus For Active In Search" => "Aramada Etkin Öğeye Odağı Atla",
        "Snippet Sort Order" => "Parçacık Sıralama Düzeni",
        "Soft Wrap" => "Yumuşak Sarma",
        "Sort Mode" => "Sıralama Modu",
        "Sort Order" => "Sıralama Düzeni",
        "Space Whitespace Indicator" => "Boşluk Karakteri Göstergesi",
        "Starts Open" => "Açık Başlar",
        "Stepping Granularity" => "Adımlama Ayrıntı Düzeyi",
        "Sticky" => "Yapışkan",
        "Sticky Scroll" => "Yapışkan Kaydırma",
        "Tab Close Position" => "Sekme Kapatma Konumu",
        "Tab Show Diagnostics" => "Sekmede Tanıları Göster",
        "Tab Size" => "Sekme Boyutu",
        "Tab Whitespace Indicator" => "Sekme Karakteri Göstergesi",
        "Terminal Button" => "Terminal Düğmesi",
        "Terminal Dock" => "Terminal Yerleşimi",
        "Terminal Panel Flexible Sizing" => "Terminal Paneli Esnek Boyutlandırma",
        "Terminal Thread Init Command" => "Terminal Konuşması Başlangıç Komutu",
        "Test Audio" => "Sesi Sına",
        "Text Rendering Mode" => "Metin Çizim Modu",
        "Theme Name" => "Tema Adı",
        "Thinking Display" => "Düşünme Gösterimi",
        "Thumb" => "Tutamaç",
        "Thumb Border" => "Tutamaç Kenarlığı",
        "Timeout" => "Zaman Aşımı",
        "Title Override" => "Başlık Geçersiz Kılma",
        "Toggle On Modifiers Press" => "Değiştirici Tuşa Basınca Aç/Kapat",
        "Toggle Relative Line Numbers" => "Göreli Satır Numaralarını Aç/Kapat",
        "Tool Permissions" => "Araç İzinleri",
        "Tree View" => "Ağaç Görünümü",
        "UI Font Family" => "Arayüz Yazı Tipi Ailesi",
        "UI Font Size" => "Arayüz Yazı Tipi Boyutu",
        "Unnecessary Code Fade" => "Gereksiz Kod Soluklaştırma",
        "Update Debounce" => "Güncelleme Gecikmesi",
        "Use Auto Surround" => "Otomatik Çevrelemeyi Kullan",
        "Use Autoclose" => "Otomatik Kapatmayı Kullan",
        "Use Modifier To Send" => "Göndermek İçin Değiştirici Tuş Kullan",
        "Use On Type Format" => "Yazarken Biçimlendirmeyi Kullan",
        "Use Smartcase Find" => "Akıllı Harf Duyarlı Bulmayı Kullan",
        "Use Smartcase Search" => "Akıllı Harf Duyarlı Aramayı Kullan",
        "Use System Clipboard" => "Sistem Panosunu Kullan",
        "Use System Path Prompts" => "Sistem Yol Diyaloglarını Kullan",
        "Use System Prompts" => "Sistem Diyaloglarını Kullan",
        "Use System Window Tabs" => "Sistem Pencere Sekmelerini Kullan",
        "Variables" => "Değişkenler",
        "Vertical Scroll Margin" => "Dikey Kaydırma Kenar Boşluğu",
        "Vertical Scrollbar" => "Dikey Kaydırma Çubuğu",
        "Vertical Split Direction" => "Dikey Bölme Yönü",
        "Vim/Emacs Modeline Support" => "Vim/Emacs Modeline Desteği",
        "Visibility" => "Görünürlük",
        "When Closing With No Tabs" => "Sekme Yokken Kapatılırken",
        "Whole Word" => "Tam Sözcük",
        "Window Decorations" => "Pencere Süslemeleri",
        "Word Diff Enabled" => "Sözcük Farkı Etkin",
        "Words" => "Sözcükler",
        "Words Min Length" => "Asgari Sözcük Uzunluğu",
        "Wrap Guides" => "Sarma Kılavuzları",
        "Zoomed Padding" => "Yakınlaştırılmış İç Boşluk",
        "Max Output Tokens" => "Azami Çıktı Token'ı",
        "Max Tokens" => "Azami Token",
        "Model" => "Model",
        "Prompt Format" => "İstem Biçimi",
        "Add" => "Ekle",
        "Remove Custom Agent" => "Özel Ajanı Kaldır",
        "Remove Registry Agent" => "Kayıt Defteri Ajanını Kaldır",
        "Models" => "Modeller",
        "1 tool" => "1 araç",
        "Add Local MCP Server" => "Yerel MCP Sunucusu Ekle",
        "Add Remote MCP Server" => "Uzak MCP Sunucusu Ekle",
        "MCP Server Timeout" => "MCP Sunucusu Zaman Aşımı",
        "Front-matter" => "Ön bilgi",
        "Allow" => "İzin Ver",
        "Always Allow" => "Her Zaman İzin Ver",
        "Always Confirm" => "Her Zaman Onay İste",
        "Always Deny" => "Her Zaman Reddet",
        "Deny" => "Reddet",
        "Create Directory" => "Dizin Oluştur",
        "Delete Path" => "Yolu Sil",
        "Move Path" => "Yolu Taşı",
        "Edit File" => "Dosyayı Düzenle",
        "Write File" => "Dosyaya Yaz",
        "Web Search" => "Web Araması",
        "Skill" => "Yetenek",
        "Scope" => "Kapsam",
        "Server" => "Sunucu",
        "Overridden by Organization" => "Kuruluş Tarafından Geçersiz Kılındı",

        // ── Ayarlar: açıklamalar (3. parti) ───────────────────────────────
        "Select input audio device" => "Ses giriş aygıtını seçin",
        "Select output audio device" => "Ses çıkış aygıtını seçin",
        "Send anonymized usage data like what languages you're using Zed with." => {
            "Zed'i hangi dillerle kullandığınız gibi anonimleştirilmiş kullanım verisi gönder."
        }
        "Send debug information like crash reports." => {
            "Çökme raporu gibi hata ayıklama bilgisi gönder."
        }
        "Sets the cursor blinking behavior in the terminal." => {
            "Terminaldeki imleç yanıp sönme davranışını ayarlar."
        }
        "Should the name or path be displayed first in the git view." => {
            "Git görünümünde önce adın mı yoksa yolun mu gösterileceği."
        }
        "Show a background for inlay hints." => "Satır içi ipuçları için arka plan göster.",
        "Show a badge on the terminal panel icon with the count of open terminals." => {
            "Terminal paneli simgesinde açık terminal sayısını gösteren rozet göster."
        }
        "Show a git status indicator next to file names in the project panel." => {
            "Proje panelinde dosya adlarının yanında git durum göstergesi göster."
        }
        "Show agent review buttons in the editor toolbar." => {
            "Editör araç çubuğunda ajan inceleme düğmelerini göster."
        }
        "Show author name as part of the commit information in branch picker." => {
            "Dal seçicide commit bilgisinin parçası olarak yazar adını göster."
        }
        "Show banners announcing new features in the titlebar." => {
            "Başlık çubuğunda yeni özellikleri duyuran afişleri göster."
        }
        "Show bookmarks in the gutter." => "Kenar sütununda yer imlerini göster.",
        "Show breadcrumbs." => "Gezinti yolunu göster.",
        "Show breakpoints in the gutter." => "Kenar sütununda kesme noktalarını göster.",
        "Show buffer search result indicators in the scrollbar." => {
            "Kaydırma çubuğunda tampon arama sonucu göstergelerini göster."
        }
        "Show code action button at start of buffer line." => {
            "Tampon satırının başında kod eylemi düğmesini göster."
        }
        "Show code action buttons in the editor toolbar." => {
            "Editör araç çubuğunda kod eylemi düğmelerini göster."
        }
        "Show code folding controls in the gutter." => {
            "Kenar sütununda kod katlama denetimlerini göster."
        }
        "Show commit summary as part of the inline blame." => {
            "Satır içi blame'in parçası olarak commit özetini göster."
        }
        "Show cursor positions in the scrollbar." => "Kaydırma çubuğunda imleç konumlarını göster.",
        "Show error and warning count badges next to file names in the project panel." => {
            "Proje panelinde dosya adlarının yanında hata ve uyarı sayısı rozetlerini göster."
        }
        "Show file icons in the file finder." => "Dosya bulucuda dosya simgelerini göster.",
        "Show file icons in the outline panel." => "Anahat panelinde dosya simgelerini göster.",
        "Show file icons in the project panel." => "Proje panelinde dosya simgelerini göster.",
        "Show file icons next to the Git status icon." => {
            "Git durum simgesinin yanında dosya simgelerini göster."
        }
        "Show Git diff indicators in the scrollbar." => {
            "Kaydırma çubuğunda Git fark göstergelerini göster."
        }
        "Show Git diff information in the editor." => "Editörde Git fark bilgisini göster.",
        "Show git status indicators on the branch icon in the titlebar." => {
            "Başlık çubuğundaki dal simgesinde git durum göstergelerini göster."
        }
        "Show Git status information in the editor." => "Editörde Git durum bilgisini göster.",
        "Show indent guides in the project panel." => "Proje panelinde girinti kılavuzlarını göster.",
        "Show line numbers in the gutter." => "Kenar sütununda satır numaralarını göster.",
        "Show opened editors as preview tabs." => "Açılan editörleri önizleme sekmesi olarak göster.",
        "Show padding for zoomed panes." => "Yakınlaştırılmış paneler için boşluk göster.",
        "Show pinned tabs in a separate row above unpinned tabs." => {
            "Sabitlenmiş sekmeleri, sabitlenmemişlerin üstünde ayrı bir satırda göster."
        }
        "Show quick action buttons (e.g., search, selection, editor controls, etc.)." => {
            "Hızlı eylem düğmelerini göster (arama, seçim, editör denetimleri vb.)."
        }
        "Show runnable buttons in the gutter." => "Kenar sütununda çalıştırma düğmelerini göster.",
        "Show selected symbol occurrences in the scrollbar." => {
            "Kaydırma çubuğunda seçili simgenin geçtiği yerleri göster."
        }
        "Show selected text occurrences in the scrollbar." => {
            "Kaydırma çubuğunda seçili metnin geçtiği yerleri göster."
        }
        "Show the active language button in the status bar." => {
            "Durum çubuğunda etkin dil düğmesini göster."
        }
        "Show the active line endings button in the status bar." => {
            "Durum çubuğunda etkin satır sonu düğmesini göster."
        }
        "Show the avatar of the author of the commit." => "Commit yazarının avatarını göster.",
        "Show the branch name button in the titlebar." => {
            "Başlık çubuğunda dal adı düğmesini göster."
        }
        "Show the collaboration panel button in the status bar." => {
            "Durum çubuğunda işbirliği paneli düğmesini göster."
        }
        "Show the cursor position button in the status bar." => {
            "Durum çubuğunda imleç konumu düğmesini göster."
        }
        "Show the debugger button in the status bar." => {
            "Durum çubuğunda hata ayıklayıcı düğmesini göster."
        }
        "Show the file icon for a tab." => "Sekme için dosya simgesini göster.",
        "Show the Git file status on a tab item." => "Sekme öğesinde Git dosya durumunu göster.",
        "Show the Git panel button in the status bar." => {
            "Durum çubuğunda Git paneli düğmesini göster."
        }
        "Show the Git status in the outline panel." => "Anahat panelinde Git durumunu göster.",
        "Show the Git status in the project panel." => "Proje panelinde Git durumunu göster.",
        "Show the informational hover box when moving the mouse over symbols in the editor." => {
            "Editörde simgelerin üzerine gelindiğinde bilgi kutusunu göster."
        }
        "Show the menus in the titlebar." => "Başlık çubuğunda menüleri göster.",
        "Show the name of the active file in the status bar." => {
            "Durum çubuğunda etkin dosyanın adını göster."
        }
        "Show the navigation history buttons in the tab bar." => {
            "Sekme çubuğunda gezinti geçmişi düğmelerini göster."
        }
        "Show the outline panel button in the status bar." => {
            "Durum çubuğunda anahat paneli düğmesini göster."
        }
        "Show the project diagnostics button in the status bar." => {
            "Durum çubuğunda proje tanıları düğmesini göster."
        }
        "Show the project host and name in the titlebar." => {
            "Başlık çubuğunda proje sunucusunu ve adını göster."
        }
        "Show the project panel button in the status bar." => {
            "Durum çubuğunda proje paneli düğmesini göster."
        }
        "Show the project search button in the status bar." => {
            "Durum çubuğunda proje arama düğmesini göster."
        }
        "Show the scrollbar in the project panel." => "Proje panelinde kaydırma çubuğunu göster.",
        "Show the selections menu in the editor toolbar." => {
            "Editör araç çubuğunda seçimler menüsünü göster."
        }
        "Show the sign in button in the titlebar." => "Başlık çubuğunda giriş düğmesini göster.",
        "Show the signature help pop-up after completions or bracket pairs are inserted." => {
            "Tamamlama veya parantez çifti eklendikten sonra imza yardımı kutusunu göster."
        }
        "Show the tab bar buttons (New, Split Pane, Zoom)." => {
            "Sekme çubuğu düğmelerini göster (Yeni, Paneyi Böl, Yakınlaştır)."
        }
        "Show the tab bar in the editor." => "Editörde sekme çubuğunu göster.",
        "Show the terminal button in the status bar." => "Durum çubuğunda terminal düğmesini göster.",
        "Show the user menu button in the titlebar." => {
            "Başlık çubuğunda kullanıcı menüsü düğmesini göster."
        }
        "Show the worktree name button in the titlebar." => {
            "Başlık çubuğunda worktree adı düğmesini göster."
        }
        "Show user picture in the titlebar." => "Başlık çubuğunda kullanıcı resmini göster.",
        "Show voting thumbs up/down icon buttons for feedback on agent edits." => {
            "Ajan düzenlemelerine geri bildirim için beğen/beğenme düğmelerini göster."
        }
        "Show wrap guides (vertical rulers)." => "Sarma kılavuzlarını göster (dikey cetveller).",
        "Show wrap guides in the editor." => "Editörde sarma kılavuzlarını göster.",
        "Size of the border surrounding the active pane." => "Etkin paneyi çevreleyen kenarlığın boyutu.",
        "Sort order for entries in the project panel." => "Proje panelindeki öğelerin sıralama düzeni.",
        "Spacing between worktree entries in the project panel." => {
            "Proje panelindeki worktree öğeleri arasındaki boşluk."
        }
        "The amount of padding between the end of the source line and the start of the inline diagnostic." => {
            "Kaynak satırın sonu ile satır içi tanının başlangıcı arasındaki boşluk miktarı."
        }
        "The arguments to pass to the shell program." => "Kabuk programına geçirilecek argümanlar.",
        "The column at which to soft-wrap lines, for buffers where soft-wrap is enabled." => {
            "Yumuşak sarmanın etkin olduğu tamponlarda satırların sarılacağı sütun."
        }
        "The custom set of icons Zed will associate with files and directories." => {
            "Zed'in dosya ve dizinlerle ilişkilendireceği özel simge kümesi."
        }
        "The debounce delay before querying highlights from the language." => {
            "Dilden vurgular sorgulanmadan önceki gecikme."
        }
        "The default mode when Vim starts." => "Vim başladığındaki varsayılan mod.",
        "The delay after which the inline blame information is shown." => {
            "Satır içi blame bilgisinin gösterileceği gecikme."
        }
        "The delay in milliseconds to show inline diagnostics after the last diagnostic update." => {
            "Son tanı güncellemesinden sonra satır içi tanıların gösterileceği milisaniye cinsinden gecikme."
        }
        "The directory path to use (will be shell expanded)." => {
            "Kullanılacak dizin yolu (kabuk tarafından genişletilir)."
        }
        "The dock position of the debug panel." => "Hata ayıklama panelinin yerleşim konumu.",
        "The font fallbacks to use for rendering in text buffers." => {
            "Metin tamponlarında çizim için kullanılacak yedek yazı tipleri."
        }
        "The font fallbacks to use for rendering in the UI." => {
            "Arayüzde çizim için kullanılacak yedek yazı tipleri."
        }
        "The icon theme to use when mode is set to dark, or when mode is set to system and it is in dark mode." => {
            "Mod koyu olduğunda ya da sistem olup sistem koyu moddayken kullanılacak simge teması."
        }
        "The icon theme to use when mode is set to light, or when mode is set to system and it is in light mode." => {
            "Mod açık olduğunda ya da sistem olup sistem açık moddayken kullanılacak simge teması."
        }
        "The list of language servers to use (or disable) for this language." => {
            "Bu dil için kullanılacak (veya devre dışı bırakılacak) dil sunucuları listesi."
        }
        "The minimum APCA perceptual contrast between foreground and background colors (0-106)." => {
            "Ön plan ve arka plan renkleri arasındaki asgari APCA algısal karşıtlığı (0-106)."
        }
        "The minimum APCA perceptual contrast to maintain when rendering text over highlight backgrounds." => {
            "Vurgu arka planları üzerine metin çizerken korunacak asgari APCA algısal karşıtlığı."
        }
        "The minimum column at which to display inline diagnostics." => {
            "Satır içi tanıların gösterileceği asgari sütun."
        }
        "The minimum column number at which to show the inline blame information." => {
            "Satır içi blame bilgisinin gösterileceği asgari sütun numarası."
        }
        "The minimum width (in columns) at which the split diff view is used. When the editor is narrower, the diff view automatically switches to unified mode. Set to 0 to disable." => {
            "Bölünmüş fark görünümünün kullanılacağı asgari genişlik (sütun). Editör daha darsa fark görünümü otomatik olarak birleşik moda geçer. Devre dışı bırakmak için 0 yapın."
        }
        "The multiplier for scrolling in the terminal with the mouse wheel" => {
            "Terminalde fare tekerleğiyle kaydırma çarpanı"
        }
        "The name of a base set of key bindings to use." => {
            "Kullanılacak temel kısayol kümesinin adı."
        }
        "The name of your selected icon theme." => "Seçtiğiniz simge temasının adı.",
        "The name of your selected theme." => "Seçtiğiniz temanın adı.",
        "The number of characters to keep on either side when scrolling with the mouse." => {
            "Fareyle kaydırırken her iki yanda tutulacak karakter sayısı."
        }
        "The number of lines to keep above/below the cursor when auto-scrolling." => {
            "Otomatik kaydırmada imlecin üstünde/altında tutulacak satır sayısı."
        }
        "The OpenType features to enable for rendering in text buffers." => {
            "Metin tamponlarında çizim için etkinleştirilecek OpenType özellikleri."
        }
        "The OpenType features to enable for rendering in UI elements." => {
            "Arayüz öğelerinde çizim için etkinleştirilecek OpenType özellikleri."
        }
        "The proxy to use for network requests." => "Ağ istekleri için kullanılacak vekil sunucu.",
        "The shell program to run." => "Çalıştırılacak kabuk programı.",
        "The shell program to use." => "Kullanılacak kabuk programı.",
        "The text rendering mode to use." => "Kullanılacak metin çizim modu.",
        "The theme to use when mode is set to dark, or when mode is set to system and it is in dark mode." => {
            "Mod koyu olduğunda ya da sistem olup sistem koyu moddayken kullanılacak tema."
        }
        "The theme to use when mode is set to light, or when mode is set to system and it is in light mode." => {
            "Mod açık olduğunda ya da sistem olup sistem açık moddayken kullanılacak tema."
        }
        "The unit for image file sizes." => "Görsel dosya boyutları için birim.",
        "The URL of the Zed server to connect to." => "Bağlanılacak Zed sunucusunun URL'si.",
        "The width of the active indent guide in pixels, between 1 and 10." => {
            "Etkin girinti kılavuzunun piksel cinsinden genişliği (1 ile 10 arası)."
        }
        "The width of the indent guides in pixels, between 1 and 10." => {
            "Girinti kılavuzlarının piksel cinsinden genişliği (1 ile 10 arası)."
        }
        "Time in milliseconds until timeout error when connecting to a TCP debug adapter." => {
            "TCP hata ayıklama bağdaştırıcısına bağlanırken zaman aşımı hatasına kadar geçecek milisaniye."
        }
        "Time to wait in milliseconds before hiding the hover popover after the mouse moves away." => {
            "Fare uzaklaştıktan sonra bilgi kutusunu gizlemeden önce beklenecek milisaniye."
        }
        "Time to wait in milliseconds before showing the informational hover box." => {
            "Bilgi kutusunu göstermeden önce beklenecek milisaniye."
        }
        "Toggle relative line numbers in Vim mode." => "Vim modunda göreli satır numaralarını aç/kapat.",
        "Toggles inlay hints (hides or shows) when the user presses the modifiers specified." => {
            "Belirtilen değiştirici tuşlara basıldığında satır içi ipuçlarını gizler veya gösterir."
        }
        "Use gitignored files when searching." => "Arama yaparken gitignore'daki dosyaları da kullan.",
        "Use LSP tasks over Zed language extension tasks." => {
            "Zed dil uzantısı görevleri yerine LSP görevlerini kullan."
        }
        "Use native OS dialogs for confirmations." => "Onaylar için işletim sistemi diyaloglarını kullan.",
        "Use native OS dialogs for 'Open' and 'Save As'." => {
            "'Aç' ve 'Farklı Kaydet' için işletim sistemi diyaloglarını kullan."
        }
        "Use regex search by default in Vim search." => {
            "Vim aramasında varsayılan olarak düzenli ifade kullan."
        }
        "Use regex search by default." => "Varsayılan olarak düzenli ifade araması kullan.",
        "Visible character used to render space characters when show_whitespaces is enabled (default: \"•\")" => {
            "show_whitespaces etkinken boşluk karakterlerini çizmek için kullanılan görünür karakter (varsayılan: \"•\")"
        }
        "Visible character used to render tab characters when show_whitespaces is enabled (default: \"→\")" => {
            "show_whitespaces etkinken sekme karakterlerini çizmek için kullanılan görünür karakter (varsayılan: \"→\")"
        }
        "What shell to use when opening a terminal." => "Terminal açılırken kullanılacak kabuk.",
        "What to do after closing the current tab." => "Geçerli sekme kapatıldıktan sonra ne yapılacağı.",
        "What to do when multibuffer is double-clicked in some of its excerpts." => {
            "Çoklu tamponun alıntılarından birine çift tıklandığında ne yapılacağı."
        }
        "What to restore from the previous session when opening Zed." => {
            "Zed açılırken önceki oturumdan nelerin geri yükleneceği."
        }
        "What working directory to use when launching the terminal." => {
            "Terminal başlatılırken hangi çalışma dizininin kullanılacağı."
        }
        "When auto compaction runs. A percentage string like \"90%\" is measured against the context window. A positive integer is the number of used tokens to compact after. A negative integer is the number of tokens remaining in the context window before compacting." => {
            "Otomatik sıkıştırmanın ne zaman çalışacağı. \"90%\" gibi bir yüzde, bağlam penceresine göre ölçülür. Pozitif tam sayı, sonrasında sıkıştırılacak kullanılmış token sayısıdır. Negatif tam sayı ise sıkıştırmadan önce bağlam penceresinde kalan token sayısıdır."
        }
        "When enabled, agent edits will also be displayed in single-file buffers for review." => {
            "Etkinken ajan düzenlemeleri inceleme için tek dosyalı tamponlarda da gösterilir."
        }
        "When enabled, the :substitute command replaces all matches in a line by default. The 'g' flag then toggles this behavior." => {
            "Etkinken :substitute komutu varsayılan olarak satırdaki tüm eşleşmeleri değiştirir. 'g' bayrağı bu davranışı tersine çevirir."
        }
        "When enabled, use folding ranges from the language server instead of indent-based folding." => {
            "Etkinken girintiye dayalı katlama yerine dil sunucusunun katlama aralıklarını kullanır."
        }
        "When enabled, use the language server's document symbols for outlines and breadcrumbs instead of tree-sitter." => {
            "Etkinken anahat ve gezinti yolu için tree-sitter yerine dil sunucusunun belge simgelerini kullanır."
        }
        "When false, forcefully disables the horizontal scrollbar." => {
            "False olduğunda yatay kaydırma çubuğunu tamamen devre dışı bırakır."
        }
        "When false, forcefully disables the vertical scrollbar." => {
            "False olduğunda dikey kaydırma çubuğunu tamamen devre dışı bırakır."
        }
        "When fetching LSP completions, determines how long to wait for a response of a particular server (set to 0 to wait indefinitely)." => {
            "LSP tamamlamaları alınırken belirli bir sunucunun yanıtı için ne kadar bekleneceğini belirler (süresiz beklemek için 0 yapın)."
        }
        "When opening Zed, avoid Restricted Mode by auto-trusting all projects, enabling use of all features without having to give permission to each new project." => {
            "Zed açılırken tüm projelere otomatik güvenerek Kısıtlı Moddan kaçınır; her yeni proje için izin vermeden tüm özellikler kullanılabilir."
        }
        "When to auto save buffer changes." => "Tampon değişikliklerinin ne zaman otomatik kaydedileceği.",
        "When to hide the mouse cursor." => "Fare imlecinin ne zaman gizleneceği.",
        "When to play a sound when the agent has either completed its response, or needs user input." => {
            "Ajan yanıtını tamamladığında veya kullanıcı girdisine ihtiyaç duyduğunda ne zaman ses çalınacağı."
        }
        "When to populate a new search's query based on the text under the cursor." => {
            "Yeni bir aramanın sorgusunun imleç altındaki metinden ne zaman doldurulacağı."
        }
        "When to scan content of linked directories" => {
            "Bağlantılı dizinlerin içeriğinin ne zaman taranacağı"
        }
        "When to show edit predictions previews in buffer. The eager mode displays them inline, while the subtle mode displays them only when holding a modifier key." => {
            "Düzenleme tahmini önizlemelerinin tamponda ne zaman gösterileceği. Eager modu satır içinde gösterir; subtle modu yalnızca bir değiştirici tuş basılıyken gösterir."
        }
        "When to show indent guides in the outline panel." => {
            "Anahat panelinde girinti kılavuzlarının ne zaman gösterileceği."
        }
        "When to show the minimap in the editor." => "Editörde mini haritanın ne zaman gösterileceği.",
        "When to show the minimap thumb." => "Mini harita tutamacının ne zaman gösterileceği.",
        "When to show the scrollbar in the completion menu." => {
            "Tamamlama menüsünde kaydırma çubuğunun ne zaman gösterileceği."
        }
        "When to show the scrollbar in the editor." => {
            "Editörde kaydırma çubuğunun ne zaman gösterileceği."
        }
        "When to show the scrollbar in the terminal." => {
            "Terminalde kaydırma çubuğunun ne zaman gösterileceği."
        }
        "Where to dock the agent panel." => "Ajan panelinin nereye yerleştirileceği.",
        "Where to dock the collaboration panel." => "İşbirliği panelinin nereye yerleştirileceği.",
        "Where to dock the Git panel." => "Git panelinin nereye yerleştirileceği.",
        "Where to dock the outline panel." => "Anahat panelinin nereye yerleştirileceği.",
        "Where to dock the project panel." => "Proje panelinin nereye yerleştirileceği.",
        "Where to dock the terminal panel." => "Terminal panelinin nereye yerleştirileceği.",
        "Where to render Git blame when it is enabled." => {
            "Git blame etkinken nerede çizileceği."
        }
        "Where to show LSP results that can contain multiple locations (Go to Definition, Go to Implementation, Find All References)." => {
            "Birden fazla konum içerebilen LSP sonuçlarının nerede gösterileceği (Tanıma Git, Uygulamaya Git, Tüm Referansları Bul)."
        }
        "Where to show notifications when the agent has completed its response or needs confirmation before running a tool action." => {
            "Ajan yanıtını tamamladığında veya bir araç eylemi öncesi onay gerektiğinde bildirimlerin nerede gösterileceği."
        }
        "Where to show the minimap in the editor." => "Editörde mini haritanın nerede gösterileceği.",
        "Whether alternate scroll mode is active by default (converts mouse scroll to arrow keys in apps like Vim)." => {
            "Alternatif kaydırma modunun varsayılan olarak etkin olup olmadığı (Vim gibi uygulamalarda fare kaydırmasını ok tuşlarına çevirir)."
        }
        "Whether and how to display code lenses from language servers." => {
            "Dil sunucularından gelen kod merceklerinin gösterilip gösterilmeyeceği ve nasıl gösterileceği."
        }
        "Whether breakpoints should be reused across Zed sessions." => {
            "Kesme noktalarının Zed oturumları arasında korunup korunmayacağı."
        }
        "Whether clicking the stop button on a running terminal tool should also cancel the agent's generation. Note that this only applies to the stop button, not to ctrl+c inside the terminal." => {
            "Çalışan bir terminal aracındaki durdur düğmesine basmanın ajanın üretimini de iptal edip etmeyeceği. Bu yalnızca durdur düğmesi için geçerlidir, terminaldeki ctrl+c için değil."
        }
        "Whether cmd-click (ctrl-click on Linux and Windows) opens hyperlinks even when the terminal application has enabled mouse reporting. When disabled, these clicks are forwarded to the application; links can still be opened with shift-cmd-click." => {
            "Terminal uygulaması fare raporlamayı açmışken bile cmd-tık (Linux ve Windows'ta ctrl-tık) bağlantıları açsın mı. Kapalıyken bu tıklamalar uygulamaya iletilir; bağlantılar yine shift-cmd-tık ile açılabilir."
        }
        "Whether edit predictions are shown in normal mode. By default, edit predictions are only shown in insert and replace modes." => {
            "Düzenleme tahminlerinin normal modda gösterilip gösterilmeyeceği. Varsayılan olarak yalnızca ekleme ve değiştirme modlarında gösterilir."
        }
        "Whether indentation of pasted content should be adjusted based on the context." => {
            "Yapıştırılan içeriğin girintisinin bağlama göre ayarlanıp ayarlanmayacağı."
        }
        "Whether newly opened file diffs show the full file instead of changes only." => {
            "Yeni açılan dosya farklarının yalnızca değişiklikler yerine tüm dosyayı gösterip göstermeyeceği."
        }
        "Whether or not to automatically check for updates." => {
            "Güncellemelerin otomatik denetlenip denetlenmeyeceği."
        }
        "Whether or not to debounce inlay hints updates after buffer edits (set to 0 to disable debouncing)." => {
            "Tampon düzenlemelerinden sonra satır içi ipucu güncellemelerinin geciktirilip geciktirilmeyeceği (kapatmak için 0 yapın)."
        }
        "Whether or not to debounce inlay hints updates after buffer scrolls (set to 0 to disable debouncing)." => {
            "Tampon kaydırmalarından sonra satır içi ipucu güncellemelerinin geciktirilip geciktirilmeyeceği (kapatmak için 0 yapın)."
        }
        "Whether or not to ensure there's a single newline at the end of a buffer when saving it." => {
            "Kaydederken tamponun sonunda tek bir satır sonu bulunmasının sağlanıp sağlanmayacağı."
        }
        "Whether or not to remove any trailing whitespace from lines of a buffer before saving it." => {
            "Kaydetmeden önce tampon satırlarının sonundaki boşlukların silinip silinmeyeceği."
        }
        "Whether or not to restore unsaved buffers on restart." => {
            "Yeniden başlatmada kaydedilmemiş tamponların geri yüklenip yüklenmeyeceği."
        }
        "Whether or not to show Git blame data for the currently focused line." => {
            "Odaklanılan satır için Git blame verisinin gösterilip gösterilmeyeceği."
        }
        "Whether other hints should be shown." => "Diğer ipuçlarının gösterilip gösterilmeyeceği.",
        "Whether parameter hints should be shown." => "Parametre ipuçlarının gösterilip gösterilmeyeceği.",
        "Whether selecting text in the terminal automatically copies to the system clipboard." => {
            "Terminalde metin seçmenin sistem panosuna otomatik kopyalayıp kopyalamayacağı."
        }
        "Whether tasks are enabled for this language." => {
            "Bu dil için görevlerin etkin olup olmadığı."
        }
        "Whether the agent panel should use flexible (proportional) sizing when docked to the left or right." => {
            "Ajan paneli sola veya sağa yerleştirildiğinde esnek (oransal) boyutlandırma kullanıp kullanmayacağı."
        }
        "Whether the cursor blinks in the editor." => "Editörde imlecin yanıp sönüp sönmeyeceği.",
        "Whether the editor search results will loop." => {
            "Editör arama sonuçlarının başa dönüp dönmeyeceği."
        }
        "Whether the editor will scroll beyond the last line." => {
            "Editörün son satırın ötesine kaydırılıp kaydırılamayacağı."
        }
        "Whether the file finder should skip focus for the active file in search results." => {
            "Dosya bulucunun arama sonuçlarında etkin dosyaya odaklanmayı atlayıp atlamayacağı."
        }
        "Whether the hover popover sticks when the mouse moves toward it, allowing interaction with its contents." => {
            "Fare üzerine gelirken bilgi kutusunun açık kalıp içeriğiyle etkileşime izin verip vermeyeceği."
        }
        "Whether the microphone should be muted when joining a channel or a call." => {
            "Bir kanala veya görüşmeye katılırken mikrofonun susturulup susturulmayacağı."
        }
        "Whether the option key behaves as the meta key." => {
            "Option tuşunun meta tuşu gibi davranıp davranmayacağı."
        }
        "Whether the project panel should open on startup." => {
            "Proje panelinin açılışta açılıp açılmayacağı."
        }
        "Whether the terminal panel should use flexible (proportional) sizing when docked to the left or right." => {
            "Terminal paneli sola veya sağa yerleştirildiğinde esnek (oransal) boyutlandırma kullanıp kullanmayacağı."
        }
        "Whether the text selection should have rounded corners." => {
            "Metin seçiminin köşelerinin yuvarlatılıp yuvarlatılmayacağı."
        }
        "Whether to align detail text in code completions context menus left or right." => {
            "Kod tamamlama menülerindeki ayrıntı metninin sola mı sağa mı hizalanacağı."
        }
        "Whether to allow horizontal scrolling in the project panel. When disabled, the view is always locked to the leftmost position and long file names are clipped." => {
            "Proje panelinde yatay kaydırmaya izin verilip verilmeyeceği. Kapalıyken görünüm hep en soldadır ve uzun dosya adları kırpılır."
        }
        "Whether to always use cmd-enter (or ctrl-enter on Linux or Windows) to send messages." => {
            "Mesaj göndermek için her zaman cmd-enter (Linux veya Windows'ta ctrl-enter) kullanılıp kullanılmayacağı."
        }
        "Whether to automatically close JSX tags." => "JSX etiketlerinin otomatik kapatılıp kapatılmayacağı.",
        "Whether to automatically enable case-sensitive search based on the search query." => {
            "Arama sorgusuna göre büyük/küçük harf duyarlı aramanın otomatik açılıp açılmayacağı."
        }
        "Whether to automatically open files after pasting or duplicating them." => {
            "Dosyaların yapıştırıldıktan veya çoğaltıldıktan sonra otomatik açılıp açılmayacağı."
        }
        "Whether to automatically open files dropped from external sources." => {
            "Dış kaynaklardan bırakılan dosyaların otomatik açılıp açılmayacağı."
        }
        "Whether to automatically open newly created files in the editor." => {
            "Yeni oluşturulan dosyaların editörde otomatik açılıp açılmayacağı."
        }
        "Whether to automatically surround text with characters for you. For example, when you select text and type '(', Zed will automatically surround text with ()." => {
            "Metnin sizin için otomatik çevrelenip çevrelenmeyeceği. Örneğin metni seçip '(' yazdığınızda Zed metni () ile çevreler."
        }
        "Whether to automatically type closing characters for you. For example, when you type '(', Zed will automatically add a closing ')' at the correct position." => {
            "Kapatma karakterlerinin sizin için otomatik yazılıp yazılmayacağı. Örneğin '(' yazdığınızda Zed doğru konuma ')' ekler."
        }
        "Whether to center the current match in the editor" => {
            "Geçerli eşleşmenin editörde ortalanıp ortalanmayacağı"
        }
        "Whether to change focus to a pane when the mouse hovers over it." => {
            "Fare bir panenin üzerine geldiğinde odağın oraya geçip geçmeyeceği."
        }
        "Whether to collapse untracked files in the diff panel." => {
            "Fark panelinde izlenmeyen dosyaların daraltılıp daraltılmayacağı."
        }
        "Whether to colorize brackets in the editor." => {
            "Editörde parantezlerin renklendirilip renklendirilmeyeceği."
        }
        "Whether to constrain the agent panel content to a maximum width, centering it when the panel is wider, for optimal readability." => {
            "En iyi okunabilirlik için ajan paneli içeriğinin azami genişlikle sınırlanıp panel daha genişken ortalanıp ortalanmayacağı."
        }
        "Whether to constrain the markdown preview content to a maximum width, centering it when the pane is wider, for optimal readability." => {
            "En iyi okunabilirlik için markdown önizleme içeriğinin azami genişlikle sınırlanıp pane daha genişken ortalanıp ortalanmayacağı."
        }
        "Whether to disable all AI features in Zed." => {
            "Zed'deki tüm yapay zekâ özelliklerinin devre dışı bırakılıp bırakılmayacağı."
        }
        "Whether to display inline and alongside documentation for items in the completions menu." => {
            "Tamamlama menüsündeki öğeler için belgelerin satır içinde ve yanında gösterilip gösterilmeyeceği."
        }
        "Whether to enable drag-and-drop operations in the project panel." => {
            "Proje panelinde sürükle-bırak işlemlerinin etkinleştirilip etkinleştirilmeyeceği."
        }
        "Whether to enable word diff highlighting in the editor. When enabled, changed words within modified lines are highlighted to show exactly what changed." => {
            "Editörde sözcük farkı vurgusunun etkinleştirilip etkinleştirilmeyeceği. Etkinken değişen satırlardaki değişen sözcükler tam olarak neyin değiştiğini göstermek için vurgulanır."
        }
        "Whether to fetch LSP completions or not." => "LSP tamamlamalarının alınıp alınmayacağı.",
        "Whether to fold directories automatically and show compact folders when a directory has only one subdirectory inside." => {
            "Bir dizinde yalnızca tek bir alt dizin varken dizinlerin otomatik katlanıp derli toplu gösterilip gösterilmeyeceği."
        }
        "Whether to fold directories automatically when a directory contains only one subdirectory." => {
            "Bir dizin yalnızca tek bir alt dizin içerdiğinde dizinlerin otomatik katlanıp katlanmayacağı."
        }
        "Whether to follow-up empty Go to definition responses from the language server." => {
            "Dil sunucusundan gelen boş Tanıma Git yanıtlarının izlenip izlenmeyeceği."
        }
        "Whether to format DAP messages when adding them to debug adapter logger." => {
            "DAP mesajlarının hata ayıklama bağdaştırıcısı günlüğüne eklenirken biçimlendirilip biçimlendirilmeyeceği."
        }
        "Whether to have edit cards in the agent panel expanded, showing a Preview of the diff." => {
            "Ajan panelindeki düzenleme kartlarının açık olup farkın önizlemesini gösterip göstermeyeceği."
        }
        "Whether to have terminal cards in the agent panel expanded, showing the whole command output." => {
            "Ajan panelindeki terminal kartlarının açık olup tüm komut çıktısını gösterip göstermeyeceği."
        }
        "Whether to hide the gitignore entries in the project panel." => {
            "Proje panelinde gitignore öğelerinin gizlenip gizlenmeyeceği."
        }
        "Whether to hide the hidden entries in the project panel." => {
            "Proje panelinde gizli öğelerin gizlenip gizlenmeyeceği."
        }
        "Whether to hide the root entry when only one folder is open in the window." => {
            "Pencerede yalnızca bir klasör açıkken kök öğenin gizlenip gizlenmeyeceği."
        }
        "Whether to indent lines using tab characters, as opposed to multiple spaces." => {
            "Satırların birden çok boşluk yerine sekme karakteriyle girintilenip girintilenmeyeceği."
        }
        "Whether to keep tabs in preview mode when code navigation is used to navigate away from them. If `enable_preview_file_from_code_navigation` or `enable_preview_multibuffer_from_code_navigation` is also true, the new tab may replace the existing one." => {
            "Kod gezinmesiyle sekmelerden ayrılırken sekmelerin önizleme modunda kalıp kalmayacağı. `enable_preview_file_from_code_navigation` veya `enable_preview_multibuffer_from_code_navigation` da true ise yeni sekme mevcut olanın yerini alabilir."
        }
        "Whether to keep the text selection after copying it to the clipboard." => {
            "Metin panoya kopyalandıktan sonra seçimin korunup korunmayacağı."
        }
        "Whether to log messages between active debug adapters and Zed." => {
            "Etkin hata ayıklama bağdaştırıcıları ile Zed arasındaki mesajların günlüğe yazılıp yazılmayacağı."
        }
        "Whether to open tabs in preview mode when code navigation is used to open a multibuffer." => {
            "Kod gezinmesiyle çoklu tampon açılırken sekmelerin önizleme modunda açılıp açılmayacağı."
        }
        "Whether to open tabs in preview mode when code navigation is used to open a single file." => {
            "Kod gezinmesiyle tek dosya açılırken sekmelerin önizleme modunda açılıp açılmayacağı."
        }
        "Whether to open tabs in preview mode when opened from a multibuffer." => {
            "Çoklu tampondan açılırken sekmelerin önizleme modunda açılıp açılmayacağı."
        }
        "Whether to open tabs in preview mode when opened from the project panel with a single click." => {
            "Proje panelinden tek tıkla açılırken sekmelerin önizleme modunda açılıp açılmayacağı."
        }
        "Whether to open tabs in preview mode when selected from the file finder." => {
            "Dosya bulucudan seçilirken sekmelerin önizleme modunda açılıp açılmayacağı."
        }
        "Whether to perform linked edits of associated ranges, if the LS supports it. For example, when editing opening <html> tag, the contents of the closing </html> tag will be edited as well." => {
            "Dil sunucusu destekliyorsa ilişkili aralıklarda bağlı düzenleme yapılıp yapılmayacağı. Örneğin açılış <html> etiketi düzenlenirken kapanış </html> etiketi de düzenlenir."
        }
        "Whether to play a sound when the BEL character (`\\a`, `0x07`) is printed" => {
            "BEL karakteri (`\\a`, `0x07`) yazdırıldığında ses çalınıp çalınmayacağı"
        }
        "Whether to pop the completions menu while typing in an editor without explicitly requesting it." => {
            "Editörde yazarken açıkça istenmeden tamamlama menüsünün açılıp açılmayacağı."
        }
        "Whether to pull for language server-powered diagnostics or not." => {
            "Dil sunucusu destekli tanıların çekilip çekilmeyeceği."
        }
        "Whether to reduce non-essential motion, such as loading spinners, by rendering them in a static state." => {
            "Yükleme göstergeleri gibi zorunlu olmayan hareketlerin durağan çizilerek azaltılıp azaltılmayacağı."
        }
        "Whether to reveal entries in the project panel automatically when a corresponding project entry becomes active." => {
            "İlgili proje öğesi etkinleştiğinde proje panelinde öğelerin otomatik gösterilip gösterilmeyeceği."
        }
        "Whether to reveal when a corresponding outline entry becomes active." => {
            "İlgili anahat öğesi etkinleştiğinde gösterilip gösterilmeyeceği."
        }
        "Whether to scroll when clicking near the edge of the visible text area." => {
            "Görünür metin alanının kenarına yakın tıklandığında kaydırılıp kaydırılmayacağı."
        }
        "Whether to show a badge on the git panel icon with the count of uncommitted changes." => {
            "Git paneli simgesinde commit edilmemiş değişiklik sayısını gösteren rozetin gösterilip gösterilmeyeceği."
        }
        "Whether to show diagnostics inline or not." => "Tanıların satır içinde gösterilip gösterilmeyeceği.",
        "Whether to show folder icons or chevrons for directories in the git panel." => {
            "Git panelinde dizinler için klasör simgesi mi yoksa ok işareti mi gösterileceği."
        }
        "Whether to show folder icons or chevrons for directories in the outline panel." => {
            "Anahat panelinde dizinler için klasör simgesi mi yoksa ok işareti mi gösterileceği."
        }
        "Whether to show folder icons or chevrons for directories in the project panel." => {
            "Proje panelinde dizinler için klasör simgesi mi yoksa ok işareti mi gösterileceği."
        }
        "Whether to show folder names with bold text in the project panel." => {
            "Proje panelinde klasör adlarının kalın yazıyla gösterilip gösterilmeyeceği."
        }
        "Whether to show tabs and spaces in the editor." => {
            "Editörde sekme ve boşlukların gösterilip gösterilmeyeceği."
        }
        "Whether to show the addition/deletion change count next to each file in the Git panel." => {
            "Git panelinde her dosyanın yanında ekleme/silme sayısının gösterilip gösterilmeyeceği."
        }
        "Whether to show the agent panel button in the status bar." => {
            "Durum çubuğunda ajan paneli düğmesinin gösterilip gösterilmeyeceği."
        }
        "Whether to show the merge conflict indicator in the status bar that offers to resolve conflicts using the agent." => {
            "Çakışmaları ajanla çözmeyi öneren birleştirme çakışması göstergesinin durum çubuğunda gösterilip gösterilmeyeceği."
        }
        "Whether to show the stage and restore buttons on diff hunks." => {
            "Fark parçalarında hazırla ve geri yükle düğmelerinin gösterilip gösterilmeyeceği."
        }
        "Whether to show turn statistics like elapsed time during generation and final turn duration." => {
            "Üretim sırasında geçen süre ve nihai tur süresi gibi tur istatistiklerinin gösterilip gösterilmeyeceği."
        }
        "Whether to show warnings or not by default." => {
            "Varsayılan olarak uyarıların gösterilip gösterilmeyeceği."
        }
        "Whether to sort file and folder names case-sensitively in the project panel." => {
            "Proje panelinde dosya ve klasör adlarının büyük/küçük harfe duyarlı sıralanıp sıralanmayacağı."
        }
        "Whether to start a new line with a comment when a previous line is a comment as well." => {
            "Önceki satır da yorumsa yeni satırın yorumla başlatılıp başlatılmayacağı."
        }
        "Whether to stick parent directories at top of the project panel." => {
            "Üst dizinlerin proje panelinin en üstüne yapıştırılıp yapıştırılmayacağı."
        }
        "Whether to stick scopes to the top of the editor" => {
            "Kapsamların editörün üstüne yapıştırılıp yapıştırılmayacağı"
        }
        "Whether to use additional LSP queries to format (and amend) the code after every \"trigger\" symbol input, defined by LSP server capabilities" => {
            "LSP sunucu yeteneklerinde tanımlı her \"tetikleyici\" simge girişinden sonra kodu biçimlendirmek (ve düzeltmek) için ek LSP sorgularının kullanılıp kullanılmayacağı"
        }
        "Whether to use language servers to provide code intelligence." => {
            "Kod zekâsı sağlamak için dil sunucularının kullanılıp kullanılmayacağı."
        }
        "Whether to zoom the editor font size with the mouse wheel while holding the primary modifier key." => {
            "Birincil değiştirici tuş basılıyken fare tekerleğiyle editör yazı tipi boyutunun yakınlaştırılıp yakınlaştırılmayacağı."
        }
        "Whether type hints should be shown." => "Tür ipuçlarının gösterilip gösterilmeyeceği.",
        "Whether your current project should be shared when joining an empty channel." => {
            "Boş bir kanala katılırken geçerli projenizin paylaşılıp paylaşılmayacağı."
        }
        "Which diagnostic indicators to show in the scrollbar." => {
            "Kaydırma çubuğunda hangi tanı göstergelerinin gösterileceği."
        }
        "Which files containing diagnostic errors/warnings to mark in the project panel." => {
            "Tanı hatası/uyarısı içeren hangi dosyaların proje panelinde işaretleneceği."
        }
        "Which files containing diagnostic errors/warnings to mark in the tabs." => {
            "Tanı hatası/uyarısı içeren hangi dosyaların sekmelerde işaretleneceği."
        }
        "Which level to use to filter out diagnostics displayed in the editor." => {
            "Editörde gösterilen tanıları hangi düzeye göre süzüleceği."
        }
        "Which settings should be activated only in Preview build of Zed." => {
            "Yalnızca Zed'in Preview sürümünde etkinleştirilecek ayarlar."
        }
        "Which side of the window the threads sidebar appears on." => {
            "Konuşma kenar çubuğunun pencerenin hangi tarafında görüneceği."
        }
        "The API key sent as Authorization: Bearer {key}." => {
            "Authorization: Bearer {key} olarak gönderilen API anahtarı."
        }
        "The API URL to use for Codestral." => "Codestral için kullanılacak API URL'si.",
        "The base URL of your Ollama server." => "Ollama sunucunuzun temel URL'si.",
        "The Codestral model id to use." => "Kullanılacak Codestral model kimliği.",
        "The maximum number of tokens to generate." => "Üretilecek azami token sayısı.",
        "The model string to pass to the OpenAI-compatible server." => {
            "OpenAI uyumlu sunucuya geçirilecek model metni."
        }
        "The Ollama model to use for edit predictions." => {
            "Düzenleme tahminleri için kullanılacak Ollama modeli."
        }
        "The prompt format to use when requesting predictions. Set to Infer to have the format inferred based on the model name." => {
            "Tahmin istenirken kullanılacak istem biçimi. Biçimin model adına göre çıkarılması için Infer seçin."
        }
        "The URL of your OpenAI-compatible server's completions API." => {
            "OpenAI uyumlu sunucunuzun tamamlama API'sinin URL'si."
        }
        "Default timeout in seconds for MCP server tool calls." => {
            "MCP sunucusu araç çağrıları için saniye cinsinden varsayılan zaman aşımı."
        }
        "Domain cannot be empty." => "Alan adı boş olamaz.",
        "No global skills installed." => "Kurulu genel yetenek yok.",
        "No project skills found." => "Proje yeteneği bulunamadı.",
        "No skills available for this context." => "Bu bağlam için kullanılabilir yetenek yok.",
        "Commands executed in the terminal" => "Terminalde çalıştırılan komutlar",
        "Controls the default behavior for all tool actions. Per-tool rules and patterns can override this." => {
            "Tüm araç eylemleri için varsayılan davranışı denetler. Araç bazlı kurallar ve desenler bunu geçersiz kılabilir."
        }
        "Directory creation" => "Dizin oluşturma",
        "File and directory copying" => "Dosya ve dizin kopyalama",
        "File and directory deletion" => "Dosya ve dizin silme",
        "File and directory moves/renames" => "Dosya ve dizin taşıma/yeniden adlandırma",
        "File creation and overwrite operations" => "Dosya oluşturma ve üzerine yazma işlemleri",
        "File editing operations" => "Dosya düzenleme işlemleri",
        "HTTP requests to URLs" => "URL'lere HTTP istekleri",
        "Loading agent skill instructions" => "Ajan yetenek yönergeleri yükleniyor",
        "Pattern preview differs from engine — showing authoritative result." => {
            "Desen önizlemesi motordan farklı — yetkili sonuç gösteriliyor."
        }
        "Web search queries" => "Web arama sorguları",
        "They can be automatically migrated to the latest version." => {
            "Otomatik olarak en son sürüme taşınabilirler."
        }
        "They must be manually migrated to the latest version." => {
            "En son sürüme elle taşınmaları gerekir."
        }
        "This project is in restricted mode. Some project settings may not apply." => {
            "Bu proje kısıtlı modda. Bazı proje ayarları uygulanmayabilir."
        }

        // ── Ajan paneli ───────────────────────────────────────────────────
        "Dismiss" => "Kapat",
        "New Profile" => "Yeni Profil",
        "Search built-in tools…" => "Yerleşik araçlarda ara…",
        "Search MCP tools…" => "MCP araçlarında ara…",
        "Review" => "İncele",
        "Select a Model" => "Bir Model Seç",
        "Add Server…" => "Sunucu Ekle…",
        "Agent" => "Ajan",
        "Current Thread" => "Geçerli Konuşma",
        "Dev Server" => "Geliştirme Sunucusu",
        "Edit Thread Title" => "Konuşma Başlığını Düzenle",
        "Install New Servers…" => "Yeni Sunucular Kur…",
        "MCP Servers" => "MCP Sunucuları",
        "Open Global Rules" => "Genel Kuralları Aç",
        "Open Project Rules" => "Proje Kurallarını Aç",
        "Persisted Shell Title" => "Kalıcı Kabuk Başlığı",
        "Profiles" => "Profiller",
        "Reauthenticate" => "Yeniden Kimlik Doğrula",
        "Regenerate Thread Title" => "Konuşma Başlığını Yeniden Oluştur",
        "Restored Terminal" => "Geri Yüklenen Terminal",
        "Settings" => "Ayarlar",
        "Skills" => "Yetenekler",
        "Title generation failed. Click to retry." => {
            "Başlık oluşturulamadı. Yeniden denemek için tıklayın."
        }
        "Toggle Agent Menu" => "Ajan Menüsünü Aç/Kapat",
        "Toggle Threads Sidebar" => "Konuşma Kenar Çubuğunu Aç/Kapat",
        "Remove" => "Kaldır",
        "Retry" => "Yeniden Dene",
        "Unavailable" => "Kullanılamıyor",
        "Visit Agent Repository" => "Ajan Deposunu Ziyaret Et",
        "Visit Agent Website" => "Ajan Web Sitesini Ziyaret Et",
        "Zed Agent" => "Zed Ajanı",
        "Files & Directories" => "Dosyalar ve Dizinler",
        "Negative Feedback" => "Olumsuz Geri Bildirim",
        "Positive Feedback" => "Olumlu Geri Bildirim",
        "Selection" => "Seçim",
        "Symbols" => "Simgeler",
        "Threads" => "Konuşmalar",
        "Unknown" => "Bilinmiyor",
        "Connected" => "Bağlandı",
        "The model" => "Model",
        "Access" => "Erişim",
        "Account Name" => "Hesap Adı",
        "Age" => "Yaş",
        "Destination" => "Hedef",
        "Environment" => "Ortam",
        "Remember Authorization" => "Yetkilendirmeyi Hatırla",
        "Token" => "Token",
        "1 global rule" => "1 genel kural",
        "Accept" => "Kabul Et",
        "Anthropic will retain inference logs." => "Anthropic çıkarım günlüklerini saklayacak.",
        "Apply" => "Uygula",
        "Authentication Required" => "Kimlik Doğrulama Gerekli",
        "Awaiting Confirmation" => "Onay Bekleniyor",
        "Change Thinking Effort" => "Düşünme Çabasını Değiştir",
        "Clear All" => "Tümünü Temizle",
        "Clear Plan" => "Planı Temizle",
        "Codex on Windows" => "Windows'ta Codex",
        "Compacting Context…" => "Bağlam sıkıştırılıyor…",
        "Compaction Canceled" => "Sıkıştırma İptal Edildi",
        "Completed Plan" => "Tamamlanan Plan",
        "Configure unicode confusables warning" => "Unicode benzeri karakter uyarısını yapılandır",
        "Configure Windows-drive warning" => "Windows sürücü uyarısını yapılandır",
        "Context Compacted" => "Bağlam Sıkıştırıldı",
        "Context Too Large" => "Bağlam Çok Büyük",
        "Copy This Agent Response" => "Bu Ajan Yanıtını Kopyala",
        "Cost" => "Maliyet",
        "Couldn't create a sandbox" => "Sandbox oluşturulamadı",
        "Current:" => "Geçerli:",
        "Cycle Thinking Effort" => "Düşünme Çabaları Arasında Geç",
        "Discard Interrupted Edit" => "Yarıda Kalan Düzenlemeyi At",
        "Dismiss Warning" => "Uyarıyı Kapat",
        "Edit" => "Düzenle",
        "Edit Queued Message" => "Kuyruktaki Mesajı Düzenle",
        "Type anything to edit" => "Düzenlemek için bir şey yazın",
        "Send Now" => "Şimdi Gönder",
        "1 Queued Message" => "Kuyrukta 1 mesaj",
        // Kaynakta satır devamı (`\`) ile bölünmüş; çalışma zamanındaki hâli budur.
        "Interrupt the agent at its next step to send this message. When off, queued messages wait for the agent to finish." => {
            "Bu mesajı göndermek için ajanı bir sonraki adımında kes. Kapalıyken kuyruktaki mesajlar ajanın bitirmesini bekler."
        }
        "Editing will restart the thread from this point." => {
            "Düzenleme, konuşmayı bu noktadan yeniden başlatır."
        }
        "Edits" => "Düzenlemeler",
        "Everything below this line was sent as output from this subagent to the main agent." => {
            "Bu satırın altındaki her şey bu alt ajandan ana ajana çıktı olarak gönderildi."
        }
        "Free Usage Exceeded" => "Ücretsiz Kullanım Aşıldı",
        "Go to File" => "Dosyaya Git",
        "Helpful Response" => "Yararlı Yanıt",
        "Input:" => "Girdi:",
        "Interrupted Edit" => "Yarıda Kalan Düzenleme",
        "Loading Added Context…" => "Eklenen bağlam yükleniyor…",
        "Make Subagent Full Screen" => "Alt Ajanı Tam Ekran Yap",
        "Minimize Subagent" => "Alt Ajanı Küçült",
        "Network access" => "Ağ erişimi",
        "New Thread" => "Yeni Konuşma",
        "Not Helpful Response" => "Yararsız Yanıt",
        "Open Docs" => "Belgeleri Aç",
        "Open in WSL" => "WSL'de Aç",
        "Open Skill" => "Yeteneği Aç",
        "Output:" => "Çıktı:",
        "Plan" => "Plan",
        "Queue and Send" => "Kuyruğa Al ve Gönder",
        "Reason" => "Gerekçe",
        "Remove Message from Queue" => "Mesajı Kuyruktan Kaldır",
        "Request Refused" => "İstek Reddedildi",
        "Restores all files in the project to the content they had at this point in the conversation." => {
            "Projedeki tüm dosyaları konuşmanın bu noktasındaki içeriğine geri yükler."
        }
        "Resumed Session" => "Sürdürülen Oturum",
        "Retry Generation" => "Üretimi Yeniden Dene",
        "Review Before Sending" => "Göndermeden Önce İncele",
        "Review Changes" => "Değişiklikleri İncele",
        "Rules" => "Kurallar",
        "Run Command" => "Komutu Çalıştır",
        "Runs without the OS sandbox" => "İşletim sistemi sandbox'ı olmadan çalışır",
        "Scroll to User Message" => "Kullanıcı Mesajına Kaydır",
        "Select Model" => "Model Seç",
        "Select Options…" => "Seçenekleri Belirle…",
        "Send Immediately" => "Hemen Gönder",
        "Send Message" => "Mesaj Gönder",
        "Skill Failed to Load" => "Yetenek Yüklenemedi",
        "Start New Thread" => "Yeni Konuşma Başlat",
        "Steer" => "Yönlendir",
        "Stop Generation" => "Üretimi Durdur",
        "Stop Subagent" => "Alt Ajanı Durdur",
        "Subagent" => "Alt Ajan",
        "Subagent Cancelled" => "Alt Ajan İptal Edildi",
        "Subagent Failed" => "Alt Ajan Başarısız",
        "Subagent Output" => "Alt Ajan Çıktısı",
        "Thanks for your feedback!" => "Geri bildiriminiz için teşekkürler!",
        "Thinking" => "Düşünüyor",
        "This agent doesn't currently support multi-root workspaces" => {
            "Bu ajan şu anda çok köklü çalışma alanlarını desteklemiyor"
        }
        "This command can write to a file on a Windows drive" => {
            "Bu komut bir Windows sürücüsündeki dosyaya yazabilir"
        }
        "Type to Send" => "Göndermek için yazın",
        "Unavailable Editing" => "Düzenleme Kullanılamıyor",
        "View Sandboxing Docs" => "Sandbox Belgelerini Görüntüle",
        "We appreciate your feedback and will use it to improve in the future." => {
            "Geri bildiriminiz için teşekkür ederiz; ileride geliştirmek için kullanacağız."
        }
        "Write Access" => "Yazma Erişimi",
        "Accept Generation" => "Üretimi Kabul Et",
        "Accept Transform" => "Dönüşümü Kabul Et",
        "Add Context" => "Bağlam Ekle",
        "Bad Result" => "Kötü Sonuç",
        "Close Assistant" => "Asistanı Kapat",
        "Execute Generated Command" => "Üretilen Komutu Çalıştır",
        "Generate" => "Üret",
        "Good Result" => "İyi Sonuç",
        "Interrupt Generation" => "Üretimi Kes",
        "Interrupt Transform" => "Dönüşümü Kes",
        "Restart Generation" => "Üretimi Yeniden Başlat",
        "Restart Transform" => "Dönüşümü Yeniden Başlat",
        "Transform" => "Dönüştür",
        "Image" => "Görsel",
        "Plain Text" => "Düz Metin",
        "Auto" => "Otomatik",
        "Favorite" => "Sık Kullanılan",
        "Manual" => "Elle",
        "Regular" => "Normal",
        "Ask" => "Sor",
        "Write" => "Yaz",
        "Archive Thread" => "Konuşmayı Arşivle",
        "Delete Thread" => "Konuşmayı Sil",
        "Older" => "Daha Eski",
        "Past Week" => "Geçen Hafta",
        "Select" => "Seç",
        "This Week" => "Bu Hafta",
        "Today" => "Bugün",
        "Yesterday" => "Dün",
        "Free" => "Ücretsiz",
        "Latest" => "En Son",
        "Sandboxing" => "Sandbox",
        "Ran without sandbox" => "Sandbox olmadan çalıştı",
        "Stop This Command" => "Bu Komutu Durdur",
        "Unsandboxed execution is allowed for the rest of this thread." => {
            "Bu konuşmanın kalanında sandbox'sız çalıştırmaya izin verilir."
        }
        "Undo" => "Geri Al",

        // ── Unicode karakter adları (benzeri karakter uyarısı) ────────────
        "arabic letter mark" => "arap harfi işareti",
        "first strong isolate" => "ilk güçlü yalıtıcı",
        "ideographic space" => "ideografik boşluk",
        "left-to-right embedding" => "soldan sağa gömme",
        "left-to-right isolate" => "soldan sağa yalıtıcı",
        "left-to-right mark" => "soldan sağa işareti",
        "left-to-right override" => "soldan sağa geçersiz kılma",
        "mongolian vowel separator" => "moğolca ünlü ayırıcı",
        "no-break space" => "bölünmez boşluk",
        "pop directional formatting" => "yön biçimlendirmesini kaldır",
        "pop directional isolate" => "yön yalıtıcısını kaldır",
        "right-to-left embedding" => "sağdan sola gömme",
        "right-to-left isolate" => "sağdan sola yalıtıcı",
        "right-to-left mark" => "sağdan sola işareti",
        "right-to-left override" => "sağdan sola geçersiz kılma",
        "soft hyphen" => "yumuşak tire",
        "word joiner" => "sözcük birleştirici",
        "zero-width joiner" => "sıfır genişlikli birleştirici",
        "zero-width no-break space" => "sıfır genişlikli bölünmez boşluk",
        "zero-width non-joiner" => "sıfır genişlikli ayırıcı",
        "zero-width space" => "sıfır genişlikli boşluk",

        // ── Editör, Git, çalışma alanı, başlık çubuğu ─────────────────────
        "Actions" => "Eylemler",
        "Activate" => "Etkinleştir",
        "Add Project" => "Proje Ekle",
        "Agentic" => "Ajan Tabanlı",
        "All Branches" => "Tüm Dallar",
        "Amend" => "Düzelt",
        "Automate Worktree Setup" => "Worktree Kurulumunu Otomatikleştir",
        "Bottom" => "Alt",
        "Branch & Stash" => "Dal ve Stash",
        "Branches" => "Dallar",
        "Changes" => "Değişiklikler",
        "Class" => "Sınıf",
        "Classic" => "Klasik",
        "Close Announcement Banner" => "Duyuru Afişini Kapat",
        "Close Terminal" => "Terminali Kapat",
        "Close Worktree" => "Worktree'yi Kapat",
        "Color" => "Renk",
        "Commit message" => "Commit mesajı",
        "Configure an LLM provider to generate commit messages." => {
            "Commit mesajı üretmek için bir LLM sağlayıcısı yapılandırın."
        }
        "Confirm" => "Onayla",
        "Conflict marked as resolved" => "Çakışma çözüldü olarak işaretlendi",
        "Conflicts" => "Çakışmalar",
        "Conflicts marked as resolved" => "Çakışmalar çözüldü olarak işaretlendi",
        "Constant" => "Sabit",
        "Constructor" => "Yapıcı",
        "Could not open file" => "Dosya açılamadı",
        "Create Remote Repository" => "Uzak Depo Oluştur",
        "Default" => "Varsayılan",
        "Delete Worktree" => "Worktree'yi Sil",
        "Discard Draft" => "Taslağı At",
        "Disconnected" => "Bağlantı Kesildi",
        "Drop" => "Bırak",
        "Drop Stash" => "Stash'i At",
        "Edit Predictions" => "Düzenleme Tahminleri",
        "Editor Closed" => "Editör Kapatıldı",
        "Editor Opened" => "Editör Açıldı",
        "Editor Saved" => "Editör Kaydedildi",
        "Enter the command you use to SSH into this server." => {
            "Bu sunucuya SSH ile bağlanmak için kullandığınız komutu girin."
        }
        "Enum" => "Numaralandırma",
        "Enum Member" => "Numaralandırma Üyesi",
        "Error parsing date" => "Tarih ayrıştırılırken hata",
        "Event" => "Olay",
        "Exit" => "Çıkış",
        "Failed to connect over SSH" => "SSH üzerinden bağlanılamadı",
        "Failed to connect to WSL" => "WSL'ye bağlanılamadı",
        "Fetch in Progress…" => "Getirme sürüyor…",
        "Fetch: Already up to date" => "Getirme: Zaten güncel",
        "Field" => "Alan",
        "File" => "Dosya",
        "Filter Branches" => "Dalları Filtrele",
        "Focus Sidebar" => "Kenar Çubuğuna Odaklan",
        "Folder" => "Klasör",
        "Force Delete Worktree" => "Worktree'yi Zorla Sil",
        "Function" => "İşlev",
        "Generate Commit Message" => "Commit Mesajı Üret",
        "Git Commit" => "Git Commit",
        "Go Forward" => "İleri Git",
        "Go to next run" => "Sonraki çalıştırmaya git",
        "Go to previous run" => "Önceki çalıştırmaya git",
        "Horizontal" => "Yatay",
        "Interface" => "Arayüz",
        "Introducing:" => "Tanıtımı:",
        "Keyword" => "Anahtar Sözcük",
        "Locked Tab" => "Kilitli Sekme",
        "Method" => "Yöntem",
        "Minimap" => "Mini Harita",
        "Module" => "Modül",
        "Mute Microphone" => "Mikrofonu Sustur",
        "New Thread In…" => "Şurada Yeni Konuşma…",
        "Next Signature" => "Sonraki İmza",
        "No changes" => "Değişiklik yok",
        "No Changes to Commit" => "Commit Edilecek Değişiklik Yok",
        "No Code Actions Available" => "Kullanılabilir Kod Eylemi Yok",
        "No retrieval runs yet" => "Henüz getirme çalıştırması yok",
        "No staged changes yet" => "Henüz hazırlanmış değişiklik yok",
        "No threads yet" => "Henüz konuşma yok",
        "No unstaged changes" => "Hazırlanmamış değişiklik yok",
        "Nothing running" => "Çalışan bir şey yok",
        "Open Commit Modal" => "Commit Penceresini Aç",
        "Open Git Graph" => "Git Grafiğini Aç",
        "Open Project in New Window" => "Projeyi Yeni Pencerede Aç",
        "Open Project in This Window" => "Projeyi Bu Pencerede Aç",
        "Open Worktrees" => "Worktree'leri Aç",
        "Operator" => "İşleç",
        "Organization" => "Kuruluş",
        "Please restart Zed to Collaborate" => "İşbirliği için Zed'i yeniden başlatın",
        "Please sign in to continue." => "Devam etmek için giriş yapın.",
        "Please update Zed to Collaborate" => "İşbirliği için Zed'i güncelleyin",
        "Pop" => "Uygula",
        "Pop Stash" => "Stash'i Uygula",
        "Previous Signature" => "Önceki İmza",
        "Property" => "Özellik",
        "Providers" => "Sağlayıcılar",
        "Pull in Progress…" => "Çekme sürüyor…",
        "Pull: Already up to date" => "Çekme: Zaten güncel",
        "Push in Progress…" => "Gönderme sürüyor…",
        "Push: Everything is up-to-date" => "Gönderme: Her şey güncel",
        "Recent Projects" => "Son Projeler",
        "Reconnect" => "Yeniden Bağlan",
        "Recorded Events & Input" => "Kaydedilen Olaylar ve Girdi",
        "Redo Failed" => "Yineleme Başarısız",
        "Reference" => "Referans",
        "Remote Project" => "Uzak Proje",
        "Remove Bookmark" => "Yer İmini Kaldır",
        "Remove Folder from Project" => "Klasörü Projeden Kaldır",
        "Remove from Recent Projects" => "Son Projelerden Kaldır",
        "Remove Project from Window" => "Projeyi Pencereden Kaldır",
        "Rename Thread" => "Konuşmayı Yeniden Adlandır",
        "Rename Title" => "Başlığı Yeniden Adlandır",
        "Review .zed/settings.json for any extensions or commands configured by this project." => {
            "Bu projenin yapılandırdığı uzantı veya komutlar için .zed/settings.json dosyasını gözden geçirin."
        }
        "Run main" => "main'i çalıştır",
        "Run test" => "Testi çalıştır",
        "Search threads…" => "Konuşmalarda ara…",
        "Set Bookmark" => "Yer İmi Koy",
        "Set Breakpoint" => "Kesme Noktası Koy",
        "Share Project" => "Projeyi Paylaş",
        "Signoff" => "İmza",
        "Snippet" => "Parçacık",
        "Splits the pane downward." => "Paneyi aşağı böler.",
        "Splits the pane horizontally." => "Paneyi yatay böler.",
        "Splits the pane to the left." => "Paneyi sola böler.",
        "Splits the pane to the right." => "Paneyi sağa böler.",
        "Splits the pane upward." => "Paneyi yukarı böler.",
        "Splits the pane vertically." => "Paneyi dikey böler.",
        "Staged" => "Hazırlanan",
        "Stashes" => "Stash'ler",
        "Struct" => "Yapı",
        "Suggested Edits" => "Önerilen Düzenlemeler",
        "Suppress" => "Bastır",
        "Switch Active Repository" => "Etkin Depoyu Değiştir",
        "Switch Branch" => "Dal Değiştir",
        "Synchronized with remotes" => "Uzak depolarla eşitlendi",
        "Test" => "Test",
        "Text" => "Metin",
        "This buffer contains unsaved edits. Do you want to save it?" => {
            "Bu tamponda kaydedilmemiş düzenlemeler var. Kaydetmek ister misiniz?"
        }
        "Toggle Branch Picker" => "Dal Seçiciyi Aç/Kapat",
        "Toggle Folder" => "Klasörü Aç/Kapat",
        "Toggle Sidebar" => "Kenar Çubuğunu Aç/Kapat",
        "Toggle Stash Picker" => "Stash Seçiciyi Aç/Kapat",
        "Tracked" => "İzlenen",
        "Type Parameter" => "Tür Parametresi",
        "Uncommit" => "Commit'i Geri Al",
        "Undo Failed" => "Geri Alma Başarısız",
        "Unified" => "Birleşik",
        "Unit" => "Birim",
        "Unknown screen" => "Bilinmeyen ekran",
        "Unlock Tab" => "Sekmenin Kilidini Aç",
        "Unmute Microphone" => "Mikrofonun Sesini Aç",
        "Unshare Project" => "Proje Paylaşımını Durdur",
        "Unstaged" => "Hazırlanmamış",
        "Untracked" => "İzlenmeyen",
        "Untrusted projects are opened in Restricted Mode to protect your system." => {
            "Güvenilmeyen projeler sisteminizi korumak için Kısıtlı Modda açılır."
        }
        "Updating..." => "Güncelleniyor...",
        "Usage" => "Kullanım",
        "Value" => "Değer",
        "Variable" => "Değişken",
        "Version" => "Sürüm",
        "Vertical" => "Dikey",
        "View Changes" => "Değişiklikleri Görüntüle",
        "View Commit Diff" => "Commit Farkını Görüntüle",
        "Worktree" => "Worktree",
        "You're in Restricted Mode" => "Kısıtlı Moddasınız",
        "Zed Editor Dev Instance Running" => "Zed Editor Dev örneği çalışıyor",
        "Zed Editor Nightly Instance Running" => "Zed Editor Nightly örneği çalışıyor",
        "Zed Editor Preview Instance Running" => "Zed Editor Preview örneği çalışıyor",
        "Zed Editor Stable Instance Running" => "Zed Editor Stable örneği çalışıyor",

        // ── Hata ayıklayıcı, REPL, dil modeli kurulumu, CSV ───────────────
        "Allows inspection of memory contents" => "Bellek içeriğinin incelenmesine olanak tanır",
        "Attach" => "Bağlan",
        "Attach New Session Setup" => "Yeni Oturum Kurulumuna Bağlan",
        "Attach the debugger to a running process" => {
            "Hata ayıklayıcıyı çalışan bir işleme bağla"
        }
        "Authentication Required." => "Kimlik Doğrulama Gerekli.",
        "Breakpoints" => "Kesme Noktaları",
        "Build Failed" => "Derleme Başarısız",
        "Busy" => "Meşgul",
        "Center" => "Orta",
        "Clear all outputs" => "Tüm çıktıları temizle",
        "Click 'Connect' below to start using llama.cpp in Zed" => {
            "Zed'de llama.cpp kullanmaya başlamak için aşağıdaki 'Bağlan'a tıklayın"
        }
        "Client Secret Required." => "İstemci Gizli Anahtarı Gerekli.",
        "Close Panel" => "Paneli Kapat",
        "Complete" => "Tamamlandı",
        "Connect" => "Bağlan",
        "Connecting to kernel..." => "Çekirdeğe bağlanılıyor...",
        "Continue Program" => "Programı Sürdür",
        "Copy Name" => "Adı Kopyala",
        "Copy Value" => "Değeri Kopyala",
        "CSV Preview" => "CSV Önizlemesi",
        "Current State" => "Geçerli Durum",
        "Debug" => "Hata Ayıkla",
        "Debugger Docs" => "Hata Ayıklayıcı Belgeleri",
        "Debugger Extensions" => "Hata Ayıklayıcı Uzantıları",
        "Deleted" => "Silindi",
        "Deprecated Feature" => "Kullanımdan Kaldırılan Özellik",
        "Detach" => "Bağlantıyı Kes",
        "Disabled" => "Devre Dışı",
        "Documents" => "Belgeler",
        "Download LM Studio" => "LM Studio'yu İndir",
        "Edit Value" => "Değeri Düzenle",
        "Empty" => "Boş",
        "Enable Fast Mode for Anthropic?" => "Anthropic için Hızlı Mod etkinleştirilsin mi?",
        "Enable Fast Mode for OpenAI?" => "OpenAI için Hızlı Mod etkinleştirilsin mi?",
        "Enable Fast Mode for Zed?" => "Zed için Hızlı Mod etkinleştirilsin mi?",
        "End" => "Son",
        "Enter either access keys OR a Bedrock API Key below (not both)" => {
            "Aşağıya ya erişim anahtarlarını YA DA bir Bedrock API Anahtarı girin (ikisini birden değil)"
        }
        "Evaluate" => "Değerlendir",
        "Everywhere" => "Her Yerde",
        "Execute all cells" => "Tüm hücreleri çalıştır",
        "Executing..." => "Çalıştırılıyor...",
        "Extra High" => "Çok Yüksek",
        "Files" => "Dosyalar",
        "Filled" => "Dolu",
        "Filter Sort:" => "Filtre Sıralaması:",
        "Go To Memory" => "Belleğe Git",
        "Grant permissions to the strategy you'll use according to the:" => {
            "Kullanacağınız yönteme şuna göre izin verin:"
        }
        "Idle" => "Boşta",
        "Important Notice" => "Önemli Duyuru",
        "Interrupt Kernel" => "Çekirdeği Kes",
        "Jupyter Server" => "Jupyter Sunucusu",
        "Kernel Error" => "Çekirdek Hatası",
        "Kernel restarting..." => "Çekirdek yeniden başlatılıyor...",
        "Kernel shutdown" => "Çekirdek kapatıldı",
        "Kernel shutting down..." => "Çekirdek kapatılıyor...",
        "Launch" => "Başlat",
        "Launch a new process with a debugger" => "Hata ayıklayıcıyla yeni bir işlem başlat",
        "Left" => "Sol",
        "Lines" => "Satırlar",
        "Lists all active breakpoints set in the code" => {
            "Kodda ayarlanmış tüm etkin kesme noktalarını listeler"
        }
        "LM Studio needs to be running with at least one model downloaded." => {
            "LM Studio'nun çalışıyor ve en az bir model indirilmiş olması gerekir."
        }
        "Loading…" => "Yükleniyor…",
        "Max" => "Azami",
        "Mix and match Zed's agent with any ACP-compatible agent" => {
            "Zed ajanını ACP uyumlu herhangi bir ajanla birlikte kullanın"
        }
        "Model Catalog" => "Model Kataloğu",
        "Move cell down" => "Hücreyi aşağı taşı",
        "Move cell up" => "Hücreyi yukarı taşı",
        "New Session" => "Yeni Oturum",
        "New Update Available" => "Yeni Güncelleme Var",
        "No Breakpoints Set" => "Kesme Noktası Ayarlanmadı",
        "No CSV content to display" => "Gösterilecek CSV içeriği yok",
        "No subscriptions enabled. Enable at least one subscription to use OpenCode." => {
            "Etkin abonelik yok. OpenCode kullanmak için en az bir aboneliği etkinleştirin."
        }
        "Normal Text" => "Normal Metin",
        "Not authenticated" => "Kimlik doğrulanmadı",
        "Not sorted. Click to sort A-Z" => "Sıralanmadı. A-Z sıralamak için tıklayın",
        "Open Debug Adapter Logs" => "Hata Ayıklama Bağdaştırıcısı Günlüklerini Aç",
        "Add code block" => "Kod bloğu ekle",
        "Add markdown block" => "Markdown bloğu ekle",

        // ── Genel arayüz, hata ayıklayıcı, uzantılar, sağlayıcı kurulumu ──
        "OK" => "Tamam",
        "Active Call" => "Etkin Görüşme",
        "Add your own keys to use AI without signing in." => {
            "Giriş yapmadan yapay zekâ kullanmak için kendi anahtarlarınızı ekleyin."
        }
        "Agent Servers" => "Ajan Sunucuları",
        "All threads" => "Tüm konuşmalar",
        "Auto Watch Screens" => "Ekranları Otomatik İzle",
        "Auto Watch Screens (paused while sharing)" => {
            "Ekranları Otomatik İzle (paylaşım sırasında duraklatılır)"
        }
        "Available in all locations in your current project." => {
            "Geçerli projenizdeki tüm konumlarda kullanılabilir."
        }
        "Available in all of your projects on this machine." => {
            "Bu makinedeki tüm projelerinizde kullanılabilir."
        }
        "Buffer Search" => "Tampon Araması",
        "Categories" => "Kategoriler",
        "Channel Invites" => "Kanal Davetleri",
        "Close" => "Kapat",
        "Code" => "Kod",
        "Connection" => "Bağlantı",
        "Contact Requests" => "Kişi İstekleri",
        "Continue" => "Devam Et",
        "Debug Adapters" => "Hata Ayıklama Bağdaştırıcıları",
        "Default Vim Bindings" => "Varsayılan Vim Kısayolları",
        "Delete from Recent Tasks" => "Son Görevlerden Sil",
        "Done" => "Bitti",
        "Edit and save files directly in the results multibuffer!" => {
            "Dosyaları doğrudan sonuçlar çoklu tamponunda düzenleyip kaydedin!"
        }
        "Edit this binding" => "Bu kısayolu düzenle",
        "Edit Zoom" => "Yakınlaştırmayı Düzenle",
        "Enable text, syntax, or semantic highlights in the toolbar" => {
            "Araç çubuğunda metin, sözdizimi veya anlamsal vurguları etkinleştirin"
        }
        "Error downloading resources locally" => "Kaynaklar yerel olarak indirilirken hata",
        "Extend the agent with focused instructions and domain knowledge." => {
            "Ajanı odaklı yönergeler ve alan bilgisiyle genişletin."
        }
        "Failed to fetch resources from template or feature repository" => {
            "Şablon veya özellik deposundan kaynaklar alınamadı"
        }
        "Failed to Load Mermaid Diagram" => "Mermaid Diyagramı Yüklenemedi",
        "Failed to load SVG image" => "SVG görseli yüklenemedi",
        "Failed to move channel down" => "Kanal aşağı taşınamadı",
        "Failed to move channel up" => "Kanal yukarı taşınamadı",
        "Failed to parse file .devcontainer/devcontainer.json" => {
            ".devcontainer/devcontainer.json dosyası ayrıştırılamadı"
        }
        "Fetch all remotes" => "Tüm uzak depoları getir",
        "Filter definitions…" => "Tanımları filtrele…",
        "Filter implementations…" => "Uygulamaları filtrele…",
        "Filter references…" => "Referansları filtrele…",
        "Filters" => "Filtreler",
        "Finished reasoning." => "Akıl yürütme tamamlandı.",
        "Fit to View" => "Görünüme Sığdır",
        "Foreground" => "Ön Plan",
        "Getting Started" => "Başlarken",
        "Global" => "Genel",
        "Grammars" => "Dilbilgileri",
        "History" => "Geçmiş",
        "Icon Themes" => "Simge Temaları",
        "Indexed Docs Providers" => "Dizinlenmiş Belge Sağlayıcıları",
        "Languages" => "Diller",
        "Layout" => "Düzen",
        "Loading draft" => "Taslak yükleniyor",
        "Loading projector" => "Yansıtıcı yükleniyor",
        "Loading weights" => "Ağırlıklar yükleniyor",
        "Loading..." => "Yükleniyor...",
        "Low" => "Düşük",
        "Medium" => "Orta",
        "XHigh" => "Çok Yüksek",
        "Off" => "Kapalı",
        "Top" => "Üst",
        "Markdown Preview" => "Markdown Önizlemesi",
        "Match Case Sensitivity" => "Büyük/Küçük Harf Eşleştir",
        "Match Whole Words" => "Tam Sözcük Eşleştir",
        "No active connection" => "Etkin bağlantı yok",
        "No connection selected" => "Seçili bağlantı yok",
        "No definitions found" => "Tanım bulunamadı",
        "No errors in" => "Şurada hata yok:",
        "No implementations found" => "Uygulama bulunamadı",
        "No messages recorded yet" => "Henüz kaydedilmiş mesaj yok",
        "No problems" => "Sorun yok",
        "No problems in" => "Şurada sorun yok:",
        "No references found" => "Referans bulunamadı",
        "No such file or directory" => "Böyle bir dosya veya dizin yok",
        "No SVG file selected" => "SVG dosyası seçilmedi",
        "No valid dev container definition found in project" => {
            "Projede geçerli bir dev container tanımı bulunamadı"
        }
        "Not within a valid project" => "Geçerli bir projenin içinde değil",
        "Open" => "Aç",
        "Open Pull Request" => "Pull Request Aç",
        "Optional credit packs for additional usage" => "Ek kullanım için isteğe bağlı kredi paketleri",
        "Overwrite" => "Üzerine Yaz",
        "Paste your API key below and hit enter to start using OpenCode" => {
            "OpenCode kullanmaya başlamak için API anahtarınızı aşağıya yapıştırıp enter'a basın"
        }
        "Pause Program" => "Programı Duraklat",
        "Performance metrics:" => "Performans metrikleri:",
        "Plan and continue" => "Planla ve devam et",
        "Preview Below" => "Altta Önizle",
        "Preview to the Right" => "Sağda Önizle",
        "Profile" => "Profil",
        "Project" => "Proje",
        "Project Diagnostics" => "Proje Tanıları",
        "Project diagnostics: no problems" => "Proje tanıları: sorun yok",
        "Provider" => "Sağlayıcı",
        "Queued..." => "Kuyrukta...",
        "Read" => "Okuma",
        "Read/Write" => "Okuma/Yazma",
        "Read Documentation" => "Belgeleri Oku",
        "Recommended" => "Önerilen",
        "Refresh Diagnostics" => "Tanıları Yenile",
        "Remote: All threads" => "Uzak: Tüm konuşmalar",
        "Remote: Foreground" => "Uzak: Ön plan",
        "Remove Breakpoint" => "Kesme Noktasını Kaldır",
        "Remove breakpoint from a breakpoint list" => {
            "Kesme noktasını kesme noktası listesinden kaldır"
        }
        "Remove data breakpoint from a breakpoint list" => {
            "Veri kesme noktasını kesme noktası listesinden kaldır"
        }
        "Remove Watch" => "İzlemeyi Kaldır",
        "Rendering..." => "Çiziliyor...",
        "Rerun Session" => "Oturumu Yeniden Çalıştır",
        "Restart" => "Yeniden Başlat",
        "Restart Stack Frame" => "Yığın Çerçevesini Yeniden Başlat",
        "Restarting" => "Yeniden başlatılıyor",
        "Retain this context." => "Bu bağlamı koru.",
        "Rows" => "Satırlar",
        "Run" => "Çalıştır",
        "Run language task" => "Dil görevini çalıştır",
        "Run predefined task" => "Önceden tanımlı görevi çalıştır",
        "Running..." => "Çalışıyor...",
        "Run local LLMs like Llama, Phi, and Qwen." => {
            "Llama, Phi ve Qwen gibi yerel LLM'leri çalıştırın."
        }
        "Search by Keystrokes" => "Tuş Vuruşlarına Göre Ara",
        "Select Language" => "Dil Seç",
        "Select Line Ending" => "Satır Sonu Seç",
        "Set Condition" => "Koşul Belirle",
        "Set Hit Condition" => "İsabet Koşulu Belirle",
        "Set Log Message" => "Günlük Mesajı Belirle",
        "Show all stack frames" => "Tüm yığın çerçevelerini göster",
        "Show stack frames from your project" => "Projenizdeki yığın çerçevelerini göster",
        "Shows all modules or libraries loaded by the program" => {
            "Programın yüklediği tüm modülleri veya kitaplıkları gösterir"
        }
        "Shutdown" => "Kapat",
        "Shutting Down" => "Kapatılıyor",
        "Sign in" => "Giriş yap",
        "Slash Commands" => "Eğik Çizgi Komutları",
        "Snippets" => "Parçacıklar",
        "Sorted A-Z. Click to sort Z-A" => "A-Z sıralandı. Z-A sıralamak için tıklayın",
        "Sorted Z-A. Click to disable sorting" => "Z-A sıralandı. Sıralamayı kapatmak için tıklayın",
        "SSH Remote" => "SSH Uzak Bağlantı",
        "WSL Remote" => "WSL Uzak Bağlantı",
        "Start" => "Başlat",
        "Start a predefined debug scenario" => "Önceden tanımlı bir hata ayıklama senaryosu başlat",
        "Start Debug Session" => "Hata Ayıklama Oturumu Başlat",
        "Starting" => "Başlatılıyor",
        "Step Back in Session History" => "Oturum Geçmişinde Geri Adımla",
        "Step In" => "İçine Adımla",
        "Step Out" => "Dışına Adımla",
        "Step Over" => "Üzerinden Adımla",
        "Stop Auto Watching Screens" => "Ekranları Otomatik İzlemeyi Durdur",
        "Stop Diagnostics Update" => "Tanı Güncellemesini Durdur",
        "Subproject" => "Alt Proje",
        "Subscriptions" => "Abonelikler",
        "Text Alignment:" => "Metin Hizalaması:",
        "the default WSL distro" => "varsayılan WSL dağıtımı",
        "The editor has no text, syntax, or semantic token highlights" => {
            "Editörde metin, sözdizimi veya anlamsal belirteç vurgusu yok"
        }
        "The model failed to generate a response." => "Model bir yanıt üretemedi.",
        "Themes" => "Temalar",
        "This method uses your AWS access key ID and secret access key, or a Bedrock API Key." => {
            "Bu yöntem AWS erişim anahtarı kimliğinizi ve gizli erişim anahtarınızı ya da bir Bedrock API Anahtarını kullanır."
        }
        "This view lets you determine the current context stack for creating custom key bindings in Zed. When a keyboard shortcut is triggered, it also shows all the possible contexts it could have triggered in, and which one matched." => {
            "Bu görünüm, Zed'de özel kısayollar oluşturmak için geçerli bağlam yığınını belirlemenizi sağlar. Bir kısayol tetiklendiğinde, tetiklenebileceği tüm olası bağlamları ve hangisinin eşleştiğini de gösterir."
        }
        "To get your first model, try running" => "İlk modelinizi edinmek için şunu çalıştırın",
        "To use OpenCode models in Zed, you need an API key:" => {
            "Zed'de OpenCode modellerini kullanmak için bir API anahtarına ihtiyacınız var:"
        }
        "To use Zed's agent with Bedrock, you can set a custom authentication strategy through your settings file or use static credentials." => {
            "Zed ajanını Bedrock ile kullanmak için ayar dosyanızdan özel bir kimlik doğrulama yöntemi tanımlayabilir ya da statik kimlik bilgileri kullanabilirsiniz."
        }
        "Toggle Data Breakpoint" => "Veri Kesme Noktasını Aç/Kapat",
        "Toggle Exact Match Mode" => "Tam Eşleşme Modunu Aç/Kapat",
        "Toggle Multi Select" => "Çoklu Seçimi Aç/Kapat",
        "Toggle Search Selection" => "Arama Seçimini Aç/Kapat",
        "Try it out for 14 days, no credit card required" => {
            "14 gün deneyin, kredi kartı gerekmez"
        }
        "Try Now" => "Şimdi Dene",
        "Type a path…" => "Bir yol yazın…",
        "Unknown status" => "Bilinmeyen durum",
        "Unlimited prompts with your AI API keys" => {
            "Kendi yapay zekâ API anahtarlarınızla sınırsız istem"
        }
        "Using Bedrock API Key" => "Bedrock API Anahtarı kullanılıyor",
        "Using IAM credentials" => "IAM kimlik bilgileri kullanılıyor",
        "Watch Expression" => "İfadeyi İzle",
        "Watch Variable" => "Değişkeni İzle",
        "You have access to Zed's hosted models through your Pro trial." => {
            "Pro denemeniz sayesinde Zed'in barındırılan modellerine erişiminiz var."
        }
        "Your input exceeds the context window of this model." => {
            "Girdiniz bu modelin bağlam penceresini aşıyor."
        }
        "Zed Default" => "Zed Varsayılanı",
        "Zed Keybind Context" => "Zed Kısayol Bağlamı",
        "Zoom In" => "Yakınlaştır",
        "Zoom Out" => "Uzaklaştır",
        "1 month ago" => "1 ay önce",
        "1 week ago" => "1 hafta önce",

        // ── Durumlar, yerleşim, onboarding ────────────────────────────────
        "At Cursor" => "İmleçte",
        "Outline Panel" => "Anahat Paneli",
        "Project Explorer" => "Proje Gezgini",
        "Optional worktree isolation keeps agents from conflicting" => {
            "İsteğe bağlı worktree yalıtımı ajanların çakışmasını önler"
        }
        "Updated workspace layout designed for agentic workflows" => {
            "Ajan tabanlı iş akışları için tasarlanmış yenilenmiş çalışma alanı düzeni"
        }
        "Partial" => "Kısmi",
        "Recent" => "Son",
        "Right" => "Sağ",
        "Selected" => "Seçili",
        "Server has an error." => "Sunucuda hata var.",
        "Server is active." => "Sunucu etkin.",
        "Server is starting." => "Sunucu başlatılıyor.",
        "Server is stopped." => "Sunucu durduruldu.",
        "Subsection" => "Alt Bölüm",
        "Subtle" => "Hafif",
        "Success" => "Başarılı",
        "The current window will be closed, and connections to any shared projects will be terminated." => {
            "Geçerli pencere kapatılacak ve paylaşılan projelere olan bağlantılar sonlandırılacak."
        }
        "Toggle All Docks" => "Tüm Yerleşimleri Aç/Kapat",
        "Transparent" => "Saydam",
        "Waiting for Authorization…" => "Yetkilendirme bekleniyor…",
        "Warning" => "Uyarı",

        // ── Ağ / vekil sunucu hataları ────────────────────────────────────
        "address type not supported" => "adres türü desteklenmiyor",
        "command not supported" => "komut desteklenmiyor",
        "connection not allowed by ruleset" => "bağlantıya kural kümesi izin vermiyor",
        "connection refused" => "bağlantı reddedildi",
        "general SOCKS server failure" => "genel SOCKS sunucu hatası",
        "host unreachable" => "sunucuya ulaşılamıyor",
        "network unreachable" => "ağa ulaşılamıyor",
        "TTL expired" => "TTL süresi doldu",
        "too many requests" => "çok fazla istek",
        "terminated by signal" => "sinyalle sonlandırıldı",
        "response error" => "yanıt hatası",

        // ── Dev container / araç hataları ─────────────────────────────────
        "docker CLI not found on $PATH" => "docker CLI $PATH üzerinde bulunamadı",
        "lifecycle scripts could not execute for dev container" => {
            "dev container için yaşam döngüsü betikleri çalıştırılamadı"
        }
        "no usable `bwrap` binary was found on PATH" => {
            "PATH üzerinde kullanılabilir bir `bwrap` ikili dosyası bulunamadı"
        }
        "git log failed" => "git log başarısız oldu",

        // ── Ajan durumu / çeşitli ─────────────────────────────────────────
        "Completed thinking" => "Düşünme tamamlandı",
        "Thinking about the answer" => "Yanıt üzerine düşünülüyor",
        "Checked what information is needed." => "Hangi bilginin gerekli olduğu denetlendi.",
        "Apply a patch" => "Bir yama uygula",
        "notebook controls" => "not defteri denetimleri",
        "Virtual cell" => "Sanal hücre",
        "click to change min width" => "asgari genişliği değiştirmek için tıklayın",

        // ── Biçim şablonları (tr_format!) ─────────────────────────────────
        // Anahtar, `format!` şablonunun kendisidir. Türkçe karşılık konumlu
        // yer tutucu kullanabilir — sözcük dizilişi değişebildiği için.
        "Exited with code {}" => "{} koduyla çıktı",
        "Project diagnostics: {}" => "Proje tanıları: {}",
        "Git Clone: {}" => "Git Klonlama: {}",
        "Create New From: {}" => "Şundan yeni oluştur: {}",
        "Changes since {}" => "{} sonrasındaki değişiklikler",
        "Base: {}" => "Temel: {}",
        "View on {}" => "{} üzerinde görüntüle",
        "Close {} Dock" => "{} paneli kapat",
        "{} Queued Messages" => "Kuyrukta {} mesaj",
        // Yerleşik ajan için ad yazmıyoruz; şablon yer tutucu içermediğinden
        // argüman kullanılmadan atılır.
        "Message the {}, @ to include context, / for commands" => {
            "Yapay zeka için mesajınızı yazın; bağlam eklemek için @, komutlar için /"
        }
        // Dış ajanlarda ad anlamlı, korunuyor.
        "Message {} — @ to include context, / for commands" => {
            "{} — mesajınızı yazın; bağlam eklemek için @, komutlar için /"
        }
        "Message {} — @ to include context" => "{} — mesajınızı yazın; bağlam eklemek için @",

        // ── Hata ayıklayıcı sekmeleri, tema modu, araç zinciri ────────────
        "Console" => "Konsol",
        "Frames" => "Çerçeveler",
        "Modules" => "Modüller",
        "Sources" => "Kaynaklar",
        "Memory View" => "Bellek Görünümü",
        "Select Debugger" => "Hata Ayıklayıcı Seç",
        "Unknown Session" => "Bilinmeyen Oturum",
        "Toolchain" => "Araç Zinciri",
        "Dark" => "Koyu",
        "Light" => "Açık",
        "System" => "Sistem",

        // ── Tanı paneli, dil sunucusu durumu, tahmin sağlayıcısı ──────────
        "No problems in workspace" => "Çalışma alanında sorun yok",
        "No errors in workspace" => "Çalışma alanında hata yok",
        "All Servers Operational" => "Tüm sunucular çalışıyor",
        "Server with errors" => "Hatalı sunucu",
        "Server with warnings" => "Uyarılı sunucu",
        "Server with notifications" => "Bildirimli sunucu",
        "Restart All Servers" => "Tüm Sunucuları Yeniden Başlat",
        "Stop All Servers" => "Tüm Sunucuları Durdur",
        "Starting…" => "Başlatılıyor…",
        "Stopped" => "Durduruldu",
        "Running" => "Çalışıyor",
        "Configure a Provider" => "Bir sağlayıcı yapılandırın",
        "Sign In Or Configure a Provider" => "Giriş yapın ya da bir sağlayıcı yapılandırın",

        // ── Ekran görüntülerinden gelen kalan metinler ────────────────────
        "Enable Full Screen" => "Tam Ekranı Etkinleştir",
        "Work with your team in realtime with collaborative editing, voice, shared notes and more." => {
            "Ekibinizle gerçek zamanlı çalışın: ortak düzenleme, sesli görüşme, paylaşılan notlar ve dahası."
        }
        "Sign In with GitHub" => "GitHub ile Giriş Yap",
        "No outlines available" => "Kullanılabilir anahat yok",
        "Focus Content" => "İçeriğe Odaklan",
        "Focus Navbar" => "Gezinti Çubuğuna Odaklan",
        "Show Thread History" => "Konuşma Geçmişini Göster",
        "New Thread…" => "Yeni Konuşma…",
        "New Thread..." => "Yeni Konuşma...",
        "New {} Thread" => "Yeni {} Konuşması",
        "Open {}" => "{} projesini aç",
        "Input Requested by {}" => "{} girdi istiyor",
        "ID: {}" => "Kimlik: {}",
        "Denied: {}" => "Reddedildi: {}",
        "Reason: {}" => "Gerekçe: {}",
        "Error: {}" => "Hata: {}",
        "Follow {}" => "{} kullanıcısını takip et",
        "Connection: {}" => "Bağlantı: {}",

        // ── Panel adları ve seçici metinleri ──────────────────────────────
        "Collab Panel" => "İşbirliği Paneli",
        "Debug Panel" => "Hata Ayıklama Paneli",
        "Search profiles…" => "Profillerde ara…",
        "Follow the Zed Agent" => "Zed Ajanını Takip Et",
        "Track the agent's location as it reads and edits files." => {
            "Ajan dosyaları okurken ve düzenlerken konumunu izleyin."
        }
        "Or type @ to include context" => "Ya da bağlam eklemek için @ yazın",
        "Edit message － @ to include context" => {
            "Mesajı düzenle － bağlam eklemek için @"
        }
        "Configure natively-included model providers." => {
            "Yerleşik model sağlayıcılarını yapılandırın."
        }
        "View, add, and remove agents connected through the Agent Client Protocol." => {
            "Agent Client Protocol üzerinden bağlanan ajanları görüntüleyin, ekleyin ve kaldırın."
        }
        "View, add, configure, and remove Model Context Protocol servers." => {
            "Model Context Protocol sunucularını görüntüleyin, ekleyin, yapılandırın ve kaldırın."
        }
        "View and manage agent skills installed globally or in project worktrees." => {
            "Genel olarak veya proje worktree'lerinde kurulu ajan yeteneklerini görüntüleyin ve yönetin."
        }
        "Review and change the elevated terminal sandbox permissions that are always allowed without prompting." => {
            "Sormadan her zaman izin verilen yükseltilmiş terminal sandbox izinlerini gözden geçirin ve değiştirin."
        }
        "Set up regex patterns to auto-allow, auto-deny, or always request confirmation, for specific tool inputs." => {
            "Belirli araç girdileri için otomatik izin, otomatik ret veya her zaman onay isteyen düzenli ifade desenleri tanımlayın."
        }

        // ── Menü çubuğu ───────────────────────────────────────────────────
        // Menü etiketleri artık İngilizce kaynakta; Türkçesi burada. Böylece
        // başka bir dil de kendi JSON dosyasıyla menüleri çevirebilir.
        "Reset Zoom" => "Yakınlaştırmayı Sıfırla",
        "Reset All Zoom" => "Tüm Yakınlaştırmaları Sıfırla",
        "Toggle Left Dock" => "Sol Paneli Aç/Kapat",
        "Toggle Right Dock" => "Sağ Paneli Aç/Kapat",
        "Toggle Bottom Dock" => "Alt Paneli Aç/Kapat",
        "Editor Layout" => "Editör Düzeni",
        "Debugger Panel" => "Hata Ayıklayıcı Paneli",
        "Agent Panel" => "Ajan Paneli",
        "Git Panel" => "Git Paneli",
        "Diagnostics" => "Tanılar",
        "Toggle GPUI Inspector" => "GPUI Denetçisini Aç/Kapat",
        "About Zed" => "Zed Hakkında",
        "Check for Updates" => "Güncellemeleri Denetle",
        "Open Settings File" => "Ayarlar Dosyasını Aç",
        "Open Project Settings" => "Proje Ayarlarını Aç",
        "Open Project Settings File" => "Proje Ayarları Dosyasını Aç",
        "Open Default Settings" => "Varsayılan Ayarları Aç",
        "Open Keymap File" => "Tuş Haritası Dosyasını Aç",
        "Open Default Key Bindings" => "Varsayılan Tuş Bağlamalarını Aç",
        "Install CLI" => "CLI'yi Yükle",
        "Hide Zed" => "Zed'i Gizle",
        "Hide Others" => "Diğerlerini Gizle",
        "Show All" => "Tümünü Göster",
        "Quit Zed" => "Zed'den Çık",
        "New" => "Yeni",
        "Open File..." => "Dosya Aç...",
        "Open Folder..." => "Klasör Aç...",
        "Open…" => "Aç…",
        "Open Recent…" => "Son Açılanlar…",
        "Open Remote…" => "Uzak Proje Aç…",
        "Add Folder to Project…" => "Projeye Klasör Ekle…",
        "Save As…" => "Farklı Kaydet…",
        "Save All" => "Tümünü Kaydet",
        "Close Editor" => "Editörü Kapat",
        "Close Project" => "Projeyi Kapat",
        "Redo" => "Yinele",
        "Find" => "Bul",
        "Find in Project" => "Projede Bul",
        "Toggle Line Comment" => "Satır Yorumunu Aç/Kapat",
        "Select Next Sibling" => "Sonraki Kardeşi Seç",
        "Select Previous Sibling" => "Önceki Kardeşi Seç",
        "Select Previous Occurrence" => "Önceki Geçtiği Yeri Seç",
        "Select All Occurrences" => "Tüm Geçtiği Yerleri Seç",
        "Go" => "Git",
        "Back" => "Geri",
        "Forward" => "İleri",
        "Command Palette..." => "Komut Paleti...",
        "Go to File..." => "Dosyaya Git...",
        "Go to Symbol in Editor..." => "Editörde Sembole Git...",
        "Go to Line/Column..." => "Satıra/Sütuna Git...",
        "Start Debugger" => "Hata Ayıklamayı Başlat",
        "Edit tasks.json…" => "tasks.json'ı Düzenle…",
        "Edit debug.json…" => "debug.json'ı Düzenle…",
        "Step Into" => "İçine Adım At",
        "Toggle Breakpoint" => "Kesme Noktasını Aç/Kapat",
        "Edit Breakpoint" => "Kesme Noktasını Düzenle",
        "Clear All Breakpoints" => "Tüm Kesme Noktalarını Temizle",
        "Window" => "Pencere",
        "Minimize" => "Küçült",
        "Zoom" => "Büyüt",
        "Help" => "Yardım",
        "View Release Notes Locally" => "Sürüm Notlarını Yerel Olarak Görüntüle",
        "View Telemetry" => "Telemetriyi Görüntüle",
        "View Dependency Licenses" => "Bağımlılık Lisanslarını Görüntüle",
        "Show Welcome" => "Karşılama Ekranını Göster",
        "File Bug Report..." => "Hata Raporu Gönder...",
        "Request Feature..." => "Özellik İste...",
        "Email Us..." => "Bize E-posta Gönder...",
        "Documentation" => "Belgeler",
        "Zed Repository" => "Zed Deposu",
        "Join the Team" => "Ekibe Katıl",
        "Expand Message Editor" => "Mesaj Editörünü Genişlet",
        "Open Recent Project" => "Son Projeyi Aç",
        "New Agent Thread" => "Yeni Ajan Konuşması",
        "untitled" => "adsız",
        "Unrecognized Project" => "Tanınmayan Proje",
        "Unrecognized Projects ({})" => "Tanınmayan Projeler ({})",
        "Retrying. Next attempt in 1 second." => "Yeniden deneniyor. Sonraki deneme 1 saniye sonra.",
        "Retrying. Next attempt in {} seconds." => {
            "Yeniden deneniyor. Sonraki deneme {} saniye sonra."
        }
        "Retrying. Next attempt in 1 second (Attempt {} of {})." => {
            "Yeniden deneniyor. Sonraki deneme 1 saniye sonra ({}/{} deneme)."
        }
        "Retrying. Next attempt in {} seconds (Attempt {} of {})." => {
            "Yeniden deneniyor. Sonraki deneme {} saniye sonra ({}/{} deneme)."
        }
        "About Zed L10n" => "Zed L10n Hakkında",


        // ── Proje paneli, karşılama ekranı, sekme menüsü ─────────────────
        // Bu metinler önceden koda Türkçe gömülüydü; İngilizce kaynağa
        // alındı ki başka diller de kendi JSON'uyla çevirebilsin.
        "Collapse All" => "Tümünü Daralt",
        "Expand All" => "Tümünü Genişlet",
        "Remove from Project" => "Projeden Kaldır",
        "Trash" => "Çöp Kutusuna Taşı",
        "Rename" => "Yeniden Adlandır",
        "View History" => "Geçmişi Görüntüle",
        "Restore File" => "Dosyayı Geri Yükle",
        "Download..." => "İndir...",
        "Duplicate" => "Çoğalt",
        "Compare Marked Files" => "İşaretli Dosyaları Karşılaştır",
        "New Folder" => "Yeni Klasör",
        "Search Inside" => "İçinde Ara",
        "Return to Onboarding" => "Kayıt Adımlarına Dön",
        "Welcome back to Zed" => "Zed'e Tekrar Hoş Geldiniz",
        "Open Agent Panel" => "Ajan Panelini Aç",
        "Collaborate with Agents" => "Ajanlarla İş Birliği Yap",
        "Run multiple threads at once, mix and match any ACP-compatible agent, and keep work conflict-free with worktrees." => "Aynı anda birden çok iş parçacığı çalıştırın, ACP uyumlu ajanları birleştirin ve worktree'ler sayesinde çakışmasız çalışın.",
        "Explore Extensions" => "Uzantıları Keşfet",
        "Customize Keymaps" => "Tuş Bağlamalarını Özelleştir",
        "Open Command Palette" => "Komut Paletini Aç",
        "Make Tab Editable" => "Sekmeyi Düzenlenebilir Yap",
        "Make Tab Read-Only" => "Sekmeyi Salt Okunur Yap",
        "Pin Tab" => "Sekmeyi Sabitle",
        "Unpin Tab" => "Sekmenin Sabitlemesini Kaldır",
        "Close All" => "Tümünü Kapat",
        "Close Clean" => "Temiz Olanları Kapat",
        "Close Right" => "Sağdakileri Kapat",
        "Close Left" => "Soldakileri Kapat",
        "Close Multibuffers" => "Çoklu Tamponları Kapat",
        "Close Others" => "Diğerlerini Kapat",

        _ => return None,
    })
}
