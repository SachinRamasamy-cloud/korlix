export function initMedia(): void {
  if (!('IntersectionObserver' in window)) return;

  const obs = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (!entry.isIntersecting) return;
      const img = entry.target as HTMLImageElement;
      if (img.dataset.src) {
        img.src = img.dataset.src;
        img.removeAttribute('data-src');
        img.classList.remove('kx-lazy-placeholder');
      }
      obs.unobserve(img);
    });
  }, { rootMargin: '50px' });

  document.querySelectorAll<HTMLImageElement>('img[data-src]').forEach(img => obs.observe(img));
}
