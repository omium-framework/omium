<script setup lang="ts">
import AppLayout from "@/Layouts/AppLayout.vue";
import { Link, useForm, usePage } from "@inertiajs/vue3";
import { computed } from "vue";

const props = defineProps<{
  appName: string;
  heading: string;
  lead: string;
  formAction: string;
  name?: string;
  description?: string;
  statusMessage?: string;
}>();

const page = usePage();
const serverErrors = computed(
  () => (page.props.errors as Record<string, string | string[] | undefined>) ?? {},
);

const form = useForm({
  name: props.name ?? "",
  description: props.description ?? "",
});

function fieldError(field: "name" | "description"): string | undefined {
  const fromForm = form.errors[field];
  if (fromForm) {
    return fromForm;
  }
  const fromPage = serverErrors.value[field];
  if (Array.isArray(fromPage)) {
    return fromPage[0];
  }
  return fromPage;
}

function submit() {
  form.post(props.formAction, { preserveScroll: true });
}
</script>

<template>
  <AppLayout :app-name="appName" :title="heading" :subtitle="lead">
    <p v-if="statusMessage" class="status-banner">{{ statusMessage }}</p>

    <div class="card card-narrow">
      <form class="stack-form" @submit.prevent="submit">
        <label class="field">
          <span>Name</span>
          <input v-model="form.name" type="text" name="name" autocomplete="off" />
          <span v-if="fieldError('name')" class="field-error">{{ fieldError("name") }}</span>
        </label>

        <label class="field">
          <span>Description</span>
          <textarea v-model="form.description" name="description" rows="3" />
          <span v-if="fieldError('description')" class="field-error">{{
            fieldError("description")
          }}</span>
        </label>

        <div class="form-actions">
          <button type="submit" class="btn-primary" :disabled="form.processing">
            Save
          </button>
          <Link href="/examples" class="btn-ghost">Cancel</Link>
        </div>
      </form>
    </div>
  </AppLayout>
</template>
