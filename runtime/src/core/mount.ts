import { initTheme } from '../theme/index';
import { initMedia } from '../media/image';
import { initPagination } from '../router/params';

export function mount(selector = '#korlix-root'): void {
  const root = document.querySelector(selector);
  if (!root) return;

  initTheme();
  initMedia();
  initPagination();

  // Wire data-on-* event handlers
  document.querySelectorAll<HTMLElement>('[data-on-click]').forEach(el => {
    el.addEventListener('click', () => {
      const code = el.getAttribute('data-on-click');
      if (code) try { new Function(code)(); } catch(e) { console.error('[Korlix]', e); }
    });
  });
}
