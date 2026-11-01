<script setup lang="ts">
defineProps<{
  title: string;
  lead?: string;
}>();
</script>

<template>
  <section class="card card-narrow auth-card stack-form">
    <h2 class="auth-card-title">{{ title }}</h2>
    <p v-if="lead" class="auth-card-lead">{{ lead }}</p>
    <div class="stack-form" style="margin-top: 1.25rem">
      <slot />
    </div>
  </section>
</template>
