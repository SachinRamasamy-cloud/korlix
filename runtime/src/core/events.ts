export function bindEvents(
  selector: string,
  event: string,
  handler: (state: any, e: Event) => void,
  state: any
): void {
  document.querySelectorAll(selector).forEach(el => {
    el.addEventListener(event, (e) => handler(state, e));
  });
}

export function delegateEvent(
  root: Element | Document,
  event: string,
  selector: string,
  handler: (el: Element, e: Event) => void
): void {
  root.addEventListener(event, (e) => {
    const target = (e.target as Element).closest(selector);
    if (target) handler(target, e);
  });
}
