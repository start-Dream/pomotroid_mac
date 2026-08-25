fn test_badge(app: &tauri::AppHandle) {
    let _ = app.set_badge_count(Some(12));
}
