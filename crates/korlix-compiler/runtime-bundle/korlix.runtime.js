/* Korlix Runtime v0.1.0 — Core + Router + Toast + Overlay + State + Media */
(function(global) {
  'use strict';

  // ── Utilities ────────────────────────────────────────────────────────
  function $(sel, ctx) { return (ctx||document).querySelector(sel); }
  function $$(sel, ctx) { return Array.from((ctx||document).querySelectorAll(sel)); }

  // ── State engine ─────────────────────────────────────────────────────
  function createState(initial) {
    var state = Object.assign({}, initial);
    var bindings = [];

    function update() {
      // Update all data-kx-bind elements
      $$('[data-kx-bind]').forEach(function(el) {
        var key = el.getAttribute('data-kx-bind');
        var val = getNestedValue(state, key);
        el.textContent = val !== undefined ? String(val) : '';
      });
      // Update conditional blocks
      $$('[data-kx-if]').forEach(function(tmpl) {
        var cond = tmpl.getAttribute('data-kx-if');
        try {
          var result = evalExpr(cond, state);
          renderConditional(tmpl, result, true);
        } catch(e) {}
      });
      $$('[data-kx-else]').forEach(function(tmpl) {
        var cond = tmpl.getAttribute('data-kx-else');
        try {
          var result = evalExpr(cond, state);
          renderConditional(tmpl, !result, false);
        } catch(e) {}
      });
    }

    function renderConditional(tmpl, show, isIf) {
      var id = tmpl.getAttribute('data-kx-' + (isIf ? 'if' : 'else')) + '_rendered';
      var existing = tmpl.nextSibling && tmpl.nextSibling._kxCond === id
        ? tmpl.nextSibling : null;
      if (show) {
        if (!existing) {
          var clone = tmpl.content ? tmpl.content.cloneNode(true) : document.createTextNode('');
          var wrapper = document.createElement('div');
          wrapper._kxCond = id;
          wrapper.style.display = 'contents';
          if (tmpl.content) wrapper.appendChild(tmpl.content.cloneNode(true));
          tmpl.parentNode.insertBefore(wrapper, tmpl.nextSibling);
        }
      } else {
        if (existing) existing.remove();
      }
    }

    function proxy(obj) {
      if (typeof Proxy === 'undefined') return obj;
      return new Proxy(obj, {
        set: function(target, key, value) {
          target[key] = value;
          update();
          return true;
        }
      });
    }

    var reactive = proxy(state);
    update();
    return reactive;
  }

  function getNestedValue(obj, path) {
    return path.split('.').reduce(function(o, k) { return o && o[k]; }, obj);
  }

  function evalExpr(expr, state) {
    var keys = Object.keys(state);
    var vals = keys.map(function(k) { return state[k]; });
    try {
      return new Function(...keys, 'return (' + expr + ')').apply(null, vals);
    } catch(e) { return false; }
  }

  // ── Event binding ────────────────────────────────────────────────────
  function bindEvent(selector, event, handler, state) {
    document.querySelectorAll(selector).forEach(function(el) {
      if (!el._kxBound) el._kxBound = {};
      var key = event + ':' + selector;
      if (el._kxBound[key]) return;
      el._kxBound[key] = true;
      el.addEventListener(event, function(e) {
        handler(state || null, e);
      });
    });
  }

  // ── Built-in call() dispatcher ───────────────────────────────────────
  function call(name, args) {
    switch(name) {
      case 'toast':           return Toast.show(args[0], args[1], args[2] || {}); break;
      case 'showToast':       return Toast.show(args[0], args[1], args[2] || {}); break;
      case 'openModal':       return Overlay.openModal(args[0]); break;
      case 'closeModal':      return Overlay.closeModal(args[0]); break;
      case 'openDrawer':      return Overlay.openDrawer(args[0]); break;
      case 'closeDrawer':     return Overlay.closeDrawer(args[0]); break;
      case 'navigate':        return Router.navigate(args[0]); break;
      case 'goBack':          return history.back(); break;
      case 'toggleTheme':     return Theme.toggle(); break;
      case 'scrollTo':        return scrollTo(args[0]); break;
      case 'copyToClipboard': return navigator.clipboard && navigator.clipboard.writeText(args[0]); break;
      case 'log':             console.log.apply(console, args); break;
      default:
        if (typeof window[name] === 'function') window[name].apply(null, args);
    }
  }

  function scrollTo(selector) {
    var el = document.querySelector(selector);
    if (el) el.scrollIntoView({ behavior: 'smooth' });
  }

  // ── Toast ────────────────────────────────────────────────────────────
  var Toast = (function() {
    var container;
    function getContainer(position) {
      var pos = position || 'top-right';
      var id = 'kx-toast-root-' + pos.replace('-', '_');
      var el = document.getElementById(id);
      if (!el) {
        el = document.createElement('div');
        el.id = id;
        el.className = 'kx-toast-root ' + pos;
        document.body.appendChild(el);
      }
      return el;
    }

    function show(type, message, opts) {
      opts = opts || {};
      var position = opts.position || 'top-right';
      var duration = opts.duration !== undefined ? opts.duration : 3000;
      var root = getContainer(position);

      var toast = document.createElement('div');
      toast.className = 'kx-toast ' + (type || 'info');
      toast.setAttribute('role', 'alert');
      toast.setAttribute('aria-live', 'polite');

      var icons = { success:'✓', error:'✕', warning:'⚠', info:'ℹ', loading:'⟳' };
      var icon = document.createElement('span');
      icon.textContent = icons[type] || 'ℹ';
      icon.setAttribute('aria-hidden', 'true');

      var msg = document.createElement('span');
      msg.textContent = message;

      var close = document.createElement('button');
      close.className = 'kx-toast-close';
      close.textContent = '×';
      close.setAttribute('aria-label', 'Dismiss');
      close.addEventListener('click', function() { dismiss(toast); });

      toast.appendChild(icon);
      toast.appendChild(msg);
      toast.appendChild(close);
      root.appendChild(toast);

      if (duration > 0) {
        setTimeout(function() { dismiss(toast); }, duration);
      }
      return toast;
    }

    function dismiss(el) {
      el.style.opacity = '0';
      el.style.transform = 'translateX(100%)';
      el.style.transition = 'opacity .2s, transform .2s';
      setTimeout(function() { if (el.parentNode) el.parentNode.removeChild(el); }, 250);
    }

    return { show: show };
  })();

  // ── Overlay (Modal + Drawer) ─────────────────────────────────────────
  var Overlay = (function() {
    var activeModal = null;
    var prevFocus = null;

    function openModal(id) {
      var el = document.getElementById('kx-modal-' + id);
      if (!el) return;
      prevFocus = document.activeElement;
      var backdrop = document.createElement('div');
      backdrop.className = 'kx-modal-backdrop';
      backdrop.id = 'kx-modal-backdrop-' + id;
      backdrop.addEventListener('click', function(e) {
        if (e.target === backdrop) closeModal(id);
      });
      el.removeAttribute('hidden');
      backdrop.appendChild(el.cloneNode(true));
      // Actually just show the original el inside the backdrop
      el.removeAttribute('hidden');
      document.body.appendChild(backdrop);
      backdrop.querySelector('.kx-modal') && backdrop.querySelector('.kx-modal').focus
        ? backdrop.querySelector('.kx-modal').setAttribute('tabindex', '-1')
        : null;
      activeModal = { id, backdrop };
      document.addEventListener('keydown', onEsc);
    }

    function closeModal(id) {
      var backdrop = document.getElementById('kx-modal-backdrop-' + id);
      if (backdrop) backdrop.remove();
      var el = document.getElementById('kx-modal-' + id);
      if (el) el.setAttribute('hidden', 'true');
      if (prevFocus) prevFocus.focus();
      activeModal = null;
      document.removeEventListener('keydown', onEsc);
    }

    function onEsc(e) {
      if (e.key === 'Escape' && activeModal) closeModal(activeModal.id);
    }

    function openDrawer(id) {
      var el = document.getElementById('kx-drawer-' + id);
      if (!el) return;
      var backdrop = document.createElement('div');
      backdrop.className = 'kx-drawer-backdrop';
      backdrop.id = 'kx-drawer-backdrop-' + id;
      backdrop.addEventListener('click', function() { closeDrawer(id); });
      document.body.appendChild(backdrop);
      el.classList.add('open');
    }

    function closeDrawer(id) {
      var el = document.getElementById('kx-drawer-' + id);
      if (el) el.classList.remove('open');
      var bd = document.getElementById('kx-drawer-backdrop-' + id);
      if (bd) bd.remove();
    }

    return { openModal, closeModal, openDrawer, closeDrawer };
  })();

  // ── Router (SPA) ─────────────────────────────────────────────────────
  var Router = (function() {
    var routes = window.__KORLIX_ROUTES__ || {};
    var currentRoute = null;
    var root = null;

    function init() {
      root = document.getElementById('korlix-root');
      // Intercept all internal link clicks
      document.addEventListener('click', function(e) {
        var a = e.target.closest('[data-kx-link]');
        if (!a) return;
        var href = a.getAttribute('href');
        if (!href || href.startsWith('http') || href.startsWith('//')) return;
        e.preventDefault();
        navigate(href);
      });
      window.addEventListener('popstate', function() {
        renderRoute(location.pathname);
      });
      renderRoute(location.pathname);
    }

    function navigate(path) {
      history.pushState(null, '', path);
      renderRoute(path);
    }

    function matchRoute(path) {
      // Exact match
      if (routes[path]) return { route: routes[path], params: {} };
      // Param match
      for (var pattern in routes) {
        var params = matchPath(pattern, path);
        if (params !== null) return { route: routes[pattern], params };
      }
      return null;
    }

    function matchPath(pattern, path) {
      var pParts = pattern.split('/');
      var uParts = path.split('/');
      if (pParts.length !== uParts.length) return null;
      var params = {};
      for (var i = 0; i < pParts.length; i++) {
        if (pParts[i].startsWith(':')) {
          params[pParts[i].slice(1)] = uParts[i];
        } else if (pParts[i] !== uParts[i]) {
          return null;
        }
      }
      return params;
    }

    function renderRoute(path) {
      var match = matchRoute(path);
      // Update active link states
      document.querySelectorAll('[data-kx-link]').forEach(function(el) {
        el.classList.toggle('active', el.getAttribute('href') === path);
      });
      // Scroll to top
      window.scrollTo(0, 0);
    }

    return { init, navigate, matchRoute };
  })();

  // ── Theme ─────────────────────────────────────────────────────────────
  var Theme = (function() {
    function init() {
      var saved = localStorage.getItem('kx-theme') || 'dark';
      apply(saved);
    }
    function apply(theme) {
      document.body.classList.toggle('dark',  theme === 'dark');
      document.body.classList.toggle('light', theme === 'light');
      localStorage.setItem('kx-theme', theme);
    }
    function toggle() {
      var current = localStorage.getItem('kx-theme') || 'dark';
      apply(current === 'dark' ? 'light' : 'dark');
    }
    return { init, apply, toggle };
  })();

  // ── Media (lazy images) ───────────────────────────────────────────────
  var Media = (function() {
    function init() {
      if ('IntersectionObserver' in window) {
        var obs = new IntersectionObserver(function(entries) {
          entries.forEach(function(entry) {
            if (entry.isIntersecting) {
              var img = entry.target;
              if (img.dataset.src) {
                img.src = img.dataset.src;
                img.removeAttribute('data-src');
              }
              obs.unobserve(img);
            }
          });
        });
        document.querySelectorAll('img[loading="lazy"]').forEach(function(img) {
          obs.observe(img);
        });
      }
    }
    return { init };
  })();

  // ── Pagination ─────────────────────────────────────────────────────────
  var Pagination = (function() {
    function init() {
      document.querySelectorAll('[data-kx-pagination]').forEach(function(el) {
        render(el);
      });
    }

    function render(el) {
      var page    = parseInt(el.dataset.page, 10)    || 1;
      var total   = parseInt(el.dataset.total, 10)   || 0;
      var perPage = parseInt(el.dataset.perPage, 10) || 10;
      var pages   = Math.ceil(total / perPage) || 1;
      el.innerHTML = '';

      function btn(label, page_, disabled, active) {
        var b = document.createElement('button');
        b.textContent = label;
        b.disabled = disabled;
        if (active) b.classList.add('active');
        b.setAttribute('aria-label', 'Page ' + label);
        if (!disabled) {
          b.addEventListener('click', function() {
            el.dataset.page = page_;
            el.dispatchEvent(new CustomEvent('kx:page-change', { detail: { page: page_ }, bubbles: true }));
            render(el);
          });
        }
        return b;
      }

      el.appendChild(btn('‹', page - 1, page <= 1));
      var start = Math.max(1, page - 2), end = Math.min(pages, start + 4);
      if (start > 1) el.appendChild(btn('1', 1, false));
      if (start > 2) { var dots = document.createElement('span'); dots.textContent = '…'; el.appendChild(dots); }
      for (var i = start; i <= end; i++) {
        el.appendChild(btn(String(i), i, false, i === page));
      }
      if (end < pages - 1) { var dots2 = document.createElement('span'); dots2.textContent = '…'; el.appendChild(dots2); }
      if (end < pages) el.appendChild(btn(String(pages), pages, false));
      el.appendChild(btn('›', page + 1, page >= pages));
    }
    return { init, render };
  })();

  // ── HMR client (dev mode) ──────────────────────────────────────────────
  var HMR = (function() {
    function connect(port) {
      try {
        var ws = new WebSocket('ws://localhost:' + (port || 3001) + '/__kx_hmr');
        ws.onmessage = function(e) {
          try {
            var msg = JSON.parse(e.data);
            handle(msg);
          } catch(_) {}
        };
        ws.onclose = function() { setTimeout(function() { connect(port); }, 2000); };
      } catch(_) {}
    }
    function handle(msg) {
      if (msg.type === 'css-update') {
        var links = document.querySelectorAll('link[rel="stylesheet"]');
        links.forEach(function(link) {
          link.href = link.href.split('?')[0] + '?t=' + Date.now();
        });
      } else if (msg.type === 'full-reload') {
        location.reload();
      } else if (msg.type === 'error') {
        showErrorOverlay(msg.error);
      } else if (msg.type === 'clear-error') {
        clearErrorOverlay();
      }
    }
    function showErrorOverlay(error) {
      clearErrorOverlay();
      var overlay = document.createElement('div');
      overlay.id = 'kx-error-overlay';
      overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.92);z-index:99999;padding:2rem;font-family:monospace;color:#f87171;overflow:auto;';
      overlay.innerHTML = '<div style="max-width:800px;margin:0 auto"><h2 style="color:#ef4444;margin-top:0">⚠ Korlix Compile Error</h2><pre style="white-space:pre-wrap;background:#1e1e2e;padding:1.5rem;border-radius:.5rem;color:#f1f5f9">' + escHTML(error) + '</pre><p style="color:#6b7280;font-size:.875rem">Fix the error above to continue.</p></div>';
      document.body.appendChild(overlay);
    }
    function clearErrorOverlay() {
      var el = document.getElementById('kx-error-overlay');
      if (el) el.remove();
    }
    function escHTML(s) { return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;'); }
    return { connect };
  })();

  // ── Mount ──────────────────────────────────────────────────────────────
  function mount(selector) {
    var root = document.querySelector(selector || '#korlix-root');
    if (!root) return;

    // Init all subsystems
    Theme.init();
    Media.init();
    Pagination.init();

    // Wire up modal close buttons
    document.querySelectorAll('[data-kx-close-modal]').forEach(function(el) {
      el.addEventListener('click', function() {
        Overlay.closeModal(el.getAttribute('data-kx-close-modal'));
      });
    });

    // Wire compiled data-on-* handlers.
    document.querySelectorAll('*').forEach(function(el) {
      Array.prototype.slice.call(el.attributes || []).forEach(function(attr) {
        if (attr.name.indexOf('data-on-') !== 0) return;
        var event = attr.name.slice('data-on-'.length);
        if (!event || !attr.value) return;
        if (!el._kxBound) el._kxBound = {};
        var key = 'attr:' + event;
        if (el._kxBound[key]) return;
        el._kxBound[key] = true;
        el.addEventListener(event, function(domEvent) {
          try {
            new Function('event', '__state', 'KorlixRuntime', attr.value)(
              domEvent,
              window.__KORLIX_STATE__ || null,
              KorlixRuntime
            );
          } catch(e) {
            console.error('[Korlix]', e);
          }
        });
      });
    });

    // Handle toast data triggers (static HTML)
    document.querySelectorAll('[data-kx-toast]').forEach(function(el) {
      var type = el.getAttribute('data-kx-toast');
      var msg  = el.getAttribute('data-message');
      if (type && msg && el.style.display === 'none') {
        // Rendered from .klx toast statement — show on mount
        Toast.show(type, msg, {});
      }
    });

    // SPA router (only if routes defined)
    if (window.__KORLIX_ROUTES__) Router.init();

    // HMR in dev mode
    if (window.__KX_DEV__) HMR.connect(window.__KX_WS_PORT__ || 3001);
  }

  // ── Public API ──────────────────────────────────────────────────────────
  var KorlixRuntime = {
    createState: createState,
    bindEvent:   bindEvent,
    call:        call,
    mount:       mount,
    Toast:       Toast,
    Overlay:     Overlay,
    Router:      Router,
    Theme:       Theme,
    HMR:         HMR,
    Pagination:  Pagination,
  };

  global.KorlixRuntime = KorlixRuntime;

  // Auto-mount on DOM ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function() { mount('#korlix-root'); });
  } else {
    mount('#korlix-root');
  }

})(window);
