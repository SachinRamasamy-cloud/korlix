export function extractParams(pattern: string, path: string): Record<string, string> {
  const params: Record<string, string> = {};
  const pp = pattern.split('/'), up = path.split('/');
  pp.forEach((seg, i) => { if (seg.startsWith(':')) params[seg.slice(1)] = up[i] || ''; });
  return params;
}

export function initPagination(): void {
  document.querySelectorAll<HTMLElement>('[data-kx-pagination]').forEach(el => {
    renderPagination(el);
  });
}

function renderPagination(el: HTMLElement): void {
  const page    = parseInt(el.dataset.page    || '1', 10);
  const total   = parseInt(el.dataset.total   || '0', 10);
  const perPage = parseInt(el.dataset.perPage || '10', 10);
  const pages   = Math.max(1, Math.ceil(total / perPage));
  el.innerHTML  = '';

  const mkBtn = (label: string, p: number, disabled = false, active = false) => {
    const btn = document.createElement('button');
    btn.textContent = label;
    btn.disabled = disabled;
    if (active) btn.classList.add('active');
    btn.setAttribute('aria-label', `Page ${label}`);
    if (!disabled) btn.addEventListener('click', () => {
      el.dataset.page = String(p);
      el.dispatchEvent(new CustomEvent('kx:page-change', { detail: { page: p }, bubbles: true }));
      renderPagination(el);
    });
    return btn;
  };

  el.appendChild(mkBtn('‹', page - 1, page <= 1));
  const start = Math.max(1, page - 2), end = Math.min(pages, start + 4);
  if (start > 1) el.appendChild(mkBtn('1', 1));
  if (start > 2) { const s = document.createElement('span'); s.textContent = '…'; el.appendChild(s); }
  for (let i = start; i <= end; i++) el.appendChild(mkBtn(String(i), i, false, i === page));
  if (end < pages - 1) { const s = document.createElement('span'); s.textContent = '…'; el.appendChild(s); }
  if (end < pages) el.appendChild(mkBtn(String(pages), pages));
  el.appendChild(mkBtn('›', page + 1, page >= pages));
}
