use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc, time::Duration};

pub enum WatchEvent {
    StyleChange,
    ContentChange,
    AnyChange,
}

pub fn watch_project<F>(src_dir: &Path, handler: F) -> Result<RecommendedWatcher, String>
where F: FnMut(WatchEvent) + Send + 'static {
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, _>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(200)),
    ).map_err(|e| format!("Watcher error: {}", e))?;

    watcher.watch(src_dir, RecursiveMode::Recursive)
        .map_err(|e| format!("Watch error: {}", e))?;

    std::thread::spawn(move || {
        let mut handler = handler;
        for event in rx {
            if let EventKind::Modify(_) | EventKind::Create(_) = event.kind {
                let is_style = event.paths.iter().any(|p| {
                    p.to_string_lossy().contains("theme") ||
                    p.to_string_lossy().contains("tokens")
                });
                handler(if is_style { WatchEvent::StyleChange } else { WatchEvent::ContentChange });
            }
        }
    });
    Ok(watcher)
}
