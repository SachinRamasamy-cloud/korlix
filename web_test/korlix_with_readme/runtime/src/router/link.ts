export function initLinks(): void {
  document.querySelectorAll<HTMLAnchorElement>('[data-kx-link]').forEach(link => {
    link.addEventListener('click', (e) => {
      const href = link.getAttribute('href');
      if (href && !href.startsWith('http')) {
        e.preventDefault();
        import('./router').then(({ navigate }) => navigate(href));
      }
    });
  });
}
