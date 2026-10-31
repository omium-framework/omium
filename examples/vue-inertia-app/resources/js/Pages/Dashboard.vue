<script setup lang="ts">
import AppLayout from "@/Layouts/AppLayout.vue";

defineProps<{
  greeting: string;
  hint: string;
  appName: string;
  modeLabel: string;
}>();
</script>

<template>
  <AppLayout
    :app-name="appName"
    title="Welcome"
    subtitle="Server-driven Vue pages with client-side navigation."
  >
    <section class="welcome-hero">
      <div class="welcome-glow" aria-hidden="true" />
      <p class="hero-eyebrow">You're all set</p>
      <h2 class="hero-title">{{ greeting }}</h2>
      <p class="hero-lead">{{ hint }}</p>
      <div class="pill-row">
        <span class="pill pill-accent">{{ modeLabel }}</span>
        <span class="pill">Inertia</span>
        <span class="pill">Axum</span>
        <span class="pill">Vue 3</span>
      </div>
      <div class="feature-grid">
        <div class="feature-tile">
          <strong>Rust owns routes</strong>
          <span>URLs, auth, and data loading stay on the server.</span>
        </div>
        <div class="feature-tile">
          <strong>SPA feel</strong>
          <span>Inertia swaps pages without a separate client router.</span>
        </div>
        <div class="feature-tile">
          <strong>Shared design</strong>
          <span>Matches Standard and React starters out of the box.</span>
        </div>
      </div>
    </section>
  </AppLayout>
</template>
