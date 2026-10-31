import AppLayout from "@/Layouts/AppLayout";

type AboutProps = {
  appName: string;
  stack: string[];
};

export default function About({ appName, stack }: AboutProps) {
  return (
    <AppLayout
      appName={appName}
      title="About"
      subtitle="How this demo relates to Laravel + Inertia."
    >
      <div className="card">
        <p className="lead">
          Routes and props come from Rust. React only renders what the server sends on each visit
          — navigation uses Inertia, not a separate client-side router.
        </p>
        <ul className="stack-list">
          {stack.map((item) => (
            <li key={item}>{item}</li>
          ))}
        </ul>
      </div>
    </AppLayout>
  );
}
