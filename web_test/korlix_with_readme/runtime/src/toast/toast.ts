export type ToastType = 'success' | 'error' | 'warning' | 'info' | 'loading';

interface ToastOptions {
  position?: string;
  duration?: number;
  id?: string;
}

function getContainer(position: string): HTMLElement {
  const id = `kx-toast-root-${position.replace('-', '_')}`;
  let el = document.getElementById(id);
  if (!el) {
    el = document.createElement('div');
    el.id = id;
    el.className = `kx-toast-root ${position}`;
    document.body.appendChild(el);
  }
  return el;
}

export function showToast(type: ToastType, message: string, opts: ToastOptions = {}): HTMLElement {
  const pos      = opts.position || 'top-right';
  const duration = opts.duration !== undefined ? opts.duration : 3000;
  const root     = getContainer(pos);
  const icons: Record<string, string> = { success:'✓', error:'✕', warning:'⚠', info:'ℹ', loading:'⟳' };

  const toast = document.createElement('div');
  toast.className = `kx-toast ${type}`;
  toast.setAttribute('role', 'alert');
  toast.setAttribute('aria-live', 'polite');
  if (opts.id) toast.id = `kx-toast-${opts.id}`;

  const iconEl  = document.createElement('span');
  iconEl.textContent = icons[type] || 'ℹ';
  iconEl.setAttribute('aria-hidden', 'true');

  const msgEl = document.createElement('span');
  msgEl.textContent = message;

  const closeBtn = document.createElement('button');
  closeBtn.className = 'kx-toast-close';
  closeBtn.textContent = '×';
  closeBtn.setAttribute('aria-label', 'Dismiss notification');
  closeBtn.addEventListener('click', () => dismiss(toast));

  toast.append(iconEl, msgEl, closeBtn);
  root.appendChild(toast);

  if (duration > 0) setTimeout(() => dismiss(toast), duration);
  return toast;
}

export function dismissToast(id: string): void {
  const el = document.getElementById(`kx-toast-${id}`);
  if (el) dismiss(el);
}

function dismiss(el: HTMLElement): void {
  el.style.cssText += ';opacity:0;transform:translateX(1rem);transition:opacity .2s,transform .2s';
  setTimeout(() => el.parentNode?.removeChild(el), 250);
}
