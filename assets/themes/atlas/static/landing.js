(() => {
  const body = document.body;
  if (!body || !body.classList.contains("landing-homepage")) {
    return;
  }

  const hero = document.querySelector(".hero-stage");
  const heroVisual = document.querySelector(".hero-visual");
  const canvases = Array.from(document.querySelectorAll(".site-canvas"));
  const orbits = Array.from(document.querySelectorAll(".visual-orbit"));
  const glows = Array.from(document.querySelectorAll(".hero-glow"));
  const processStops = Array.from(document.querySelectorAll(".process-stop"));
  const copyButtons = Array.from(document.querySelectorAll("[data-copy-text]"));
  const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  window.requestAnimationFrame(() => {
    body.classList.add("landing-ready");
  });

  const setActiveCanvas = (canvas) => {
    if (!heroVisual) {
      return;
    }

    heroVisual.classList.add("is-carousel");
    heroVisual.classList.remove("has-active-docs", "has-active-journal", "has-active-personal");
    canvases.forEach((item) => item.classList.remove("is-active"));

    if (!canvas) {
      heroVisual.classList.remove("is-carousel");
      return;
    }

    const key = canvas.getAttribute("data-canvas-key");
    canvas.classList.add("is-active");

    if (key) {
      heroVisual.classList.add(`has-active-${key}`);
    }
  };

  const copyText = async (text) => {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      await navigator.clipboard.writeText(text);
      return;
    }

    const temp = document.createElement("textarea");
    temp.value = text;
    temp.setAttribute("readonly", "");
    temp.style.position = "absolute";
    temp.style.left = "-9999px";
    document.body.appendChild(temp);
    temp.select();
    document.execCommand("copy");
    temp.remove();
  };

  copyButtons.forEach((button) => {
    const feedback = button.querySelector(".hero-copy-feedback");

    button.addEventListener("click", async () => {
      const text = button.getAttribute("data-copy-text");
      if (!text) {
        return;
      }

      try {
        await copyText(text);
        button.classList.add("is-copied");

        window.setTimeout(() => {
          button.classList.remove("is-copied");
        }, 1400);
      } catch (_) {
        if (feedback) {
          feedback.textContent = "Failed";
          button.classList.add("is-copied");
          window.setTimeout(() => {
            button.classList.remove("is-copied");
            feedback.textContent = "Copied";
          }, 1400);
        }
      }
    });
  });

  if (processStops.length > 0 && !prefersReducedMotion) {
    let activeIndex = 0;
    processStops[0].classList.add("is-emphasized");
    window.setInterval(() => {
      processStops[activeIndex].classList.remove("is-emphasized");
      activeIndex = (activeIndex + 1) % processStops.length;
      processStops[activeIndex].classList.add("is-emphasized");
    }, 1800);
  }

  if (!hero || prefersReducedMotion) {
    return;
  }

  const reset = () => {
    canvases.forEach((canvas) => {
      canvas.style.setProperty("--canvas-shift-x", "0px");
      canvas.style.setProperty("--canvas-shift-y", "0px");
    });
    orbits.forEach((orbit) => {
      orbit.style.transform = "";
    });
    glows.forEach((glow) => {
      glow.style.transform = "";
    });
    setActiveCanvas(null);
  };

  hero.addEventListener("mousemove", (event) => {
    const rect = hero.getBoundingClientRect();
    const x = (event.clientX - rect.left) / rect.width - 0.5;
    const y = (event.clientY - rect.top) / rect.height - 0.5;

    canvases.forEach((canvas) => {
      const depth = Number(canvas.getAttribute("data-canvas-depth") || "1");
      canvas.style.setProperty("--canvas-shift-x", `${x * depth * 18}px`);
      canvas.style.setProperty("--canvas-shift-y", `${y * depth * 14}px`);
    });

    orbits.forEach((orbit, index) => {
      const multiplier = index === 0 ? 10 : -8;
      orbit.style.transform = `translate(${x * multiplier}px, ${y * multiplier}px) rotate(${index === 0 ? -11 : 14}deg)`;
    });

    glows.forEach((glow, index) => {
      const multiplier = index === 0 ? 16 : -12;
      glow.style.transform = `translate(${x * multiplier}px, ${y * multiplier}px)`;
    });
  });

  hero.addEventListener("mouseleave", reset);

  canvases.forEach((canvas) => {
    canvas.addEventListener("mouseenter", () => setActiveCanvas(canvas));
    canvas.addEventListener("focus", () => setActiveCanvas(canvas));
  });

  heroVisual?.addEventListener("mouseleave", () => setActiveCanvas(null));
})();
