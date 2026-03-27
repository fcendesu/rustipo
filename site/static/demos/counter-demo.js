for (const mount of document.querySelectorAll('[data-rustipo-demo="counter-demo"]')) {
  if (mount.dataset.rustipoMounted === "true") {
    continue;
  }

  mount.dataset.rustipoMounted = "true";
  mount.innerHTML = `
    <div class="rustipo-demo-card">
      <h3>Counter demo</h3>
      <p>This demo is mounted by a page-scoped asset declared directly in Markdown.</p>
      <div class="rustipo-demo-controls">
        <button type="button" data-action="increment">Increment</button>
        <button type="button" data-action="reset" class="secondary">Reset</button>
        <span class="rustipo-demo-value" data-value>0</span>
      </div>
    </div>
  `;

  let value = 0;
  const valueNode = mount.querySelector("[data-value]");

  mount.addEventListener("click", (event) => {
    const button = event.target.closest("button");
    if (!button || !valueNode) {
      return;
    }

    if (button.dataset.action === "increment") {
      value += 1;
    } else if (button.dataset.action === "reset") {
      value = 0;
    }

    valueNode.textContent = String(value);
  });
}
