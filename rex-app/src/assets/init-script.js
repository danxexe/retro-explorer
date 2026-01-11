(()=>{
const TAB_NAME = window.name;

const isExternalDomain = window.location.host !== "rex.localhost";
const isGuidePage = window.location.pathname === '/pages/guide.html';

if (window.location.href === 'about:blank') return;
if (!TAB_NAME || TAB_NAME === '') return;
if (!(isExternalDomain || isGuidePage)) return;

let lastLoadedPosition = null;

function shouldLoadPosition() {
  window.parent.postMessage({
    type: 'rex-tab:should-load-position',
    tabName: TAB_NAME,
  }, '*');
}

function shouldSavePosition({ tabUrl, scrollY }) {
  window.parent.postMessage({
    type: 'rex-tab:should-save-position',
    tabName: TAB_NAME,
    tabUrl,
    scrollY,
  }, '*');
}

function loadPosition(data) {
  lastLoadedPosition = data;
  const { tabUrl, scrollY } = data;

  if (tabUrl !== null && window.location.href !== tabUrl) {
    window.location.href = tabUrl;
  }

  if ((window.location.href === tabUrl) && (window.scrollY !== scrollY)) {
    window.scrollTo(0, scrollY);
  };
}

function refreshPosition() {
  if (!lastLoadedPosition) return;

  if (window.location.href !== lastLoadedPosition.url || window.scrollY !== lastLoadedPosition.scrollY) {
    loadPosition(lastLoadedPosition);
  }
}

function initScrollDetection() {
  shouldLoadPosition();

  window.addEventListener('message', (e) => {
    if (e.data.type === 'scroll') {
      window.scrollBy(0, e.data.amount);
    }
  });

  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        refreshPosition();
      }
    });
  }, { threshold: 0 });

  observer.observe(document.documentElement);

  document.addEventListener('scrollend', (e) => {
    shouldSavePosition({
      tabUrl: window.location.href,
      scrollY: window.scrollY,
    });
  }, {passive: true });

  navigation.addEventListener('navigate', (e) => {
    const isNewUrl = e.destination.url !== window.location.href;

    if (!isNewUrl) return;

    shouldSavePosition({
      tabUrl: e.destination.url,
      scrollY: 0,
    });
  });

  window.addEventListener('message', (event) => {
    if (event.data.tabName !== TAB_NAME) return;

    if (event.data.type === 'rex-tab:load-position') {
      loadPosition(event.data);
    };
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initScrollDetection);
} else {
  initScrollDetection();
}

})();

export {};
