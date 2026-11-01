import "../css/app.css";
import { createInertiaApp, router } from "@inertiajs/react";
import { createRoot } from "react-dom/client";
import type { ComponentType } from "react";

function readCsrfToken(): string | undefined {
  const meta = document.querySelector('meta[name="csrf-token"]');
  const content = meta?.getAttribute("content");
  return content && content.length > 0 ? content : undefined;
}

router.on("before", (event) => {
  const token = readCsrfToken();
  if (token) {
    event.detail.visit.headers["X-XSRF-TOKEN"] = token;
  }
});

createInertiaApp({
  resolve: async (name) => {
    const pages = import.meta.glob<{ default: ComponentType }>("./Pages/**/*.tsx");
    const path = `./Pages/${name}.tsx`;
    const loader = pages[path];
    if (!loader) {
      throw new Error(`Unknown Inertia page: ${name}`);
    }
    const module = await loader();
    return module.default;
  },
  setup({ el, App, props }) {
    createRoot(el).render(<App {...props} />);
  },
});
