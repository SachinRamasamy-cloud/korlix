interface HmrMessage {
  type: 'css-update' | 'full-reload' | 'route-update' | 'error' | 'clear-error';
  error?: string;
}

export function connectHMR(port = 3001): void {
  let retryCount = 0;
  function connect(): WebSocket {
    const ws = new WebSocket(`ws://localhost:${port}/__kx_hmr`);

    ws.onopen    = () => { retryCount = 0; console.log('[Korlix] 🔗 HMR connected'); };
    ws.onmessage = (e) => { try { handle(JSON.parse(e.data)); } catch(_) {} };
    ws.onclose   = () => {
      retryCount++;
      setTimeout(connect, Math.min(1000 * retryCount, 5000));
    };
    return ws;
  }
  connect();
}

function handle(msg: HmrMessage): void {
  switch (msg.type) {
    case 'css-update':
      refreshCSS();
      console.log('[Korlix] ⚡ CSS updated');
      break;
    case 'full-reload':
      location.reload();
      break;
    case 'error':
      showErrorOverlay(msg.error || 'Unknown error');
      break;
    case 'clear-error':
      clearErrorOverlay();
      break;
  }
}

function refreshCSS(): void {
  document.querySelectorAll<HTMLLinkElement>('link[rel="stylesheet"]').forEach(link => {
    const url = new URL(link.href);
    url.searchParams.set('t', String(Date.now()));
    link.href = url.toString();
  });
}

function showErrorOverlay(error: string): void {
  clearErrorOverlay();
  const overlay = document.createElement('div');
  overlay.id = 'kx-error-overlay';
  overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.93);z-index:99999;padding:2rem;font-family:monospace;overflow:auto;display:flex;align-items:flex-start;justify-content:center;';
  overlay.innerHTML = `
    <div style="max-width:800px;width:100%;margin-top:2rem">
      <h2 style="color:#ef4444;font-size:1.25rem;margin:0 0 1rem">⚠ Korlix Compile Error</h2>
      <pre style="background:#1e1e2e;color:#f1f5f9;padding:1.5rem;border-radius:.5rem;white-space:pre-wrap;border:1px solid #2d2d3d;overflow-x:auto">${escapeHtml(error)}</pre>
      <p style="color:#6b7280;font-size:.875rem;margin-top:1rem">Fix the error above. The page will update automatically.</p>
    </div>`;
  document.body.appendChild(overlay);
}

function clearErrorOverlay(): void {
  document.getElementById('kx-error-overlay')?.remove();
}

function escapeHtml(s: string): string {
  return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
}
