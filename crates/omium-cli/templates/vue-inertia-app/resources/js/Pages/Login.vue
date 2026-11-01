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
const verified = computed(() => new URLSearchParams(window.location.search).get("verified"));

const form = useForm({
  email: "",
  password: "",
  _token: "",
});

function submit() {
  if (csrfToken.value) {
    form._token = csrfToken.value;
  }
  form.post("/login", { preserveScroll: true });
}
</script>

<template>
  <AppLayout
    :app-name="appName"
    title="Sign in"
    subtitle="Demo admin or an account you registered"
  >
    <AuthPanel title="Welcome back" lead="Use your email and password to continue.">
      <p
        v-if="verified === '1'"
        class="status-banner"
        style="margin-bottom: 0.75rem"
      >
        Email verified. You can sign in now.
      </p>
      <p
        v-else-if="verified === '0'"
        class="status-banner status-banner-muted"
        style="margin-bottom: 0.75rem"
      >
        Verification link is invalid or expired.
      </p>
      <p v-if="error" class="form-error" style="margin-bottom: 0.75rem">{{ error }}</p>
      <form class="stack-form" @submit.prevent="submit">
        <FormInput
          v-model="form.email"
          label="Email"
          type="email"
          autocomplete="username"
          required
        />
        <FormInput
          v-model="form.password"
          label="Password"
          type="password"
          autocomplete="current-password"
          required
        />
        <FormButton type="submit" block :disabled="form.processing">Sign in</FormButton>
      </form>
      <div class="auth-footer-row">
        <Link href="/forgot-password" class="auth-footer-link auth-footer-link-accent">
          Forgot password?
        </Link>
        <Link href="/register" class="auth-footer-link">Create account</Link>
      </div>
      <p class="muted" style="margin: 0.75rem 0 0">Demo admin: admin@omium.local / omium-dev</p>
    </AuthPanel>
  </AppLayout>
</template>
