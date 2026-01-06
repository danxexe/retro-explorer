import {LitElement, css, html} from '/assets/lit-core.min.js';

export class RexCollection extends LitElement {
  static styles = css`
  ol {
    padding: 0;
  }
  li {
    display: grid;
    grid-template-rows: 1fr;
    grid-template-columns: auto 1fr;

    button {
      grid-column: 1;
      justify-self: start;
    }

    progress {
      z-index: 1;
      grid-row: 1;
      grid-column: 2;
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
      grid-column: 2;
      margin: 0;
      background: transparent;
    }
  }
  `;

  static properties = {
    entries: { type: Array },
  };

  constructor() {
    super();
    this.entries = [];
  }

  render() {
    return html`
    <link rel="stylesheet" href="/assets/pico.min.css">
    <ol>
      ${this.entries.map((entry) => this.renderEntry(entry))}
    </ol>
    `
  }

  renderEntry(entry) {
    return html`
    <li>
      <button @click="${(e) => this.handleClick(e, entry)}">Scan collection</button>
      <progress value="${entry.scanned ?? 0}" max=${entry.total ?? 0}></progress>
      <input type="text" name="path" placeholder="path" value=${entry.path} readonly>
    </li>
    `
  }

  connectedCallback() {
    super.connectedCallback();
  }

  handleClick(_e, entry) {
    this.dispatchEvent(new CustomEvent('rex-scan', {
      composed: true,
      bubbles: true,
      detail: {
        entry,
      },
    }));
  }

  replaceEntry(newEntry) {
    this.entries = this.entries.map(oldEntry => {
      if (newEntry.id === oldEntry.id) {
        return newEntry;
      }
      return oldEntry;
    });
  }
}

customElements.define('rex-collection', RexCollection);
