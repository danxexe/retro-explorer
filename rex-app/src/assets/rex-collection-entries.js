import {LitElement, css, html} from '/assets/lit-core.min.js';
import picoCss from '/assets/pico.min.css' with { type: 'css' };
import picoColorsCss from '/assets/pico.colors.min.css' with { type: 'css' };

export class RexCollectionEntries extends LitElement {
  static styles = [picoCss, picoColorsCss, css`
  img {
    width: 100%;
  }

  input[type=search] {
     position: sticky;
     top: 0.5rem;
  }

  .tag {
    background-color: var(--pico-color-slate-800);
    font-size: 70%;
    padding: 0rem 0.3rem 0.1rem;
    border-radius: 0.3rem;

    &.console {
      background-color: var(--pico-color-violet-900);
    }
  }

  .title {
    word-break: break-word;
  }

  ol {
    padding: 0;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    gap: 1rem;
  }

  li {
    list-style: none;

    & article {
      height: 100%;
      margin: 0;
      padding: 0rem 0.8rem 0.6rem;

      & header {
        margin: 0;
        padding: 0;
        background-color: transparent;
      }
    }
  }
  `];

  static properties = {
    entries: { type: Array },
    // visibleEntries: { type: Array, state: true },
  }

  constructor() {
    super();
    this.entries = [];
    // this.visibleEntries = [];
  }

  render() {
    return html`
    <input type="search" @input="${this.#handleSearch}" autofocus>
    <ol>
      ${this.entries.map(entry => this.#renderEntry(entry))}
    </ol>
    `
  }

  #renderEntry(entry) {
    const [gameName, consoleName, tags] = this.#normalizeEntryName(entry);

    return html`
    <li data-path="${entry.fs_path.toLowerCase()}">
      <article>
        <div>${this.#renderEntryImage(entry)}
        <header>
          ${tags.map(tag => html`
            <span class="tag">${tag}</span>
          `)}
        </header>
        <div class="title">${gameName} <span class="tag console">${consoleName}</span></div>
        </div>
      </article>
    </li>
    `
  }

  #renderEntryImage(entry) {
    if (entry.icon) {
      return html`<img class="screenshot" src="http://media.retroachievements.org${entry.icon}">`;
    } else {
      return html`<img class="screenshot" src="http://media.retroachievements.org/Images/000001.png">`;
    }
  }

  /**
   * TODO: move logic to Rust and persist
   *
   * @param {String} path
   * @returns {[String, String, Array<String>]}
   */
  #normalizeEntryName(entry) {
    const path = entry.fs_path;
    const parts = path.split('/');
    const consoleName = parts[0];
    const filename = parts[parts.length - 1];
    const filenameParts = filename.split('.');
    const ext = filenameParts.pop()
    const basename = filenameParts.join('.');
    // const [gameName, tags] = this.#extractTags(basename);

    const gameName = entry.Title;

    return [gameName, consoleName, []];
  }

  /**
   * @param {String} filename
   * @returns {[String, Array<String>]}
   */
  #extractTags(filename) {
    const tagsPattern = /(.*?)\s+\(([^)]+)\)$/;
    let tags = [];
    let gameName = filename;
    let matches = tagsPattern.exec(gameName);
    while (matches !== null) {
      let [_, rest, tag] = matches;
      gameName = rest;
      tags.unshift(tag);
      matches = tagsPattern.exec(gameName);
    }

    return [gameName, tags];
  }

  #handleSearch(e) {
    const words = e.target.value.split(' ');

    const articles = this.shadowRoot.querySelectorAll('li');

    articles.forEach(e => {
      e.hidden = true;
    });

    const hidden = words.reduce((acc, word) => {
      return acc.filter(article => article.dataset.path.includes(word));
    }, [...articles]);

    hidden.forEach(e => {
      e.hidden = false;
    });

    // TODO: filter on state instead of DOM.
    // Unfortunately, the following implementation uses more CPU than direct DOM manipulation.
    // Should probably try lit's `repeat`.

    // const visibleEntries = words.reduce((acc, word) => {
    //   return acc.filter(entry => entry.fs_path.toLowerCase().includes(word));
    // }, this.entries);

    // this.visibleEntries = visibleEntries;
  }
}

customElements.define('rex-collection-entries', RexCollectionEntries);
