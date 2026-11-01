import "../css/app.css";
import { createApp, h, type DefineComponent } from "vue";
import { createInertiaApp, router } from "@inertiajs/vue3";

function readCsrfToken(): string | undefined {
  const meta = document.querySelector('meta[name="csrf-token"]');
  const content = meta?.getAttribute("content");
  return content && content.length > 0 ? content : undefined;
}

let lastVisitHref: string | null = null;
let devNetworkRetry = false;

router.on("before", (event) => {
  const token = readCsrfToken();
  if (token) {
    event.detail.visit.headers["X-XSRF-TOKEN"] = token;
  }
  lastVisitHref = event.detail.visit.url.href;
});

router.on("success", () => {
  devNetworkRetry = false;
});

if (import.meta.env.DEV) {
  document.addEventListener("inertia:exception", (raw) => {
    const event = raw as CustomEvent<{ exception?: { message?: string; code?: string } }>;
    const exception = event.detail?.exception;
    const isNetwork =
      exception?.message === "Network Error" || exception?.code === "ERR_NETWORK";
    if (!isNetwork || devNetworkRetry || !lastVisitHref) {
      return;
    }
    devNetworkRetry = true;
    event.preventDefault();
    window.setTimeout(() => {
      router.visit(lastVisitHref!, { preserveScroll: true, preserveState: true });
    }, 900);
  });
}

createInertiaApp({
  defaults: {
    future: {
      useScriptElementForInitialPage: true,
    },
  },
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
