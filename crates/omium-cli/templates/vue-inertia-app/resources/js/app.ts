import "../css/app.css";
import { createApp, h, type DefineComponent } from "vue";
import { createInertiaApp } from "@inertiajs/vue3";

createInertiaApp({
  resolve: (name) => {
    const pages = import.meta.glob<DefineComponent>("./Pages/**/*.vue");
    const path = `./Pages/${name}.vue`;
    const loader = pages[path];
    if (!loader) {
      throw new Error(`Unknown Inertia page: ${name}`);
    }
    return loader();
  },
  setup({ el, App, props, plugin }) {
    createApp({ render: () => h(App, props) })
      .use(plugin)
      .mount(el);
  },
});
