export function initTheme(): void {
  const saved = localStorage.getItem('kx-theme') || 'dark';
  applyTheme(saved);
}

export function applyTheme(theme: string): void {
  document.body.classList.toggle('dark',  theme === 'dark');
  document.body.classList.toggle('light', theme === 'light');
  localStorage.setItem('kx-theme', theme);
  document.documentElement.setAttribute('data-theme', theme);
}

export function toggleTheme(): void {
  const current = localStorage.getItem('kx-theme') || 'dark';
  applyTheme(current === 'dark' ? 'light' : 'dark');
}

export function getCurrentTheme(): string {
  return localStorage.getItem('kx-theme') || 'dark';
}
