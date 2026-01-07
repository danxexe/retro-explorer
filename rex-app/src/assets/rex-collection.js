import {LitElement, css, html} from '/assets/lit-core.min.js';
import picoCss from '/assets/pico.min.css' with { type: 'css' };
import picoColorsCss from '/assets/pico.colors.min.css' with { type: 'css' };

export class RexCollection extends LitElement {
  static styles = [picoCss, picoColorsCss, css`
  :host {
    --pico-form-element-spacing-vertical: 0.4rem;
  }

  button {
    --pico-primary-background: transparent;
    --pico-secondary-background: transparent;
    --pico-primary-border: var(--button-color);
    --pico-primary-hover-background: var(--button-color);
    --pico-primary-hover-border: var(--button-color);

    --button-color: #0172ad;

    &.scan {
      --button-color: var(--pico-color-jade-400);
    }

    &.delete {
      --button-color: var(--pico-color-red-500);
    }
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

    .buttons {
      grid-row: 1;
      grid-column: 2;
      justify-self: start;
    }
  }
  `];

  static properties = {
    entries: { type: Array },
    isDeleteDialogOpen: { type: Boolean, state: true },
    selectedEntries: { type: Array, state: true },
  };

  constructor() {
    super();
    this.entries = [];
    this.isDeleteDialogOpen = false;
    this.selectedEntries = [];
  }

  render() {
    return html`
    <dialog
      id="delete-dialog"
      data-click-action="cancel-dialog"
      ?open="${this.isDeleteDialogOpen}"
      @click="${this.#handleDialogClick}"
    >
      <article>
        <h1>Remove from collection?</h1>
        <ul>
          ${this.selectedEntries.map(entry => html`
            <li><b>${entry.path}</b><li>
          `)}
        </ul>
        <p>All scanned metadata related to these paths will be deleted.</p>
        <p>If you wish to add it again later, the folders will need to be re-scanned.</p>
        <footer>
          <button class="secondary" data-click-action="cancel-dialog">Cancel</button>
          <button class="delete" data-click-action="confirm-delete">
            Remove <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-folder-x"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M13.5 19h-8.5a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4" /><path d="M22 22l-5 -5" /><path d="M17 22l5 -5" /></svg>
          </button>
        </footer>
      </article>
    </dialog>
    <header>
      <h1>Collection</h1>
      <button @click="${this.#handleAddClick}">
        Add folder <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-folder-plus"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5" /><path d="M16 19h6" /><path d="M19 16v6" /></svg>
      </button>
    </header>
    <ol>
      ${this.entries.map(entry => this.#renderEntry(entry))}
    </ol>
    `
  }

  #renderEntry(entry) {
    return html`
    <li>
      <progress value="${entry.scanned ?? 0}" max=${entry.total ?? 0}></progress>
      <input type="text" name="path" placeholder="path" value=${entry.path} readonly>
      <div class="buttons">
        <button class="scan" @click="${(e) => this.#handleScanClick(e, entry)}">
          Scan <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-refresh"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4" /><path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 4v-4h-4" /></svg>
        </button>
        <button class="delete" @click="${(e) => this.#handleDeleteClick(e, entry)}">
          Remove <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-folder-x"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M13.5 19h-8.5a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4" /><path d="M22 22l-5 -5" /><path d="M17 22l5 -5" /></svg>
        </button>
      </div>
    </li>
    `
  }

  connectedCallback() {
    super.connectedCallback();
  }

  #handleDialogClick(e) {
    const action = e.target.dataset.clickAction;
    if (action) {
      this.#actionTriggered(action);
    }
  }

  #handleAddClick(_) {
    this.dispatchEvent(new CustomEvent('rex-add-collection', {
      composed: true, bubbles: true,
    }));
  }

  #handleScanClick(_, entry) {
    this.dispatchEvent(new CustomEvent('rex-scan-collection', {
      composed: true, bubbles: true, detail: {
        entry,
      },
    }));
  }
  #handleDeleteClick(_, entry) {
    this.selectedEntries = [entry];
    this.isDeleteDialogOpen = true;
  }

  #actionTriggered(action) {
    if (action === 'cancel-dialog') {
      this.isDeleteDialogOpen = false;
    } else if (action === 'confirm-delete') {
      this.dispatchEvent(new CustomEvent('rex-delete-collection', {
        composed: true, bubbles: true, detail: {
          entries: this.selectedEntries,
        },
      }));
      this.isDeleteDialogOpen = false;
    }
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
