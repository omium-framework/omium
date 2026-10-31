import { Link, usePage } from "@inertiajs/react";
import type { ReactNode } from "react";

type NavItem = { href: string; label: string; active: boolean };
type FlashProps = { success?: string; error?: string };

type AppLayoutProps = {
  appName: string;
  title: string;
  subtitle?: string;
  children: ReactNode;
};

export default function AppLayout({ appName, title, subtitle, children }: AppLayoutProps) {
  const page = usePage();
  const nav = (page.props.nav as NavItem[] | undefined) ?? [];
  const flash = (page.props.flash as FlashProps | undefined) ?? {};

  return (
    <div className="app-shell">
      <header className="app-header">
        <div className="app-header-inner">
          <div>
            <p className="brand">{appName}</p>
            <h1 className="page-title">{title}</h1>
            {subtitle ? <p className="page-subtitle">{subtitle}</p> : null}
          </div>
          <nav className="app-nav" aria-label="Main">
            {nav.map((item) => (
              <Link
                key={item.href}
                href={item.href}
                className={`nav-link${item.active ? " nav-link-active" : ""}`}
              >
                {item.label}
              </Link>
            ))}
          </nav>
        </div>
      </header>
      <main className="app-main">
        {flash.success ? <p className="status-banner">{flash.success}</p> : null}
        {flash.error ? <p className="status-banner status-banner-muted">{flash.error}</p> : null}
        {children}
      </main>
    </div>
  );
}
