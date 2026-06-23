export function updateDOM(el: Element, html: string): void {
  const tmp = document.createElement('div');
  tmp.innerHTML = html;
  patch(el, tmp);
}

function patch(target: Element, source: Element): void {
  const targetChildren = Array.from(target.childNodes);
  const sourceChildren = Array.from(source.childNodes);
  const maxLen = Math.max(targetChildren.length, sourceChildren.length);

  for (let i = 0; i < maxLen; i++) {
    const t = targetChildren[i];
    const s = sourceChildren[i];

    if (!s) { t.parentNode?.removeChild(t); continue; }
    if (!t) { target.appendChild(s.cloneNode(true)); continue; }

    if (t.nodeType !== s.nodeType) {
      target.replaceChild(s.cloneNode(true), t);
    } else if (t.nodeType === Node.TEXT_NODE) {
      if (t.textContent !== s.textContent) t.textContent = s.textContent;
    } else if (t.nodeType === Node.ELEMENT_NODE) {
      patchElement(t as Element, s as Element);
    }
  }
}

function patchElement(target: Element, source: Element): void {
  if (target.tagName !== source.tagName) {
    target.parentNode?.replaceChild(source.cloneNode(true), target);
    return;
  }
  // Patch attributes
  Array.from(source.attributes).forEach(attr => {
    if (target.getAttribute(attr.name) !== attr.value) {
      target.setAttribute(attr.name, attr.value);
    }
  });
  Array.from(target.attributes).forEach(attr => {
    if (!source.hasAttribute(attr.name)) target.removeAttribute(attr.name);
  });
  patch(target, source);
}
