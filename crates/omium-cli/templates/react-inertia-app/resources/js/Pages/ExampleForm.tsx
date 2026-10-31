import AppLayout from "@/Layouts/AppLayout";
import { Link, useForm, usePage } from "@inertiajs/react";
import type { FormEvent } from "react";

type ExampleFormProps = {
  appName: string;
  heading: string;
  lead: string;
  formAction: string;
  name?: string;
  description?: string;
  statusMessage?: string;
};

export default function ExampleForm({
  appName,
  heading,
  lead,
  formAction,
  name,
  description,
  statusMessage,
}: ExampleFormProps) {
  const page = usePage();
  const serverErrors =
    (page.props.errors as Record<string, string | string[] | undefined> | undefined) ?? {};

  const form = useForm({
    name: name ?? "",
    description: description ?? "",
  });

  function fieldError(field: "name" | "description"): string | undefined {
    const fromForm = form.errors[field];
    if (fromForm) {
      return fromForm;
    }
    const fromPage = serverErrors[field];
    if (Array.isArray(fromPage)) {
      return fromPage[0];
    }
    return fromPage;
  }

  function submit(event: FormEvent) {
    event.preventDefault();
    form.post(formAction, { preserveScroll: true });
  }

  return (
    <AppLayout appName={appName} title={heading} subtitle={lead}>
      {statusMessage ? <p className="status-banner">{statusMessage}</p> : null}

      <div className="card card-narrow">
        <form className="stack-form" onSubmit={submit}>
          <label className="field">
            <span>Name</span>
            <input
              type="text"
              name="name"
              autoComplete="off"
              value={form.data.name}
              onChange={(event) => form.setData("name", event.target.value)}
            />
            {fieldError("name") ? <span className="field-error">{fieldError("name")}</span> : null}
          </label>

          <label className="field">
            <span>Description</span>
            <textarea
              name="description"
              rows={3}
              value={form.data.description}
              onChange={(event) => form.setData("description", event.target.value)}
            />
            {fieldError("description") ? (
              <span className="field-error">{fieldError("description")}</span>
            ) : null}
          </label>

          <div className="form-actions">
            <button type="submit" className="btn-primary" disabled={form.processing}>
              Save
            </button>
            <Link href="/examples" className="btn-ghost">
              Cancel
            </Link>
          </div>
        </form>
      </div>
    </AppLayout>
  );
}
