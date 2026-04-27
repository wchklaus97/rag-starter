/**
 * RAG model field guide — loads static JSON files only.
 */
const state = {
  data: null,
  publicExamples: null,
  agentGuide: null,
  /** @type {Record<string, string>} */
  wizardPick: {},
  lang: "en",
  filterTag: "all",
  search: "",
};

const STORAGE_KEY = "rag-field-guide-lang";
const THEME_KEY = "rag-field-guide-theme";

function loadLang() {
  try {
    const s = localStorage.getItem(STORAGE_KEY);
    if (s === "en" || s === "zh" || s === "zh_hans") return s;
  } catch {
    /* ignore */
  }
  return "en";
}

function saveLang() {
  try {
    localStorage.setItem(STORAGE_KEY, state.lang);
  } catch {
    /* ignore */
  }
}

function t(key) {
  const u = state.data?.ui?.[state.lang];
  if (!u) return key;
  if (Object.prototype.hasOwnProperty.call(u, key) && u[key] !== null && typeof u[key] !== "object") {
    return String(u[key]);
  }
  const parts = key.split(".");
  let cur = u;
  for (const p of parts) {
    if (cur && typeof cur === "object" && p in cur) cur = cur[p];
    else return key;
  }
  if (typeof cur === "string" || typeof cur === "number") return String(cur);
  return key;
}

function localized(value) {
  if (value && typeof value === "object") {
    return value[state.lang] || value.zh || value.en || "";
  }
  return value || "";
}

function formatMoney(n) {
  if (n === undefined || n === null || Number.isNaN(n)) return "—";
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
    maximumFractionDigits: 4,
  }).format(n);
}

function escapeHtml(value) {
  return String(value ?? "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function modelMatches(m) {
  if (state.filterTag !== "all" && !m.tags?.includes(state.filterTag)) {
    return false;
  }
  if (!state.search.trim()) return true;
  const q = state.search.trim().toLowerCase();
  const bundle = [
    m.id,
    m.provider,
    m.strengths?.en,
    m.strengths?.zh,
    m.strengths?.zh_hans,
    m.bestFor?.en,
    m.bestFor?.zh,
    m.bestFor?.zh_hans,
    m.longNote?.en,
    m.longNote?.zh,
    m.longNote?.zh_hans,
    ...(m.tags || []),
  ]
    .filter(Boolean)
    .join(" ")
    .toLowerCase();
  return bundle.includes(q);
}

function allTags(models) {
  const set = new Set();
  for (const m of models) (m.tags || []).forEach((t) => set.add(t));
  return [...set].sort();
}

function applyI18nToDom() {
  const u = state.data?.ui?.[state.lang] || {};
  document.documentElement.lang = state.lang === "zh" ? "zh-Hant" : state.lang === "zh_hans" ? "zh-Hans" : "en";
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    const key = el.getAttribute("data-i18n");
    if (!key || u[key] === undefined) return;
    el.textContent = u[key];
  });
  const sp = document.getElementById("search-input");
  const phKey = sp?.getAttribute("data-i18n-placeholder");
  if (sp && phKey && u[phKey] !== undefined) {
    sp.setAttribute("placeholder", u[phKey]);
  }
  if (sp && u.searchLabel) {
    sp.setAttribute("aria-label", u.searchLabel);
  }
  const sl = document.querySelector("label[for='search-input']");
  if (sl && u.searchLabel) sl.textContent = u.searchLabel;
  syncThemeButton();
}

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
  const hasUi = state.data?.ui?.[state.lang];
  btn.setAttribute(
    "aria-label",
    hasUi ? (dark ? t("themeToLight") : t("themeToDark")) : dark ? "Light theme" : "Dark theme",
  );
  btn.setAttribute("aria-pressed", dark ? "true" : "false");
}

function wireTheme() {
  const btn = document.getElementById("theme-toggle");
  if (!btn) return;
  btn.addEventListener("click", () => {
    setTheme(currentTheme() === "dark" ? "light" : "dark");
  });
}

function renderRecommendations() {
  const list = state.data.recommendations?.[state.lang] || state.data.recommendations?.en || [];
  const ol = document.getElementById("recommendation-list");
  const fb = document.getElementById("recommendation-fallback");
  if (!ol) return;
  if (!list.length) {
    ol.hidden = true;
    if (fb) {
      fb.hidden = false;
      fb.textContent = "—";
    }
    return;
  }
  ol.hidden = false;
  if (fb) fb.hidden = true;
  ol.innerHTML = list
    .map((row) => {
      const a = (row.if || "").replace(/</g, "&lt;");
      const b = (row.then || "").replace(/</g, "&lt;");
      return `<li><strong>${a}</strong> → <code>${b}</code></li>`;
    })
    .join("");
}

function renderTesting() {
  const steps = state.data.testingPlan?.[state.lang] || state.data.testingPlan?.en || [];
  const ol = document.getElementById("testing-list");
  if (!ol) return;
  ol.innerHTML = steps.map((s) => `<li>${s.replace(/</g, "&lt;")}</li>`).join("");
}

function renderUpdated() {
  const el = document.getElementById("data-updated");
  if (!el) return;
  const d = state.data.updated;
  if (d) {
  const prefix = state.lang === "zh" ? "資料日期：" : state.lang === "zh_hans" ? "数据日期：" : "Data snapshot: ";
    el.textContent = prefix + d;
  } else {
    el.textContent = "";
  }
}

function renderChips() {
  const host = document.getElementById("filter-chips");
  if (!host) return;
  const tags = allTags(state.data.models || []);
  const tagLabels = state.data.ui?.[state.lang]?.tagLabels || state.data.ui?.en?.tagLabels || {};

  const parts = [
    `<button type="button" class="chip" data-tag="all" aria-pressed="${state.filterTag === "all"}">${t("all")}</button>`,
    ...tags.map(
      (tag) =>
        `<button type="button" class="chip" data-tag="${tag}" aria-pressed="${state.filterTag === tag}">${tagLabels[tag] || tag}</button>`,
    ),
  ];
  host.innerHTML = parts.join("");

  host.querySelectorAll(".chip").forEach((btn) => {
    btn.addEventListener("click", () => {
      state.filterTag = btn.getAttribute("data-tag") || "all";
      host.querySelectorAll(".chip").forEach((b) => {
        b.setAttribute("aria-pressed", b.getAttribute("data-tag") === state.filterTag);
      });
      renderCards();
    });
  });
}

function renderCards() {
  const grid = document.getElementById("model-grid");
  const empty = document.getElementById("empty-state");
  if (!grid) return;
  const models = state.data.models || [];
  const visible = models.filter(modelMatches);
  if (empty) {
    empty.hidden = visible.length > 0;
    empty.textContent = t("empty");
  }
  grid.querySelectorAll(".model-card").forEach((c) => c.remove());
  if (!visible.length) {
    return;
  }

  const tagLabels = state.data.ui?.[state.lang]?.tagLabels || state.data.ui?.en?.tagLabels || {};
  const slink = t("sourceLink");
  const html = visible
    .map((m) => {
      const st = localized(m.strengths);
      const bf = localized(m.bestFor);
      const ln = localized(m.longNote);
      const tags = (m.tags || [])
        .map(
          (tag) =>
            `<li title="${escapeHtml(tagLabels[tag] || tag)}">${escapeHtml(tagLabels[tag] || tag)}</li>`,
        )
        .join("");

      const outPrice = m.outputPricePerMillionTokens ?? 0;
      const sourceLinks = (m.sourceUrls || [])
        .map(
          (u) =>
            `<li><a href="${escapeHtml(u)}" rel="noopener noreferrer">${escapeHtml(slink)}</a></li>`,
        )
        .join("");

      return `
        <article class="model-card">
          <header>
            <h3 class="model-id">${escapeHtml(m.id)}</h3>
            <p class="provider">${escapeHtml(m.provider || "")}</p>
          </header>
          <div class="price-row">
            <span><span class="label-muted">${t("priceInput")}</span> ${formatMoney(m.pricePerMillionInputTokens)}</span>
            <span><span class="label-muted">${t("priceOutput")}</span> ${formatMoney(outPrice)}</span>
          </div>
          <ul class="card-tags" aria-label="Tags">${tags}</ul>
          <p class="strengths"><span class="label-muted">${escapeHtml(t("labelStrengths"))}</span> ${escapeHtml(st)}</p>
          <p class="bestfor"><span class="label-muted">${escapeHtml(t("labelBestFor"))}</span> ${escapeHtml(bf)}</p>
          <p class="card-long">${escapeHtml(ln)}</p>
          ${
            sourceLinks
              ? `<ul class="source-mini" aria-label="Sources">${sourceLinks}</ul>`
              : ""
          }
        </article>`;
    })
    .join("");

  if (empty) {
    empty.insertAdjacentHTML("afterend", html);
  } else {
    grid.insertAdjacentHTML("beforeend", html);
  }
}

function agT(key) {
  const u = state.agentGuide?.ui?.[state.lang] || state.agentGuide?.ui?.en;
  if (u && Object.prototype.hasOwnProperty.call(u, key)) return String(u[key]);
  return key;
}

function wizardSteps() {
  return state.agentGuide?.wizard?.steps?.[state.lang] || state.agentGuide?.wizard?.steps?.en || [];
}

function computeWizardScores() {
  const scores = /** @type {Record<string, number>} */ ({});
  const steps = wizardSteps();
  for (const st of steps) {
    const optId = state.wizardPick[st.id] || st.options[0]?.id;
    const opt = st.options.find((o) => o.id === optId) || st.options[0];
    if (!opt?.weight) continue;
    for (const [k, w] of Object.entries(opt.weight)) {
      if (w > 0) scores[k] = (scores[k] || 0) + w;
    }
  }
  return Object.entries(scores)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 4);
}

function updateWizardResultBox() {
  const box = document.getElementById("wizard-resultlist");
  if (!box) return;
  const top = computeWizardScores();
  if (!top.length) {
    box.innerHTML = `<p class="muted">—</p>`;
    return;
  }
  const rows = state.agentGuide?.frameworkRows?.[state.lang] || state.agentGuide?.frameworkRows?.en || [];
  box.innerHTML = top
    .map(([name, sc]) => {
      const row = rows.find((r) => r.name === name);
      const note = row ? ` — <span class="muted">${escapeHtml(row.for)}</span>` : "";
      return `<li><strong>${escapeHtml(name)}</strong> <span class="score-badge">${sc}</span>${note}</li>`;
    })
    .join("");
}

function stepProgressPrefix(i, n) {
  if (state.lang === "en") return `Step ${i + 1}/${n}: `;
  return `第 ${i + 1}/${n} 步：`;
}

function renderAgentFieldGuide() {
  const body = document.getElementById("agent-guide-body");
  const wrap = document.getElementById("agent-guide");
  if (!state.agentGuide || !body) return;
  if (wrap) wrap.hidden = false;
  const nav = document.getElementById("inpage-nav");
  if (nav) nav.hidden = false;

  const eb = document.getElementById("agent-guide-eyebrow");
  const ti = document.getElementById("agent-guide-title");
  const le = document.getElementById("agent-guide-lede");
  if (eb) eb.textContent = agT("sectionEyebrow");
  if (ti) ti.textContent = agT("sectionTitle");
  if (le) le.textContent = agT("sectionLede");

  const foundations = state.agentGuide.foundations?.[state.lang] || state.agentGuide.foundations?.en || [];
  const paradigms = state.agentGuide.paradigms?.[state.lang] || state.agentGuide.paradigms?.en || [];
  const rows = state.agentGuide.frameworkRows?.[state.lang] || state.agentGuide.frameworkRows?.en || [];
  const trends = state.agentGuide.trends?.[state.lang] || state.agentGuide.trends?.en || [];
  const advice = state.agentGuide.advice?.[state.lang] || state.agentGuide.advice?.en || [];
  const steps = wizardSteps();

  for (const st of steps) {
    if (!state.wizardPick[st.id]) {
      state.wizardPick[st.id] = st.options[0]?.id || "";
    }
  }

  const parts = [];
  parts.push(`<section class="panel agent-foundations" aria-labelledby="f3"><h2 id="f3" class="h-like">${escapeHtml(agT("foundationsTitle"))}</h2><div class="foundation-grid">`);
  for (const f of foundations) {
    parts.push(
      `<article class="foundation-card"><h3 class="f3h">${escapeHtml(f.name)}</h3><p>${escapeHtml(f.body)}</p></article>`,
    );
  }
  parts.push(`</div></section>`);

  parts.push(`<section class="panel agent-paradigms" aria-labelledby="p5"><h2 id="p5" class="h-like">${escapeHtml(agT("paradigmsTitle"))}</h2><div class="paradigm-grid">`);
  for (const p of paradigms) {
    parts.push(
      `<article class="paradigm-card"><h3 class="f3h">${escapeHtml(p.title)}</h3><p class="exemplar">${escapeHtml(
        p.exemplar,
      )}</p><p><span class="sign">+</span> ${escapeHtml(p.strengths)}</p><p class="caveat"><span class="sign">−</span> ${escapeHtml(
        p.caveats,
      )}</p></article>`,
    );
  }
  parts.push(`</div></section>`);

  const th =
    state.lang === "en"
      ? ["Framework", "Pattern", "Best for", "Note"]
      : state.lang === "zh_hans"
        ? ["框架", "形态", "适用", "备注"]
        : ["框架", "型態", "適用", "備註"];
  parts.push(
    `<section class="panel" aria-labelledby="ftable"><h2 id="ftable" class="h-like">${escapeHtml(agT("tableTitle"))}</h2><div class="table-scroll"><table class="agent-table"><thead><tr><th>${th[0]}</th><th>${th[1]}</th><th>${th[2]}</th><th>${th[3]}</th></tr></thead><tbody>`,
  );
  for (const r of rows) {
    parts.push(
      `<tr><td>${escapeHtml(r.name)}</td><td>${escapeHtml(r.pattern)}</td><td>${escapeHtml(r.for)}</td><td>${escapeHtml(
        r.note,
      )}</td></tr>`,
    );
  }
  parts.push(`</tbody></table></div></section>`);

  parts.push(
    `<section class="panel agent-wizard" aria-labelledby="wiz-h"><h2 id="wiz-h" class="h-like">${escapeHtml(agT("wizardTitle"))}</h2><p class="muted">${escapeHtml(agT("wizardResultHint"))}</p><form class="wizard-form" id="wizard-form">`,
  );
  for (let i = 0; i < steps.length; i += 1) {
    const st = steps[i];
    const legend = stepProgressPrefix(i, steps.length) + st.title;
    const groupName = `wizard-${st.id}`;
    const opts = (st.options || [])
      .map((o) => {
        const cur = state.wizardPick[st.id] || st.options[0].id;
        const checked = cur === o.id;
        return `<label class="radio-label"><input type="radio" name="${groupName}" value="${escapeHtml(o.id)}" ${checked ? "checked" : ""} /> <span>${escapeHtml(o.label)}</span></label>`;
      })
      .join("");
    parts.push(`<fieldset class="wizard-step" data-wizard-step="${escapeHtml(st.id)}"><legend>${escapeHtml(legend)}</legend>${opts}</fieldset>`);
  }
  parts.push(
    `</form><p><button type="button" class="btn-reset" id="wizard-reset">${escapeHtml(agT("wizardReset"))}</button></p><h3 class="f3h">${escapeHtml(agT("wizardResultTitle"))}</h3><ol class="wizard-result" id="wizard-resultlist"></ol></section>`,
  );

  parts.push(`<section class="panel" aria-labelledby="tr"><h2 id="tr" class="h-like">${escapeHtml(agT("trendsTitle"))}</h2><ul class="trend-list">`);
  for (const line of trends) {
    parts.push(`<li>${escapeHtml(line)}</li>`);
  }
  parts.push(`</ul></section>`);

  parts.push(`<section class="panel" aria-labelledby="ad"><h2 id="ad" class="h-like">${escapeHtml(agT("adviceTitle"))}</h2><div class="advice-grid">`);
  for (const a of advice) {
    parts.push(
      `<article class="advice-card"><h3 class="f3h">${escapeHtml(a.audience)}</h3><p>${escapeHtml(a.text)}</p></article>`,
    );
  }
  parts.push(`</div></section>`);

  body.innerHTML = parts.join("");

  body.querySelectorAll('.wizard-form input[type="radio"]').forEach((inp) => {
    inp.addEventListener("change", () => {
      const el = /** @type {HTMLInputElement} */ (inp);
      const m = el.name?.match(/^wizard-(.+)$/);
      if (m) state.wizardPick[m[1]] = el.value;
      updateWizardResultBox();
    });
  });

  const reset = document.getElementById("wizard-reset");
  if (reset) {
    reset.addEventListener("click", () => {
      for (const st of steps) {
        state.wizardPick[st.id] = st.options[0]?.id || "";
      }
      const form = document.getElementById("wizard-form");
      if (form) {
        for (const st of steps) {
          const first = form.querySelector(`input[name="wizard-${st.id}"]`);
          if (first) {
            form.querySelectorAll(`input[name="wizard-${st.id}"]`).forEach((r) => {
              /** @type {HTMLInputElement} */ (r).checked = false;
            });
            /** @type {HTMLInputElement} */ (first).checked = true;
          }
        }
      }
      updateWizardResultBox();
    });
  }

  updateWizardResultBox();
}

function renderPublicExamples() {
  const host = document.getElementById("public-examples-grid");
  if (!host) return;
  const examples = state.publicExamples?.examples || [];
  const note = document.getElementById("public-examples-note");
  if (note && state.publicExamples?.note) {
    note.textContent = localized(state.publicExamples.note);
  }
  if (!examples.length) {
    host.innerHTML = `<p class="muted">No public example sources found.</p>`;
    return;
  }
  host.innerHTML = examples
    .map((item) => {
      const models = (item.recommendedModels || [])
        .map((model) => `<li>${escapeHtml(model)}</li>`)
        .join("");
      return `
        <article class="example-card">
          <h3>${escapeHtml(localized(item.title))}</h3>
          <p class="muted">${escapeHtml(localized(item.category))}</p>
          <p>${escapeHtml(localized(item.goodFor))}</p>
          <ul class="model-pill-list" aria-label="Recommended embedding models">${models}</ul>
          <p>${escapeHtml(localized(item.why))}</p>
          <p class="muted">${escapeHtml(localized(item.licenseNote))}</p>
          <a href="${escapeHtml(item.sourceUrl)}" rel="noopener noreferrer">Source</a>
        </article>`;
    })
    .join("");
}

function wireLang() {
  document.querySelectorAll(".lang-btn").forEach((b) => {
    b.addEventListener("click", () => {
      const lang = b.getAttribute("data-lang");
      if (lang !== "en" && lang !== "zh" && lang !== "zh_hans") return;
      state.lang = lang;
      saveLang();
      document.querySelectorAll(".lang-btn").forEach((x) => {
        const isOn = x.getAttribute("data-lang") === state.lang;
        x.classList.toggle("is-active", isOn);
        x.setAttribute("aria-pressed", isOn);
      });
      applyI18nToDom();
      renderChips();
      renderRecommendations();
      renderTesting();
      renderUpdated();
      renderCards();
      renderPublicExamples();
      renderAgentFieldGuide();
    });
  });
  state.lang = loadLang();
  document.querySelectorAll(".lang-btn").forEach((x) => {
    const isOn = x.getAttribute("data-lang") === state.lang;
    x.classList.toggle("is-active", isOn);
    x.setAttribute("aria-pressed", isOn);
  });
}

function wireSearch() {
  const sp = document.getElementById("search-input");
  if (!sp) return;
  sp.addEventListener("input", () => {
    state.search = sp.value;
    renderCards();
  });
}

async function init() {
  const [res, examplesRes, agentRes] = await Promise.all([
    fetch("data/models.json", { cache: "no-store" }),
    fetch("data/public_examples.json", { cache: "no-store" }).catch(() => null),
    fetch("data/agent_framework_guide.json", { cache: "no-store" }).catch(() => null),
  ]);
  if (!res.ok) throw new Error("Failed to load data/models.json");
  state.data = await res.json();
  if (examplesRes?.ok) {
    state.publicExamples = await examplesRes.json();
  }
  if (agentRes?.ok) {
    state.agentGuide = await agentRes.json();
  }
  wireTheme();
  wireLang();
  applyI18nToDom();
  renderRecommendations();
  renderTesting();
  renderUpdated();
  renderChips();
  renderCards();
  renderPublicExamples();
  renderAgentFieldGuide();
  wireSearch();
}

init().catch((err) => {
  const main = document.getElementById("main");
  if (main) {
    const p = document.createElement("p");
    p.className = "panel muted";
    p.setAttribute("role", "alert");
    p.textContent = `Could not load data: ${err.message}`;
    main.prepend(p);
  }
});
