<script setup lang="ts">
import { computed } from "vue";
import { useForm, usePage } from "@inertiajs/vue3";
import AppLayout from "@/Layouts/AppLayout.vue";
import AuthPanel from "@/components/AuthPanel.vue";
import FormButton from "@/components/FormButton.vue";
import FormInput from "@/components/FormInput.vue";

const props = defineProps<{
  appName: string;
  token: string;
  error?: string | null;
}>();

const page = usePage();
const csrfToken = computed(() => page.props.csrfToken as string | null);

const form = useForm({
  password: "",
  password_confirmation: "",
  _token: "",
});

function submit() {
  if (csrfToken.value) {
    form._token = csrfToken.value;
  }
  form.post(`/reset-password/${props.token}`, { preserveScroll: true });
}
</script>

<template>
  <AppLayout :app-name="appName" title="Reset password" subtitle="Choose a new password">
    <AuthPanel title="New password">
      <p v-if="error" class="form-error" style="margin-bottom: 0.75rem">{{ error }}</p>
      <form class="stack-form" @submit.prevent="submit">
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
        <FormButton type="submit" block :disabled="form.processing">Update password</FormButton>
      </form>
    </AuthPanel>
  </AppLayout>
</template>
