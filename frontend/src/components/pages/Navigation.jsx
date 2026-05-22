import { A } from "@solidjs/router";

export default function Navigation() {
  return (
    <div class="navbar">
      {/* <div class="navbar-container container"> */}
        <input type="checkbox" name="" id=""/>
        <div class="hamburger-lines">
            <span class="line line1"></span>
            <span class="line line2"></span>
            <span class="line line3"></span>
        </div>
        <ul class="menu-items">
    {/* <div id="navigationmenu">
      <ul id="navigationlist"> */}
          <li><A activeClass="active" href="/home">Home</A></li>
          <li><A activeClass="active" href="/facts">Facts</A></li>
          <li><A activeClass="active" href="/updated">Updated</A></li>
          <li><A activeClass="active" href="/erratas">Erratas</A></li>
          <li><A activeClass="active" href="/inventory">Inventory</A></li>
        </ul>
      {/* </div> */}
    </div>
  );
}
