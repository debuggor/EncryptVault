import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ErrorBanner from "../components/ErrorBanner";

type Mode = "text" | "file";
type Op = "encrypt" | "decrypt";

export default function EncryptPage() {
  const [mode, setMode] = useState<Mode>("text");
  const [passphrase, setPassphrase] = useState("");
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const fileRef = useRef<HTMLInputElement>(null);

  async function handle(op: Op) {
    setError(null);
    setLoading(true);
    try {
      if (mode === "text") {
        const result =
          op === "encrypt"
            ? await invoke<string>("cmd_encrypt_text", {
                passphrase,
                plaintext: input,
              })
            : await invoke<string>("cmd_decrypt_text", {
                passphrase,
                ciphertext: input,
              });
        setOutput(result);
      } else {
        const file = fileRef.current?.files?.[0];
        if (!file) {
          setError("Select a file first.");
          return;
        }
        const filePath = (file as File & { path?: string }).path ?? file.name;
        const result =
          op === "encrypt"
            ? await invoke<string>("cmd_encrypt_file", { passphrase, filePath })
            : await invoke<string>("cmd_decrypt_file", {
                passphrase,
                filePath,
              });
        setOutput(`Saved to: ${result}`);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="max-w-2xl">
      <h2 className="text-xl font-bold text-gray-900 dark:text-gray-50 mb-6">
        Encrypt / Decrypt
      </h2>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />

      <div className="flex gap-2 mb-4">
        {(["text", "file"] as Mode[]).map((m) => (
          <button
            key={m}
            onClick={() => setMode(m)}
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${mode === m ? "bg-blue-600 text-white" : "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"}`}
          >
            {m === "text" ? "Text" : "File"}
          </button>
        ))}
      </div>

      <div className="space-y-3">
        <input
          type="password"
          placeholder="Passphrase"
          value={passphrase}
          onChange={(e) => setPassphrase(e.target.value)}
          className="w-full border border-gray-300 dark:border-gray-700 rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-50"
        />

        {mode === "text" ? (
          <textarea
            placeholder="Paste text here…"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            rows={5}
            className="w-full border border-gray-300 dark:border-gray-700 rounded-lg px-4 py-2 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-50"
          />
        ) : (
          <input
            ref={fileRef}
            type="file"
            className="w-full text-sm text-gray-600 file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
          />
        )}

        <div className="flex gap-2">
          <button
            onClick={() => handle("encrypt")}
            disabled={loading}
            className="bg-blue-600 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors"
          >
            Encrypt
          </button>
          <button
            onClick={() => handle("decrypt")}
            disabled={loading}
            className="bg-gray-700 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-gray-800 disabled:opacity-50 transition-colors"
          >
            Decrypt
          </button>
        </div>

        {output && (
          <div className="bg-gray-100 dark:bg-gray-800 rounded-lg p-4">
            <p className="text-xs text-gray-500 dark:text-gray-400 mb-1 font-medium">
              Output
            </p>
            <pre className="text-sm font-mono whitespace-pre-wrap break-all">
              {output}
            </pre>
            <button
              onClick={() => navigator.clipboard.writeText(output)}
              className="mt-2 text-xs text-blue-600 hover:underline"
            >
              Copy
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
