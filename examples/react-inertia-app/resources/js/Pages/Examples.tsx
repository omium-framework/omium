import AppLayout from "@/Layouts/AppLayout";
import { Link, router } from "@inertiajs/react";

type ExampleRow = {
  id: number;
  name: string;
  description: string;
};

type ExamplesProps = {
  appName: string;
  rows: ExampleRow[];
  dbEnabled?: boolean;
  statusMessage?: string;
};

export default function Examples({ appName, rows, dbEnabled, statusMessage }: ExamplesProps) {
  function destroy(id: number) {
    if (!window.confirm("Delete this example?")) {
      return;
    }
    router.delete(`/examples/${id}`, { preserveScroll: true });
  }

  return (
    <AppLayout
      appName={appName}
      title="Examples"
      subtitle="Validation demo: create, edit, and delete rows via Inertia."
    >
      {statusMessage ? <p className="status-banner">{statusMessage}</p> : null}
      {dbEnabled === false ? (
        <p className="status-banner status-banner-muted">
          Database is off — run ./omnisan install and ./omnisan migrate.
        </p>
      ) : null}

      <div className="page-toolbar">
        <Link href="/examples/new" className="btn-primary">
          New example
        </Link>
      </div>

      <div className="card card-wide">
        <table className="data-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Description</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.id}>
                <td>{row.id}</td>
                <td>{row.name}</td>
                <td>{row.description}</td>
                <td className="table-actions">
                  {row.id > 0 ? (
                    <>
                      <Link href={`/examples/${row.id}/edit`} className="nav-link-inline">
                        Edit
                      </Link>
                      <button type="button" className="btn-link-danger" onClick={() => destroy(row.id)}>
                        Delete
                      </button>
                    </>
                  ) : null}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </AppLayout>
  );
}
