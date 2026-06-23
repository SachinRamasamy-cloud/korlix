pub fn format_error_html(error: &str) -> String {
    format!(r#"<div id="kx-error-overlay" style="position:fixed;inset:0;background:rgba(0,0,0,.92);z-index:99999;padding:2rem;font-family:monospace;color:#f87171;overflow:auto;"><h2 style="color:#ef4444">⚠ Korlix Error</h2><pre style="white-space:pre-wrap">{}</pre></div>"#,
        error.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;"))
}
