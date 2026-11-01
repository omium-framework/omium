<script setup lang="ts">
import { computed } from "vue";
import { Link, useForm, usePage } from "@inertiajs/vue3";
import AppLayout from "@/Layouts/AppLayout.vue";
import AuthPanel from "@/components/AuthPanel.vue";
import FormButton from "@/components/FormButton.vue";
import FormInput from "@/components/FormInput.vue";

defineProps<{
  appName: string;
  error?: string | null;
}>();

const page = usePage();
const csrfToken = computed(() => page.props.csrfToken as string | null);

const form = useForm({
  name: "",
  email: "",
  password: "",
  password_confirmation: "",
  _token: "",
});

function submit() {
  if (csrfToken.value) {
    form._token = csrfToken.value;
  }
  form.post("/register", { preserveScroll: true });
}
</script>

<template>
  <AppLayout
    :app-name="appName"
    title="Create account"
    subtitle="Demo registration — check Mailpit for the verification email"
  >
    <AuthPanel
      title="Sign up"
      lead="Accounts are stored locally for now. After registering, open Mailpit at port 8025."
    >
      <p v-if="error" class="form-error" style="margin-bottom: 0.75rem">{{ error }}</p>
      <form class="stack-form" @submit.prevent="submit">
        <FormInput v-model="form.name" label="Name" autocomplete="name" required />
        <FormInput
          v-model="form.email"
          label="Email"
          type="email"
          autocomplete="email"
          required
        />
        <FormInput
          v-model="form.password"
          label="Password"
          type="password"
          autocomplete="new-password"
          required
        />
        <FormInput
          v-model="form.password_confirmation"
          label="Confirm password"
          type="password"
          autocomplete="new-password"
          required
        />
        <FormButton type="submit" block :disabled="form.processing">Create account</FormButton>
      </form>
      <Link href="/login" class="auth-footer-link">Already have an account? Sign in</Link>
    </AuthPanel>
  </AppLayout>
</template>
