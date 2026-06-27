/**
 * Korlix API client — handles reactive GET queries and write mutations.
 *
 * Each `get name "url"` declaration registers a named query.
 * The runtime exposes:
 *   - `name`         → the fetched data (array or object)
 *   - `nameLoading`  → boolean
 *   - `nameError`    → string | null
 *
 * Inside actions, `post/put/patch/delete "url" body` and `reload name`
 * map to `request()` and `reload()` respectively.
 */

type ApiQueryState<T = unknown> = {
  data: T | null;
  loading: boolean;
  error: string | null;
  url: string;
};

/** Registry of all named queries declared via `get name "url"`. */
const queries = new Map<string, ApiQueryState>();

// ── HTTP helpers ───────────────────────────────────────────────────────────

async function fetchJson(url: string, init?: RequestInit): Promise<unknown> {
  const response = await fetch(url, {
    ...init,
    headers: {
      "Content-Type": "application/json",
      ...((init?.headers as Record<string, string>) || {}),
    },
  });

  if (!response.ok) {
    throw new Error(`${response.status} ${response.statusText}`);
  }

  const contentType = response.headers.get("content-type") ?? "";

  if (contentType.includes("application/json")) {
    return response.json();
  }

  return response.text();
}

// ── Reactive state integration ─────────────────────────────────────────────

/**
 * Write a value into the Korlix reactive state.
 *
 * Priority:
 *   1. `KorlixRuntime.state[name]` if a reactive proxy is present.
 *   2. Falls back to `window[name]` for plain global access.
 *
 * After writing, triggers `updateBindings()` and `render()` if available.
 */
function setGlobalValue(name: string, value: unknown): void {
  const runtime = (window as any).KorlixRuntime;

  if (runtime?.state) {
    runtime.state[name] = value;
  } else {
    (window as any)[name] = value;
  }

  if (typeof runtime?.updateBindings === "function") {
    runtime.updateBindings();
  }

  if (typeof runtime?.render === "function") {
    runtime.render();
  }
}

// ── Query (GET) ────────────────────────────────────────────────────────────

/**
 * Fetch `url` and expose the result as reactive state under `name`.
 *
 * Automatically sets `nameLoading` and `nameError` companion values.
 * Safe to call multiple times — subsequent calls replace the previous state.
 */
async function runQuery(name: string, url: string): Promise<unknown> {
  const state: ApiQueryState = {
    data: null,
    loading: true,
    error: null,
    url,
  };

  queries.set(name, state);

  // Optimistically clear data and set loading flag
  setGlobalValue(name, []);
  setGlobalValue(`${name}Loading`, true);
  setGlobalValue(`${name}Error`, null);

  try {
    const data = await fetchJson(url);

    state.data = data;
    state.loading = false;
    state.error = null;

    setGlobalValue(name, data);
    setGlobalValue(`${name}Loading`, false);
    setGlobalValue(`${name}Error`, null);

    return data;
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);

    state.loading = false;
    state.error = message;

    setGlobalValue(`${name}Loading`, false);
    setGlobalValue(`${name}Error`, message);

    // Re-throw so callers can handle errors if needed
    throw err;
  }
}

// ── Mutation (POST / PUT / PATCH / DELETE) ─────────────────────────────────

/**
 * Send a write mutation to `url` using `method`.
 * `body` is JSON-serialised when provided (undefined → no request body).
 */
async function request(
  method: string,
  url: string,
  body?: unknown
): Promise<unknown> {
  return fetchJson(url, {
    method,
    body: body === undefined ? undefined : JSON.stringify(body),
  });
}

// ── Reload ────────────────────────────────────────────────────────────────

/**
 * Re-fetch a previously declared `get` query by name.
 *
 * Throws if no query with that name has been registered.
 * Edge case: if the runtime registers the query asynchronously and `reload`
 * is called before the first fetch completes, this will simply re-trigger
 * the same URL — this is safe and idempotent.
 */
async function reload(name: string): Promise<unknown> {
  const existing = queries.get(name);

  if (!existing) {
    // Graceful degradation: warn instead of hard-throwing so the UI
    // doesn't break if reload is called before the initial query registers.
    console.warn(
      `[Korlix] reload("${name}"): no query registered with that name. ` +
        `Did you forget \`get ${name} "url"\`?`
    );
    return;
  }

  return runQuery(name, existing.url);
}

// ── Public API ────────────────────────────────────────────────────────────

export const api = {
  /** Register and run a named GET query. Called automatically at page init. */
  query: runQuery,
  /** Send a write mutation (POST/PUT/PATCH/DELETE). Called from action bodies. */
  request,
  /** Re-fetch a named query. Called from `reload name` statements. */
  reload,
};
