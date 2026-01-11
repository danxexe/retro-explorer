if (window.location.host !== "rex.localhost") {

  const scrollConfigKey = 'rex-tab-scroll-position:' + window.location.href;

  function loadScrollPosition() {
    const scrollY = localStorage.getItem(scrollConfigKey);

    if (scrollY !== null) {
      window.scrollTo(0, scrollY);
    }
  }

  function initScrollDetection() {
    loadScrollPosition();

    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          loadScrollPosition();
        }
      });
    }, { threshold: 0 });

    observer.observe(document.documentElement);

    document.addEventListener('scrollend', (_) => {
      localStorage.setItem(scrollConfigKey, window.scrollY);
    }, {passive: true });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initScrollDetection);
  } else {
    initScrollDetection();
  }
}

export {};
