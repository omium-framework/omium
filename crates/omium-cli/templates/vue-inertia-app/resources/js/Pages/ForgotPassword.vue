<script setup lang="ts">
import { computed } from "vue";
import { Link, useForm, usePage } from "@inertiajs/vue3";
import AppLayout from "@/Layouts/AppLayout.vue";
import AuthPanel from "@/components/AuthPanel.vue";
import FormButton from "@/components/FormButton.vue";
import FormInput from "@/components/FormInput.vue";

defineProps<{
  appName: string;
  status?: string | null;
  error?: string | null;
}>();

const page = usePage();
const csrfToken = computed(() => page.props.csrfToken as string | null);

const form = useForm({
  email: "",
  _token: "",
});

function submit() {
  if (csrfToken.value) {
    form._token = csrfToken.value;
  }
  form.post("/forgot-password", { preserveScroll: true });
}
</script>

<template>
  <AppLayout
    :app-name="appName"
    title="Forgot password"
    subtitle="We email a reset link when the address is registered"
  >
    <AuthPanel title="Reset your password">
      <p v-if="status" class="status-banner" style="margin-bottom: 0.75rem">{{ status }}</p>
      <p v-if="error" class="form-error" style="margin-bottom: 0.75rem">{{ error }}</p>
      <form class="stack-form" @submit.prevent="submit">
        <FormInput
          v-model="form.email"
          label="Email"
          type="email"
          autocomplete="username"
          required
        />
        <FormButton type="submit" block :disabled="form.processing">Send reset link</FormButton>
      </form>
      <Link href="/login" class="auth-footer-link">Back to sign in</Link>
    </AuthPanel>
  </AppLayout>
</template>
