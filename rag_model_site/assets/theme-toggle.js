/**
 * Standalone light/dark toggle for pages that don't load app.js.
 * Shares localStorage key with the field guide so theme persists.
 */
(function () {
  const THEME_KEY = "rag-field-guide-theme";

  function currentTheme() {
    return document.documentElement.getAttribute("data-theme") === "dark" ? "dark" : "light";
  }

  function setTheme(next) {
    if (next !== "light" && next !== "dark") return;
    document.documentElement.setAttribute("data-theme", next);
    try {
      localStorage.setItem(THEME_KEY, next);
    } catch {
      /* ignore */
    }
    const meta = document.getElementById("meta-theme-color");
    if (meta) {
      meta.setAttribute("content", next === "dark" ? "#1b1628" : "#f7f4ef");
    }
    syncThemeButton();
  }

  function syncThemeButton() {
    const btn = document.getElementById("theme-toggle");
    const icon = btn?.querySelector(".theme-btn__icon");
    if (!btn) return;
    const dark = currentTheme() === "dark";
    if (icon) icon.textContent = dark ? "☼" : "☾";
    btn.setAttribute("aria-label", dark ? "Light theme" : "Dark theme");
    btn.setAttribute("aria-pressed", dark ? "true" : "false");
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }

  function init() {
    syncThemeButton();
    document.getElementById("theme-toggle")?.addEventListener("click", () => {
      setTheme(currentTheme() === "dark" ? "light" : "dark");
    });
  }
})();
