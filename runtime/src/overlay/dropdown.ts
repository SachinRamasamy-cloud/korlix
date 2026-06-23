export function initDropdowns(): void {
  document.addEventListener('click', (e) => {
    const trigger = (e.target as Element).closest('[data-kx-dropdown]');
    if (!trigger) {
      document.querySelectorAll('.kx-dropdown.open').forEach(d => d.classList.remove('open'));
      return;
    }
    const id = trigger.getAttribute('data-kx-dropdown');
    const menu = document.getElementById(`kx-dropdown-${id}`);
    if (menu) menu.classList.toggle('open');
  });
}
