import {LitElement, css, html} from '/assets/lit-core.min.js';

export class RexTab extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }

    details {
      display: contents;

      &[open]::details-content {
        display: contents;
      }

      & > summary {
        grid-row: 1;

        &::after {
          display: none;
        }
      }

      &[open] > section {
        grid-row: 2;
        grid-column: 1 / -1;
        display: flex;
        flex-direction: column;
      }
    }

    details {
      & > summary {
        white-space: nowrap;
        cursor: pointer;
        padding: 0.5rem 1.5rem;
        border-radius: 6px 6px 0 0;
        text-align: center;
        margin: 0;
        border: 1px solid var(--pico-mark-background-color);
        list-style: none;
        line-height: 1rem;
      }

      &[open] {
        & > summary {
          --pico-accordion-open-summary-color: #fff;
          background-color: var(--pico-mark-background-color);
          pointer-events: none;
        }
      }

      & > summary:hover {
        background-color: var(--pico-secondary-background);
      }

      & > section {
        margin: 0;
        border-top: 1px solid var(--pico-primary-border);

        & > iframe {
          border: none;
          width: 100%;
          height: 100%;
        }
      }
    }
  `;

  static properties = {
    name: { type: String },
    label: { type: String },
    group: { type: String },
    src: { type: String, reflect: true },
    open: { type: Boolean, reflect: true },
  };

  #storageKey;

  constructor() {
    super();
    this.label = '';
  }

  firstUpdated(_) {
    this.#storageKey = `rex-tab:${this.name}`;
    const savedUrl = localStorage.getItem(`${this.#storageKey}:url`);
    if (savedUrl) {
      this.src = savedUrl;
    }
  }

  onToggleHandler(e) {
    const type = e.target.open ? 'toggleOn' : 'toggleOff';

    const evt = new CustomEvent(type, {
      composed: true,
      bubbles: true,
      cancelable: true,
      detail: {
        group: this.group,
      },
    });

    this.dispatchEvent(evt);

    if (evt.defaultPrevented) {
      e.target.setAttribute('open', this.open);
    } else {
      this.open = e.target.open;
    }
  }

  render() {
    return html`
      <details data-plugin="test" name="${this.group}" @toggle="${this.onToggleHandler}" ?open=${this.open}>
        <summary>${this.label}</summary>
        <section>
          <iframe tabindex="-1" src="${this.src}" name="${this.name}"></iframe>
        </section>
      </details>
    `
  };

  async connectedCallback() {
    super.connectedCallback();

    setTimeout(() => {
      const iframe = this.renderRoot.querySelector('iframe');
      const win = iframe?.contentWindow;

      win?.addEventListener('keydown', function(e) {
        win.parent.postMessage({
            type: 'keydown',
            key: e.key,
            keyCode: e.keyCode,
            ctrlKey: e.ctrlKey,
            shiftKey: e.shiftKey,
            altKey: e.altKey,
            metaKey: e.metaKey,
        }, '*');

        if ((e.ctrlKey || e.metaKey) && e.key === "p") {
          e.preventDefault();
        }
      });

      win?.addEventListener('error', function(e) {
        win.parent.postMessage({
          type: 'error',
          message: e.message,
          filename: e.filename,
          lineno: e.lineno,
          colno: e.colno,
        }, '*');
      });

      window.addEventListener('message', (event) => {
        if (event.data.tabName !== this.name) return;

        if (event.data.type === 'rex-tab:should-save-position') {
          this.#savePosition(event.data);
        } else if (event.data.type === 'rex-tab:should-load-position') {
          this.postMessage(this.#loadPosition())
        };
      });
    });
  }

  postMessage(message) {
    this.renderRoot.querySelector('iframe')?.contentWindow.postMessage(message, '*');
  }

  #loadPosition() {
    const scrollY = localStorage.getItem(`${this.#storageKey}:scrollY`);

    return {
      tabName: this.name,
      type: 'rex-tab:load-position',
      tabUrl: localStorage.getItem(`${this.#storageKey}:url`),
      scrollY: scrollY ? parseInt(scrollY) : null,
    };
  }

  #savePosition(data) {
    localStorage.setItem(`${this.#storageKey}:url`, data.tabUrl);
    localStorage.setItem(`${this.#storageKey}:scrollY`, data.scrollY);
  }
}

customElements.define('rex-tab', RexTab);
