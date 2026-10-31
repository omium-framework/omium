<script setup lang="ts">
import { computed } from "vue";
import { Link, usePage } from "@inertiajs/vue3";

defineProps<{
  appName: string;
  title: string;
  subtitle?: string;
}>();

const page = usePage();
const nav = computed(() => (page.props.nav as { href: string; label: string; active: boolean }[]) ?? []);
const flash = computed(
  () => (page.props.flash as { success?: string; error?: string }) ?? {},
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
