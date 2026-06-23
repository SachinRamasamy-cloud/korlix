export function lazyLoad(img: HTMLImageElement, src: string): void {
  img.dataset.src = src;
  img.src = 'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"%3E%3C/svg%3E';
  img.classList.add('kx-lazy-placeholder');
}
