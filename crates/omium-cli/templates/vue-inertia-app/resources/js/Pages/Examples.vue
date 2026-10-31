<script setup lang="ts">
import AppLayout from "@/Layouts/AppLayout.vue";
import { Link, router } from "@inertiajs/vue3";

type ExampleRow = {
  id: number;
  name: string;
  description: string;
};

defineProps<{
  appName: string;
  rows: ExampleRow[];
  dbEnabled?: boolean;
  statusMessage?: string;
}>();

function destroy(id: number) {
  if (!window.confirm("Delete this example?")) {
    return;
  }
  router.delete(`/examples/${id}`, { preserveScroll: true });
}
</script>

<template>
  <AppLayout
    :app-name="appName"
    title="Examples"
    subtitle="Validation demo: create, edit, and delete rows via Inertia."
  >
    <p v-if="statusMessage" class="status-banner">{{ statusMessage }}</p>
    <p v-if="dbEnabled === false" class="status-banner status-banner-muted">
      Database is off — run ./omnisan install and ./omnisan migrate.
    </p>

    <div class="page-toolbar">
      <Link href="/examples/new" class="btn-primary">New example</Link>
    </div>

    <div class="card card-wide">
      <table class="data-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Description</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in rows" :key="row.id">
            <td>{{ row.id }}</td>
            <td>{{ row.name }}</td>
            <td>{{ row.description }}</td>
            <td class="table-actions">
              <template v-if="row.id > 0">
                <Link :href="`/examples/${row.id}/edit`" class="nav-link-inline">Edit</Link>
                <button type="button" class="btn-link-danger" @click="destroy(row.id)">
                  Delete
                </button>
              </template>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </AppLayout>
</template>
