<script setup lang="ts">
import { computed, watch } from "vue";
import { Link, usePage, router } from "@inertiajs/vue3";

defineProps<{
  appName: string;
  title: string;
  subtitle?: string;
}>();

const page = usePage();
const authenticated = computed(() => Boolean(page.props.authenticated));
const nav = computed(() => {
  const items =
    (page.props.nav as { href: string; label: string; active: boolean }[]) ?? [];
  if (!authenticated.value) {
    return items;
  }
  return items.filter((item) => item.href !== "/login" && item.href !== "/register");
});
const flash = computed(
  () => (page.props.flash as { success?: string; error?: string }) ?? {},
);
const csrfToken = computed(() => page.props.csrfToken as string | null);

watch(
  csrfToken,
  (token) => {
    if (typeof token !== "string" || token.length === 0) {
      return;
    }
    let meta = document.querySelector('meta[name="csrf-token"]');
    if (!meta) {
      meta = document.createElement("meta");
      meta.setAttribute("name", "csrf-token");
      document.head.appendChild(meta);
    }
    meta.setAttribute("content", token);
  },
  { immediate: true },
);
</script>

<template>
  <div class="app-shell">
    <header class="app-header">
      <div class="app-header-inner">
        <div>
          <p class="brand">{{ appName }}</p>
          <h1 class="page-title">{{ title }}</h1>
          <p v-if="subtitle" class="page-subtitle">{{ subtitle }}</p>
        </div>
        <nav class="app-nav" aria-label="Main">
          <Link
            v-for="item in nav"
            :key="item.href"
            :href="item.href"
            class="nav-link"
            :class="{ 'nav-link-active': item.active }"
          >
            {{ item.label }}
          </Link>
          <button
            v-if="authenticated"
            type="button"
            class="nav-link"
            @click="router.post('/logout')"
          >
            Sign out
          </button>
        </nav>
      </div>
    </header>
    <main class="app-main">
      <p v-if="flash.success" class="status-banner">{{ flash.success }}</p>
      <p v-if="flash.error" class="status-banner status-banner-muted">{{ flash.error }}</p>
      <slot />
    </main>
  </div>
</template>
