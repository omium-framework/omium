import "../css/app.css";
import { createInertiaApp } from "@inertiajs/react";
import { createRoot } from "react-dom/client";
import type { ComponentType } from "react";

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
