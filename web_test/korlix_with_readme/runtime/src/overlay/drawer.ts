export function openDrawer(id: string): void {
  const el = document.getElementById(`kx-drawer-${id}`);
  if (!el) return;
  const backdrop = document.createElement('div');
  backdrop.className = 'kx-drawer-backdrop';
  backdrop.id = `kx-drawer-backdrop-${id}`;
  backdrop.addEventListener('click', () => closeDrawer(id));
  document.body.appendChild(backdrop);
  el.classList.add('open');
}

export function closeDrawer(id: string): void {
  const el = document.getElementById(`kx-drawer-${id}`);
  if (el) el.classList.remove('open');
  const bd = document.getElementById(`kx-drawer-backdrop-${id}`);
  if (bd) bd.remove();
}
