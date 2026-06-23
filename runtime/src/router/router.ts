interface RouteEntry { id: string; path: string; }
interface Routes { [pattern: string]: RouteEntry; }

declare global {
  interface Window { __KORLIX_ROUTES__?: Routes; }
}

export function initRouter(): void {
  const routes = window.__KORLIX_ROUTES__;
  if (!routes) return;

  document.addEventListener('click', (e) => {
    const a = (e.target as Element).closest('[data-kx-link]') as HTMLAnchorElement | null;
    if (!a) return;
    const href = a.getAttribute('href');
    if (!href || href.startsWith('http') || href.startsWith('//') || href.startsWith('#')) return;
    e.preventDefault();
    navigate(href);
  });

  window.addEventListener('popstate', () => renderRoute(location.pathname));
  renderRoute(location.pathname);
}

export function navigate(path: string): void {
  history.pushState(null, '', path);
  renderRoute(path);
  window.scrollTo(0, 0);
}

function renderRoute(path: string): void {
  // Update active links
  document.querySelectorAll<HTMLAnchorElement>('[data-kx-link]').forEach(el => {
    el.classList.toggle('active', el.getAttribute('href') === path);
    el.setAttribute('aria-current', el.getAttribute('href') === path ? 'page' : 'false');
  });
}

export function matchRoute(path: string): { entry: RouteEntry; params: Record<string, string> } | null {
  const routes = window.__KORLIX_ROUTES__;
  if (!routes) return null;

  if (routes[path]) return { entry: routes[path], params: {} };

  for (const [pattern, entry] of Object.entries(routes)) {
    const params = matchPattern(pattern, path);
    if (params) return { entry, params };
  }
  return null;
}

function matchPattern(pattern: string, path: string): Record<string, string> | null {
  const pp = pattern.split('/'), up = path.split('/');
  if (pp.length !== up.length) return null;
  const params: Record<string, string> = {};
  for (let i = 0; i < pp.length; i++) {
    if (pp[i].startsWith(':')) { params[pp[i].slice(1)] = up[i]; }
    else if (pp[i] !== up[i]) return null;
  }
  return params;
}
