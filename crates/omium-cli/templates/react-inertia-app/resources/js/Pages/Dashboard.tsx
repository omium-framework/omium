import AppLayout from "@/Layouts/AppLayout";

type DashboardProps = {
  greeting: string;
  hint: string;
  appName: string;
  modeLabel: string;
};

export default function Dashboard({ greeting, hint, appName, modeLabel }: DashboardProps) {
  return (
    <AppLayout
      appName={appName}
      title="Welcome"
      subtitle="Server-driven React pages with client-side navigation."
    >
      <section className="welcome-hero">
        <div className="welcome-glow" aria-hidden="true" />
        <p className="hero-eyebrow">You&apos;re all set</p>
        <h2 className="hero-title">{greeting}</h2>
        <p className="hero-lead">{hint}</p>
        <div className="pill-row">
          <span className="pill pill-accent">{modeLabel}</span>
          <span className="pill">Inertia</span>
          <span className="pill">Axum</span>
          <span className="pill">React</span>
        </div>
        <div className="feature-grid">
          <div className="feature-tile">
            <strong>Rust owns routes</strong>
            <span>URLs, auth, and data loading stay on the server.</span>
          </div>
          <div className="feature-tile">
            <strong>SPA feel</strong>
            <span>Inertia swaps pages without a separate client router.</span>
          </div>
          <div className="feature-tile">
            <strong>Shared design</strong>
            <span>Matches Standard and Vue starters out of the box.</span>
          </div>
        </div>
      </section>
    </AppLayout>
  );
}
