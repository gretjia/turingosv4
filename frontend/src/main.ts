// TuringOS Phase 7 Web Components entry — W0 scaffold only.
// Real block-type components land in W3.
class TuringOSRoot extends HTMLElement {
  connectedCallback() {
    this.innerHTML = "<h1>TuringOS Phase 7 — placeholder root component</h1>";
  }
}
customElements.define("turingos-root", TuringOSRoot);
