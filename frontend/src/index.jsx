/* @refresh reload */
import { render } from 'solid-js/web';
import { HashRouter, Route, Navigate } from "@solidjs/router";

import './index.css';
import Home from './components/pages/Home';
import Login from './components/pages/Login';
import Updated from './components/pages/Updated';
import Hardware from './components/pages/Hardware';
import Domains from './components/pages/Domains';
import Inventory from './components/pages/Inventory';
import Facts from './components/pages/Facts';
import Erratas from './components/pages/Erratas';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(
  () => (
    //<Router source={hashIntegration()}>
    <HashRouter>
      <Route path="/" component={() => <Navigate href="/home"/>}/>
      <Route path="/home" component={Home} />
      <Route path="/login" component={Login} />
      <Route path="/updated" component={Updated} />
      <Route path="/updated/:ts" component={Updated} />
      <Route path="/hardware" component={Hardware} />
      <Route path="/domains" component={Domains} />
      <Route path="/inventory" component={Inventory} />
      <Route path="/facts" component={Facts} />
      <Route path="/facts/:fkey" component={Facts} />
      <Route path="/facts/:fkey/:fvalue" component={Facts} />
      <Route path="/erratas" component={Erratas} />
      <Route path="/erratas/:ts" component={Erratas} />
    </HashRouter>
  ),
  root
);
