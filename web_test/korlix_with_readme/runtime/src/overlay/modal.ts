let prevFocus: HTMLElement | null = null;

export function openModal(id: string): void {
  const el = document.getElementById(`kx-modal-${id}`);
  if (!el) return;
  prevFocus = document.activeElement as HTMLElement;

  const backdrop = document.createElement('div');
  backdrop.className = 'kx-modal-backdrop';
  backdrop.id = `kx-modal-backdrop-${id}`;
  backdrop.addEventListener('click', (e) => {
    if (e.target === backdrop) closeModal(id);
  });

  el.removeAttribute('hidden');
  backdrop.appendChild(el);
  document.body.appendChild(backdrop);

  const firstFocusable = el.querySelector<HTMLElement>('button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])');
  firstFocusable?.focus();

  document.addEventListener('keydown', escHandler);
}

export function closeModal(id: string): void {
  const backdrop = document.getElementById(`kx-modal-backdrop-${id}`);
  const original = document.getElementById(`kx-modal-${id}`);

  if (backdrop) {
    const modalEl = backdrop.querySelector(`#kx-modal-${id}`);
    if (modalEl && original) {
      original.setAttribute('hidden', 'true');
      document.body.appendChild(original);
    }
    backdrop.remove();
  }
  prevFocus?.focus();
  prevFocus = null;
  document.removeEventListener('keydown', escHandler);
}

function escHandler(e: KeyboardEvent): void {
  if (e.key !== 'Escape') return;
  const backdrop = document.querySelector<HTMLElement>('.kx-modal-backdrop');
  if (backdrop?.id) closeModal(backdrop.id.replace('kx-modal-backdrop-', ''));
}
