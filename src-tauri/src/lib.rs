use notify::{RecommendedWatcher, RecursiveMode, Watcher, EventKind};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

/// Aktif dosya izleyicisini (watcher) tutan uygulama durumu.
/// Kullanıcı senkron dosyasını değiştirdiğinde eski izleyici bırakılıp
/// yenisi kurulur; bu yüzden bir Mutex içinde tutuluyor.
struct WatcherState(Mutex<Option<RecommendedWatcher>>);

#[derive(Clone, serde::Serialize)]
struct SyncFileChangedPayload {
    path: String,
}

/// Verilen metin dosyasını okur. Dosya yoksa boş string döner
/// (henüz hiç yazılmamış yeni bir senkron dosyası olabilir).
#[tauri::command]
fn read_sync_file(path: String) -> Result<String, String> {
    match std::fs::read_to_string(&path) {
        Ok(contents) => Ok(contents),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(format!("Dosya okunamadı: {e}")),
    }
}

/// Verilen metni dosyaya yazar (tamamen üzerine yazar).
/// Geçici dosyaya yazıp yeniden adlandırma (atomic write) kullanır,
/// böylece bulut senkron istemcisi (Drive/iCloud) yarım yazılmış bir
/// dosyayı asla görmez.
#[tauri::command]
fn write_sync_file(path: String, contents: String) -> Result<(), String> {
    let target = PathBuf::from(&path);
    let tmp = target.with_extension("tmp-write");
    std::fs::write(&tmp, contents).map_err(|e| format!("Geçici dosyaya yazılamadı: {e}"))?;
    std::fs::rename(&tmp, &target).map_err(|e| format!("Dosya değiştirilemedi: {e}"))?;
    Ok(())
}

/// Verilen dosyanın var olup olmadığını ve boş olup olmadığını bildirir.
#[tauri::command]
fn sync_file_info(path: String) -> Result<serde_json::Value, String> {
    match std::fs::metadata(&path) {
        Ok(meta) => Ok(serde_json::json!({ "exists": true, "size": meta.len() })),
        Err(_) => Ok(serde_json::json!({ "exists": false, "size": 0 })),
    }
}

/// Verilen dosyayı arka planda izlemeye başlar. Dosya her değiştiğinde
/// (başka bir program -- örn. bulut senkron istemcisi -- tarafından
/// güncellendiğinde dahil) arayüze "sync-file-changed" olayı gönderilir,
/// arayüz de dosyayı yeniden okuyup kendi verisiyle birleştirir.
#[tauri::command]
fn start_watch(app: AppHandle, state: State<WatcherState>, path: String) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    // Önceki izleyici varsa bırak (drop), yenisini kur.
    *guard = None;

    let watch_path = PathBuf::from(&path);
    let parent = watch_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| watch_path.clone());

    let app_handle = app.clone();
    let target_file_name = watch_path
        .file_name()
        .map(|f| f.to_os_string());

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        let Ok(event) = res else { return; };
        // Sadece içerik değişikliği/oluşturma olaylarıyla ilgileniyoruz.
        let relevant = matches!(
            event.kind,
            EventKind::Modify(_) | EventKind::Create(_)
        );
        if !relevant {
            return;
        }
        // Aynı klasördeki alakasız dosyaları (ör. .tmp-write geçici dosyası) filtrele.
        let touches_target = event.paths.iter().any(|p| {
            match (&target_file_name, p.file_name()) {
                (Some(target), Some(name)) => name == target.as_os_str(),
                _ => true,
            }
        });
        if !touches_target {
            return;
        }
        let _ = app_handle.emit(
            "sync-file-changed",
            SyncFileChangedPayload { path: p_to_string(&event.paths) },
        );
    })
    .map_err(|e| format!("İzleyici oluşturulamadı: {e}"))?;

    // Dosyanın kendisini değil, içinde bulunduğu klasörü izliyoruz.
    // Çünkü çoğu bulut senkron istemcisi (Drive/iCloud) dosyayı
    // yazarken "sil + yeniden oluştur" deseni kullanabiliyor; sadece
    // dosyayı izlersek bu durumda izleme kopabilir.
    watcher
        .watch(&parent, RecursiveMode::NonRecursive)
        .map_err(|e| format!("Klasör izlenemedi: {e}"))?;

    *guard = Some(watcher);
    Ok(())
}

fn p_to_string(paths: &[PathBuf]) -> String {
    paths
        .first()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// İzlemeyi durdurur (kullanıcı senkron bağlantısını kestiğinde çağrılır).
#[tauri::command]
fn stop_watch(state: State<WatcherState>) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    *guard = None;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(WatcherState(Mutex::new(None)))
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            read_sync_file,
            write_sync_file,
            sync_file_info,
            start_watch,
            stop_watch
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
