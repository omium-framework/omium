<script setup lang="ts">
defineProps<{
  label: string;
  modelValue: string;
  type?: string;
  autocomplete?: string;
  required?: boolean;
}>();

defineEmits<{
  "update:modelValue": [value: string];
}>();
</script>

<template>
  <label class="field">
    <span>{{ label }}</span>
    <input
      :type="type ?? 'text'"
      :value="modelValue"
      :autocomplete="autocomplete"
      :required="required"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
  </label>
</template>
