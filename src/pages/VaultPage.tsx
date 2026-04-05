import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ErrorBanner from "../components/ErrorBanner";

interface Credential {
  id: string;
  name: string;
  username: string;
  password: string;
  url: string;
  notes: string;
  created_at: number;
  updated_at: number;
}

const EMPTY = { name: "", username: "", password: "", url: "", notes: "" };

export default function VaultPage() {
  const [creds, setCreds] = useState<Credential[]>([]);
  const [search, setSearch] = useState("");
  const [form, setForm] = useState(EMPTY);
  const [editId, setEditId] = useState<string | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showPw, setShowPw] = useState<Record<string, boolean>>({});

  async function load() {
    try {
      setCreds(await invoke<Credential[]>("list_credentials"));
    } catch (err) {
      setError(String(err));
    }
  }

  useEffect(() => {
    load();
  }, []);

  const filtered = search.trim()
    ? creds.filter(
        (c) =>
          c.name.toLowerCase().includes(search.toLowerCase()) ||
          c.url.toLowerCase().includes(search.toLowerCase()),
      )
    : creds;

  async function save(e: React.FormEvent) {
    e.preventDefault();
    try {
      if (editId) await invoke("update_credential", { id: editId, ...form });
      else await invoke("add_credential", form);
      setForm(EMPTY);
      setEditId(null);
      setShowForm(false);
      await load();
    } catch (err) {
      setError(String(err));
    }
  }

  async function remove(id: string) {
    if (!confirm("Delete this credential?")) return;
    try {
      await invoke("delete_credential", { id });
      await load();
    } catch (err) {
      setError(String(err));
    }
  }

  function startEdit(c: Credential) {
    setForm({
      name: c.name,
      username: c.username,
      password: c.password,
      url: c.url,
      notes: c.notes,
    });
    setEditId(c.id);
    setShowForm(true);
  }

  return (
    <div className="max-w-3xl">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-xl font-bold text-gray-900 dark:text-gray-50">
          Password Vault
        </h2>
        <button
          onClick={() => {
            setForm(EMPTY);
            setEditId(null);
            setShowForm(true);
          }}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-blue-700"
        >
          + Add
        </button>
      </div>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />

      <input
        placeholder="Search…"
        value={search}
        onChange={(e) => setSearch(e.target.value)}
        className="w-full border border-gray-300 dark:border-gray-700 rounded-lg px-4 py-2 text-sm mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-50"
      />

      {showForm && (
        <form
          onSubmit={save}
          className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-5 mb-4 space-y-3 shadow-sm"
        >
          <h3 className="font-semibold text-sm text-gray-900 dark:text-gray-50">
            {editId ? "Edit" : "New credential"}
          </h3>
          {(["name", "username", "password", "url", "notes"] as const).map(
            (f) => (
              <input
                key={f}
                placeholder={f.charAt(0).toUpperCase() + f.slice(1)}
                type={f === "password" ? "password" : "text"}
                value={form[f]}
                onChange={(e) => setForm({ ...form, [f]: e.target.value })}
                required={f === "name"}
                className="w-full border border-gray-300 dark:border-gray-700 rounded-lg px-3 py-2 text-sm bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-50"
              />
            ),
          )}
          <div className="flex gap-2">
            <button
              type="submit"
              className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium"
            >
              Save
            </button>
            <button
              type="button"
              onClick={() => setShowForm(false)}
              className="text-sm text-gray-500 dark:text-gray-400 hover:underline"
            >
              Cancel
            </button>
          </div>
        </form>
      )}

      <div className="space-y-2">
        {filtered.length === 0 && (
          <p className="text-sm text-gray-400 dark:text-gray-500">
            No credentials found.
          </p>
        )}
        {filtered.map((c) => (
          <div
            key={c.id}
            className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl px-5 py-4 flex justify-between items-start shadow-sm"
          >
            <div>
              <p className="font-medium text-sm text-gray-900 dark:text-gray-50">
                {c.name}
              </p>
              <p className="text-xs text-gray-500 dark:text-gray-400">
                {c.username}
              </p>
              {c.url && <p className="text-xs text-blue-500">{c.url}</p>}
              <div className="text-xs text-gray-400 dark:text-gray-500 mt-1 font-mono">
                {showPw[c.id] ? c.password : "••••••••"}
                <button
                  onClick={() => setShowPw((p) => ({ ...p, [c.id]: !p[c.id] }))}
                  className="ml-2 text-blue-500 hover:underline"
                >
                  {showPw[c.id] ? "hide" : "show"}
                </button>
                <button
                  onClick={() => navigator.clipboard.writeText(c.password)}
                  className="ml-2 text-blue-500 hover:underline"
                >
                  copy
                </button>
              </div>
            </div>
            <div className="flex gap-2 ml-4 shrink-0">
              <button
                onClick={() => startEdit(c)}
                className="text-xs text-gray-500 hover:underline"
              >
                Edit
              </button>
              <button
                onClick={() => remove(c.id)}
                className="text-xs text-red-500 hover:underline"
              >
                Delete
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
