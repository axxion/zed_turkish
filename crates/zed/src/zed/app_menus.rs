use collab_ui::collab_panel;
use gpui::{App, Menu, MenuItem, OsAction};
use release_channel::ReleaseChannel;
use terminal_view::terminal_panel;
use zed_actions::{Quit, assistant, debug_panel, dev, git_panel, project_panel};

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    let mut view_items = vec![
        MenuItem::action(
            "Yakınlaştır",
            zed_actions::IncreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "Uzaklaştır",
            zed_actions::DecreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "Yakınlaştırmayı Sıfırla",
            zed_actions::ResetBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "Tüm Yakınlaştırmaları Sıfırla",
            zed_actions::ResetAllZoom { persist: false },
        ),
        MenuItem::separator(),
        MenuItem::action("Sol Paneli Aç/Kapat", workspace::ToggleLeftDock),
        MenuItem::action("Sağ Paneli Aç/Kapat", workspace::ToggleRightDock),
        MenuItem::action("Alt Paneli Aç/Kapat", workspace::ToggleBottomDock),
        MenuItem::action("Tüm Panelleri Aç/Kapat", workspace::ToggleAllDocks),
        MenuItem::submenu(Menu {
            name: "Editör Düzeni".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Yukarı Böl", workspace::SplitUp::default()),
                MenuItem::action("Aşağı Böl", workspace::SplitDown::default()),
                MenuItem::action("Sola Böl", workspace::SplitLeft::default()),
                MenuItem::action("Sağa Böl", workspace::SplitRight::default()),
            ],
        }),
        MenuItem::separator(),
        MenuItem::action("Proje Paneli", project_panel::ToggleFocus),
        MenuItem::action("Anahat Paneli", outline_panel::ToggleFocus),
        MenuItem::action("İş Birliği Paneli", collab_panel::ToggleFocus),
        MenuItem::action("Terminal Paneli", terminal_panel::Toggle),
        MenuItem::action("Hata Ayıklayıcı Paneli", debug_panel::ToggleFocus),
        MenuItem::action("Ajan Paneli", assistant::ToggleFocus),
        MenuItem::action("Git Paneli", git_panel::ToggleFocus),
        MenuItem::separator(),
        MenuItem::action("Tanılar", diagnostics::Deploy),
        MenuItem::separator(),
    ];

    if ReleaseChannel::try_global(cx) == Some(ReleaseChannel::Dev) {
        view_items.push(MenuItem::action(
            "GPUI Denetçisini Aç/Kapat",
            dev::ToggleInspector,
        ));
        view_items.push(MenuItem::separator());
    }

    vec![
        Menu {
            name: "Zed".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Zed Hakkında", zed_actions::About),
                MenuItem::action("Güncellemeleri Denetle", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu::new("Ayarlar").items([
                    MenuItem::action("Ayarları Aç", zed_actions::OpenSettings),
                    MenuItem::action("Ayarlar Dosyasını Aç", super::OpenSettingsFile),
                    MenuItem::action("Proje Ayarlarını Aç", zed_actions::OpenProjectSettings),
                    MenuItem::action(
                        "Proje Ayarları Dosyasını Aç",
                        super::OpenProjectSettingsFile,
                    ),
                    MenuItem::action("Varsayılan Ayarları Aç", super::OpenDefaultSettings),
                    MenuItem::separator(),
                    MenuItem::action("Tuş Haritasını Aç", zed_actions::OpenKeymap),
                    MenuItem::action("Tuş Haritası Dosyasını Aç", zed_actions::OpenKeymapFile),
                    MenuItem::action(
                        "Varsayılan Tuş Bağlamalarını Aç",
                        zed_actions::OpenDefaultKeymap,
                    ),
                    MenuItem::separator(),
                    MenuItem::action(
                        "Tema Seç...",
                        zed_actions::theme_selector::Toggle::default(),
                    ),
                    MenuItem::action(
                        "Simge Teması Seç...",
                        zed_actions::icon_theme_selector::Toggle::default(),
                    ),
                ])),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu("Services", gpui::SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action("Uzantılar", zed_actions::Extensions::default()),
                #[cfg(not(target_os = "windows"))]
                MenuItem::action("CLI'yi Yükle", install_cli::InstallCliBinary),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action("Zed'i Gizle", super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("Diğerlerini Gizle", super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action("Tümünü Göster", super::ShowAll),
                MenuItem::separator(),
                MenuItem::action("Zed'den Çık", Quit),
            ],
        },
        Menu {
            name: "Dosya".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Yeni", workspace::NewFile),
                MenuItem::action("Yeni Pencere", workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action("Dosya Aç...", workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        "Klasör Aç..."
                    } else {
                        "Aç…"
                    },
                    workspace::Open::default(),
                ),
                MenuItem::action("Son Açılanlar…", zed_actions::OpenRecent::default()),
                MenuItem::action("Uzak Proje Aç…", zed_actions::OpenRemote::default()),
                MenuItem::separator(),
                MenuItem::action("Projeye Klasör Ekle…", workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action("Kaydet", workspace::Save { save_intent: None }),
                MenuItem::action("Farklı Kaydet…", workspace::SaveAs),
                MenuItem::action("Tümünü Kaydet", workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    "Editörü Kapat",
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action("Projeyi Kapat", workspace::CloseProject),
                MenuItem::action("Pencereyi Kapat", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "Düzen".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action("Geri Al", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("Yinele", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Kes", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("Kopyala", editor::actions::Copy, OsAction::Copy),
                MenuItem::action("Kopyala ve Kırp", editor::actions::CopyAndTrim),
                MenuItem::os_action("Yapıştır", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("Bul", search::buffer_search::Deploy::find()),
                MenuItem::action("Projede Bul", workspace::DeploySearch::default()),
                MenuItem::separator(),
                MenuItem::action(
                    "Satır Yorumunu Aç/Kapat",
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: "Seçim".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action(
                    "Tümünü Seç",
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action("Seçimi Genişlet", editor::actions::SelectLargerSyntaxNode),
                MenuItem::action("Seçimi Daralt", editor::actions::SelectSmallerSyntaxNode),
                MenuItem::action(
                    "Sonraki Kardeşi Seç",
                    editor::actions::SelectNextSyntaxNode,
                ),
                MenuItem::action(
                    "Önceki Kardeşi Seç",
                    editor::actions::SelectPreviousSyntaxNode,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "Üstüne İmleç Ekle",
                    editor::actions::AddSelectionAbove {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    "Altına İmleç Ekle",
                    editor::actions::AddSelectionBelow {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    "Sonraki Geçtiği Yeri Seç",
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    "Önceki Geçtiği Yeri Seç",
                    editor::actions::SelectPrevious {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    "Tüm Geçtiği Yerleri Seç",
                    editor::actions::SelectAllMatches,
                ),
                MenuItem::separator(),
                MenuItem::action("Satırı Yukarı Taşı", editor::actions::MoveLineUp),
                MenuItem::action("Satırı Aşağı Taşı", editor::actions::MoveLineDown),
                MenuItem::action("Seçimi Çoğalt", editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: "Görünüm".into(),
            disabled: false,
            items: view_items,
        },
        Menu {
            name: "Git".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Geri", workspace::GoBack),
                MenuItem::action("İleri", workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action("Komut Paleti...", zed_actions::command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action("Dosyaya Git...", workspace::ToggleFileFinder::default()),
                // MenuItem::action("Go to Symbol in Project", project_symbols::Toggle),
                MenuItem::action(
                    "Editörde Sembole Git...",
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action("Satıra/Sütuna Git...", editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action(
                    "Tanıma Git",
                    editor::actions::GoToDefinition::default(),
                ),
                MenuItem::action("Bildirime Git", editor::actions::GoToDeclaration),
                MenuItem::action("Tür Tanımına Git", editor::actions::GoToTypeDefinition),
                MenuItem::action(
                    "Tüm Başvuruları Bul",
                    editor::actions::FindAllReferences::default(),
                ),
                MenuItem::separator(),
                MenuItem::action("Sonraki Sorun", editor::actions::GoToDiagnostic::default()),
                MenuItem::action(
                    "Önceki Sorun",
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: "Çalıştır".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "Görev Başlat",
                    zed_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::action("Hata Ayıklamayı Başlat", debugger_ui::Start),
                MenuItem::separator(),
                MenuItem::action("tasks.json'ı Düzenle…", zed_actions::OpenProjectTasks),
                MenuItem::action("debug.json'ı Düzenle…", zed_actions::OpenProjectDebugTasks),
                MenuItem::separator(),
                MenuItem::action("Devam Et", debugger_ui::Continue),
                MenuItem::action("Üzerinden Adım At", debugger_ui::StepOver),
                MenuItem::action("İçine Adım At", debugger_ui::StepInto),
                MenuItem::action("Dışına Adım At", debugger_ui::StepOut),
                MenuItem::separator(),
                MenuItem::action("Kesme Noktasını Aç/Kapat", editor::actions::ToggleBreakpoint),
                MenuItem::action("Kesme Noktasını Düzenle", editor::actions::EditLogBreakpoint),
                MenuItem::action(
                    "Tüm Kesme Noktalarını Temizle",
                    debugger_ui::ClearAllBreakpoints,
                ),
            ],
        },
        Menu {
            name: "Pencere".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Küçült", super::Minimize),
                MenuItem::action("Büyüt", super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Yardım".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "Sürüm Notlarını Yerel Olarak Görüntüle",
                    auto_update_ui::ViewReleaseNotesLocally,
                ),
                MenuItem::action("Telemetriyi Görüntüle", zed_actions::OpenTelemetryLog),
                MenuItem::action(
                    "Bağımlılık Lisanslarını Görüntüle",
                    zed_actions::OpenLicenses,
                ),
                MenuItem::action("Karşılama Ekranını Göster", onboarding::ShowWelcome),
                MenuItem::separator(),
                MenuItem::action("Hata Raporu Gönder...", zed_actions::feedback::FileBugReport),
                MenuItem::action("Özellik İste...", zed_actions::feedback::RequestFeature),
                MenuItem::action("Bize E-posta Gönder...", zed_actions::feedback::EmailZed),
                MenuItem::separator(),
                MenuItem::action(
                    "Belgeler",
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action("Zed Deposu", feedback::OpenZedRepo),
                MenuItem::action(
                    "Zed Twitter",
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    "Ekibe Katıl",
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
