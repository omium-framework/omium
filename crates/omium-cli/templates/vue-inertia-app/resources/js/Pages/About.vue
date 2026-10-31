<script setup lang="ts">
import AppLayout from "@/Layouts/AppLayout.vue";

defineProps<{
  appName: string;
  stack: string[];
}>();
</script>

<template>
  <AppLayout
    :app-name="appName"
    title="About"
    subtitle="How this demo relates to Laravel + Inertia."
  >
    <div class="card">
      <p class="lead">
        Routes and props come from Rust. Vue only renders what the server sends on each
        visit — navigation uses Inertia, not a separate client-side router.
      </p>
      <ul class="stack-list">
        <li v-for="item in stack" :key="item">{{ item }}</li>
      </ul>
    </div>
  </AppLayout>
</template>
