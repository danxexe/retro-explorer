import {LitElement, css, html} from '/assets/lit-core.min.js';
import picoCss from '/assets/pico.min.css' with { type: 'css' };

export class RexCollection extends LitElement {
  static styles = [picoCss, css`
  :host {
    --pico-form-element-spacing-vertical: 0.4rem;
  }

  header {
    display: grid;
    grid-template-rows: 1fr;
    grid-template-columns: 1fr auto;

    button {
      padding: 0 var(--pico-form-element-spacing-horizontal);
      margin-bottom: var(--pico-typography-spacing-vertical);
    }
  }
  ol {
    padding: 0;
  }
  li {
    display: grid;
    grid-template-rows: 1fr;
    grid-template-columns: 1fr auto;

    button {
      grid-column: 2;
      justify-self: start;
      --pico-primary-background: #00A66E;
      --pico-primary-border: #00A66E;
      --pico-primary-hover-background: #00B478;
      --pico-primary-hover-border: #00B478;
    }

    progress {
      z-index: 1;
      grid-row: 1;
      grid-column: 1;
      margin: 0;
      opacity: 50%;
      block-size: auto;

      &.active {
        opacity: 100%;
      }
    }

    input[type=text] {
      z-index: 2;
      grid-row: 1;
      grid-column: 1;
      margin: 0;
      background: transparent;
    }
  }
  `];

  static properties = {
    entries: { type: Array },
  };

  constructor() {
    super();
    this.entries = [];
  }

  render() {
    return html`
    <header>
      <h1>Collection</h1>
      <button @click="${this.#handleAddClick}">
        Add <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-folder-plus"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5" /><path d="M16 19h6" /><path d="M19 16v6" /></svg>
      </button>
    </header>
    <ol>
      ${this.entries.map((entry) => this.#renderEntry(entry))}
    </ol>
    `
  }

  #renderEntry(entry) {
    return html`
    <li>
      <progress value="${entry.scanned ?? 0}" max=${entry.total ?? 0}></progress>
      <input type="text" name="path" placeholder="path" value=${entry.path} readonly>
      <button @click="${(e) => this.#handleScanClick(e, entry)}">
        Scan <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-refresh"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4" /><path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 4v-4h-4" /></svg>
      </button>
    </li>
    `
  }

  connectedCallback() {
    super.connectedCallback();
  }

  #handleAddClick(_e) {
    this.dispatchEvent(new CustomEvent('rex-add-collection', {
      composed: true, bubbles: true,
    }));
  }

  #handleScanClick(_e, entry) {
    this.dispatchEvent(new CustomEvent('rex-scan-collection', {
      composed: true, bubbles: true, detail: {
        entry,
      },
    }));
  }

  updateEntryBy(cb, updates) {
    this.entries = this.entries.map(entry => {
      if (cb(entry)) {
        return {...entry, ...updates};
      }
      return entry;
    });
  }
}

customElements.define('rex-collection', RexCollection);
