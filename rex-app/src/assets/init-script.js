if (window.location.host !== "rex.localhost") {

  const scrollConfigKey = 'rex-tab-scroll-position:' + window.location.href;

  function loadScrollPosition() {
    const scrollY = localStorage.getItem(scrollConfigKey);

    if (scrollY !== null) {
      window.scrollTo(0, scrollY);
    }
  }

  function saveScrollPosition() {
    localStorage.setItem(scrollConfigKey, window.scrollY);
  }

  function initScrollDetection() {
    loadScrollPosition();

    window.addEventListener('message', (e) => {
      if (e.data.type === 'scroll') {
        window.scrollBy(0, e.data.amount);
      }
    });

    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          loadScrollPosition();
        }
      });
    }, { threshold: 0 });

    observer.observe(document.documentElement);

    document.addEventListener('scrollend', (e) => {
      saveScrollPosition();
    }, {passive: true });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initScrollDetection);
  } else {
    initScrollDetection();
  }
}

export {};
