import { FormEvent } from "react";
import { useForm, usePage } from "@inertiajs/react";
import AppLayout from "@/Layouts/AppLayout";

type LoginPageProps = {
  appName: string;
  error?: string | null;
};

export default function Login({ appName, error }: LoginPageProps) {
  const page = usePage();
  const csrfToken = page.props.csrfToken as string | null;
  const form = useForm({
    email: "",
    password: "",
    _token: "",
  });

  function submit(event: FormEvent) {
    event.preventDefault();
    if (csrfToken) {
      form.setData("_token", csrfToken);
    }
    form.post("/login");
  }

  return (
    <AppLayout appName={appName} title="Sign in" subtitle="Demo credentials in config/auth.toml">
      <section className="card stack-form">
        {error ? <p className="status-banner status-banner-muted">{error}</p> : null}
        <form className="stack-form" onSubmit={submit}>
          <label className="field">
            <span>Email</span>
            <input
              value={form.data.email}
              onChange={(e) => form.setData("email", e.target.value)}
              type="email"
              required
              autoComplete="username"
            />
          </label>
          <label className="field">
            <span>Password</span>
            <input
              value={form.data.password}
              onChange={(e) => form.setData("password", e.target.value)}
              type="password"
              required
              autoComplete="current-password"
            />
          </label>
          <button type="submit" className="btn btn-primary" disabled={form.processing}>
            Sign in
          </button>
        </form>
        <p className="page-subtitle hint">Demo: admin@omium.local / omium-dev</p>
      </section>
    </AppLayout>
  );
}
