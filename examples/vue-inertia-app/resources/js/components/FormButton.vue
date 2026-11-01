<script setup lang="ts">
defineProps<{
  type?: "button" | "submit";
  variant?: "primary" | "ghost";
  disabled?: boolean;
  block?: boolean;
}>();
</script>

<template>
  <button
    :type="type ?? 'button'"
    class="btn btn-primary"
    :class="{ 'btn-primary-block': block === true }"
    :disabled="disabled"
  >
    <slot />
  </button>
</template>
