import { A } from "@solidjs/router";
import { createSignal, onMount } from "solid-js";

export default function Navigation() {
  const [dark, setDark] = createSignal(false);

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

  return (
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
          {/*<li><a class="cursor-pointer" onClick={toggleTheme}>{dark() ? 'Light' : 'Dark'}</a></li>*/}
        </ul>
    </div>
  );
}
