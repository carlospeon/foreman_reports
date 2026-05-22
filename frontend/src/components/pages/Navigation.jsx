import { A } from "@solidjs/router";
import { createSignal, createResource, createEffect, onMount, Show } from "solid-js";

export default function Navigation() {
  const [dark, setDark] = createSignal(false);
  const [errorMsg, setErrorMsg] = createSignal('');

  const apiFetch = async (trigger) => {
    const f = await fetch('/api/refresh');
    const body = await f.text().catch(() => '');
    return { fetch: f, body };
  };

  const [refreshTrigger, setRefreshTrigger] = createSignal();
  const [refreshResource] = createResource(refreshTrigger, apiFetch);

  createEffect(() => {
    if (refreshResource.state === 'ready' && refreshResource().fetch.ok) {
      window.location.reload();
    }
    if (refreshResource.state === 'ready' && !refreshResource().fetch.ok) {
      const status = refreshResource().fetch.status;
      const detail = refreshResource().body || refreshResource().fetch.statusText;
      setErrorMsg(`Error ${status}: ${detail}`);
    }
    if (refreshResource.state === 'errored') {
      setErrorMsg('Network error');
    }
  });

  onMount(() => {
    const saved = localStorage.getItem('theme');
    const prefersDark = saved === 'dark' || (!saved && window.matchMedia('(prefers-color-scheme: dark)').matches);
    if (prefersDark) {
      document.documentElement.dataset.theme = 'dark';
      setDark(true);
    }
  });

  function toggleTheme() {
    const next = !dark();
    setDark(next);
    if (next) {
      document.documentElement.dataset.theme = 'dark';
      localStorage.setItem('theme', 'dark');
    } else {
      delete document.documentElement.dataset.theme;
      localStorage.setItem('theme', 'light');
    }
  }

  function handleRefresh() {
    if (refreshResource.loading) return;
    setRefreshTrigger(Date.now());
  }

  return (
    <>
    <div class="navbar">
        <input type="checkbox" name="" id=""/>
        <div class="hamburger-lines">
            <span class="line line1"></span>
            <span class="line line2"></span>
            <span class="line line3"></span>
        </div>
        <ul class="menu-items">
          <li><A activeClass="active" href="/home">Home</A></li>
          <li><A activeClass="active" href="/facts">Facts</A></li>
          <li><A activeClass="active" href="/updated">Updated</A></li>
          <li><A activeClass="active" href="/erratas">Erratas</A></li>
          <li><A activeClass="active" href="/inventory">Inventory</A></li>
          <li class="refresh-item">
            <a
              class={`cursor-pointer refresh-btn ${refreshResource.loading ? 'refreshing' : ''}`}
              onClick={handleRefresh}
              aria-disabled={refreshResource.loading}
            >
              Refresh
              <svg class="refresh-icon" viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M23 4v6h-6M1 20v-6h6"/>
                <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
              </svg>
            </a>
          </li>
          {/*<li><a class="cursor-pointer" onClick={toggleTheme}>{dark() ? 'Light' : 'Dark'}</a></li>*/}
        </ul>
    </div>
    <Show when={errorMsg()}>
      <div class="toast-error">
        <span>{errorMsg()}</span>
        <button class="toast-close" onClick={() => setErrorMsg('')}>&#x2715;</button>
      </div>
    </Show>
    </>
  );
}
