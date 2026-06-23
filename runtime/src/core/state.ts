export type StateValue = string | number | boolean | null | object | any[];

export interface ReactiveState {
  [key: string]: StateValue;
}

type Subscriber = () => void;

const subscribers = new Set<Subscriber>();

export function createState(initial: ReactiveState): ReactiveState {
  if (typeof Proxy === 'undefined') return { ...initial };

  return new Proxy({ ...initial }, {
    set(target, key: string, value) {
      target[key] = value;
      subscribers.forEach(fn => fn());
      updateBindings(target);
      return true;
    }
  });
}

function updateBindings(state: ReactiveState) {
  document.querySelectorAll<HTMLElement>('[data-kx-bind]').forEach(el => {
    const key = el.getAttribute('data-kx-bind')!;
    const val = getPath(state, key);
    if (val !== undefined) el.textContent = String(val);
  });
}

function getPath(obj: Record<string, any>, path: string): any {
  return path.split('.').reduce((o, k) => o?.[k], obj);
}

export function subscribe(fn: Subscriber): () => void {
  subscribers.add(fn);
  return () => subscribers.delete(fn);
}
