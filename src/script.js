const { invoke } = window.__TAURI__.core;
const main = document.getElementById("main");
const tabs = document.querySelectorAll("nav button");
let current = "random";

tabs.forEach(
  (b) =>
    (b.onclick = () => {
      current = b.dataset.tab;
      tabs.forEach((x) => x.classList.remove("active"));
      b.classList.add("active");
      render();
    }),
);

function toast(msg, type = "") {
  const el = document.createElement("div");
  el.className = `toast ${type}`;
  el.textContent = msg;
  document.getElementById("toasts").appendChild(el);
  setTimeout(() => {
    el.classList.add("out");
    setTimeout(() => el.remove(), 250);
  }, 2200);
}

function confirmModal(text, onConfirm) {
  document.getElementById("modalText").textContent = text;
  document.getElementById("modalOverlay").classList.add("show");
  const btn = document.getElementById("modalConfirmBtn");
  const newBtn = btn.cloneNode(true);
  btn.replaceWith(newBtn);
  newBtn.onclick = () => {
    closeModal();
    onConfirm();
  };
}
function closeModal() {
  document.getElementById("modalOverlay").classList.remove("show");
}

function escapeHtml(s) {
  return s.replace(
    /[&<>"']/g,
    (c) =>
      ({
        "&": "&amp;",
        "<": "&lt;",
        ">": "&gt;",
        '"': "&quot;",
        "'": "&#39;",
      })[c],
  );
}

function quoteCard(q) {
  return `<div class="card">
    <div class="rating">ELO ${Math.round(q.rating)} · ♥ ${q.likes}</div>
    <div class="text">${escapeHtml(q.text)}</div>
    ${q.source ? `<div class="src">— ${escapeHtml(q.source)}</div>` : ""}
    <div class="actions">
      <button onclick="doLike(${q.id})">♥ Like</button>
      <button class="del" onclick="askDelete(${q.id})">✕ Borrar</button>
    </div>
  </div>`;
}

async function render() {
  if (current === "random") return renderRandom();
  if (current === "duel") return renderDuel();
  if (current === "top") return renderTop();
  if (current === "add") return renderAdd();
}

async function renderRandom() {
  const q = await invoke("get_random");
  main.innerHTML = q
    ? quoteCard(q) +
      `<button class="addbtn secondary" onclick="renderRandom()">Other</button>`
    : `<div class="empty">No hay frases todavía.<br/>Agrega una en "Agregar".</div>`;
}

async function renderDuel() {
  const pair = await invoke("get_random_pair");
  if (pair.length < 2) {
    main.innerHTML = `<div class="empty">Necesitas al menos 2 frases<br/>para hacer duelos.</div>`;
    return;
  }
  main.innerHTML = `<div class="pair">
    <div class="pick-card" onclick="pick(${pair[0].id},${pair[1].id})">${escapeHtml(pair[0].text)}</div>
    <div class="vs">VS</div>
    <div class="pick-card" onclick="pick(${pair[1].id},${pair[0].id})">${escapeHtml(pair[1].text)}</div>
  </div>`;
}

async function pick(winner, loser) {
  await invoke("vote_pair", { winnerId: winner, loserId: loser });
  toast("Elo actualizado", "ok");
  renderDuel();
}

async function renderTop() {
  const all = await invoke("get_all");
  main.innerHTML = all.length
    ? all.map((q) => quoteCard(q)).join("") +
      `<button class="addbtn secondary" onclick="doExport()">⬇ Backup (.json)</button>`
    : `<div class="empty">Sin frases aún.</div>`;
}

function renderAdd() {
  main.innerHTML = `
    <textarea id="text" rows="4" placeholder="Escribe o pega la frase..."></textarea>
    <input id="source" placeholder="Fuente (libro, autor)... opcional" />
    <button class="addbtn" onclick="submitQuote()">+ Guardar</button>
  `;
}

async function submitQuote() {
  const textEl = document.getElementById("text");
  const text = textEl.value.trim();
  const source = document.getElementById("source").value.trim() || null;
  if (!text) {
    toast("Escribe algo primero", "err");
    return;
  }
  try {
    await invoke("add_quote", { text, source });
    toast("Frase guardada", "ok");
    current = "top";
    tabs.forEach((x) => x.classList.remove("active"));
    document.querySelector('[data-tab="top"]').classList.add("active");
    renderTop();
  } catch (e) {
    if (e === "DUPLICATE") toast("Esa frase ya existe", "err");
    else toast("Error al guardar", "err");
  }
}

async function doLike(id) {
  await invoke("like_quote", { id });
  toast("♥ Like");
  render();
}

function askDelete(id) {
  confirmModal(
    "¿Seguro que quieres borrar esta frase? No se puede deshacer.",
    async () => {
      await invoke("delete_quote", { id });
      toast("Frase borrada", "err");
      render();
    },
  );
}

async function doExport() {
  try {
    await invoke("export_backup");
    toast("Backup guardado ✓", "ok");
  } catch (e) {
    toast("Error al hacer backup", "err");
  }
}

render();