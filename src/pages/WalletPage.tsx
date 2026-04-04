import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ErrorBanner from "../components/ErrorBanner";

interface EthTx {
  chain_id: number; nonce: number; to: string; value: string;
  gas_price: string; gas_limit: number; data: string;
}

const DEFAULT_TX: EthTx = {
  chain_id: 1, nonce: 0, to: "", value: "0",
  gas_price: "20000000000", gas_limit: 21000, data: "0x",
};

export default function WalletPage() {
  const [tab, setTab] = useState<"setup" | "addresses" | "sign">("setup");
  const [mnemonic, setMnemonic] = useState("");
  const [importInput, setImportInput] = useState("");
  const [addrIndex, setAddrIndex] = useState(0);
  const [ethAddr, setEthAddr] = useState("");
  const [btcAddr, setBtcAddr] = useState("");
  const [tx, setTx] = useState<EthTx>(DEFAULT_TX);
  const [signedTx, setSignedTx] = useState("");
  const [error, setError] = useState<string | null>(null);

  async function generate() {
    try { setMnemonic(await invoke<string>("setup_wallet")); }
    catch (err) { setError(String(err)); }
  }

  async function importWallet() {
    try { await invoke("import_wallet", { mnemonic: importInput }); setMnemonic(importInput); }
    catch (err) { setError(String(err)); }
  }

  async function deriveAddresses() {
    try {
      setEthAddr(await invoke<string>("derive_eth_address", { index: addrIndex }));
      setBtcAddr(await invoke<string>("derive_btc_address", { index: addrIndex }));
    } catch (err) { setError(String(err)); }
  }

  async function signTx() {
    try { setSignedTx(await invoke<string>("sign_eth_tx", { index: addrIndex, tx })); }
    catch (err) { setError(String(err)); }
  }

  return (
    <div className="max-w-2xl">
      <h2 className="text-xl font-bold mb-6">Wallet</h2>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />

      <div className="flex gap-2 mb-6">
        {(["setup", "addresses", "sign"] as const).map(t => (
          <button key={t} onClick={() => setTab(t)}
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${tab === t ? "bg-blue-600 text-white" : "bg-gray-200 text-gray-700 hover:bg-gray-300"}`}>
            {t.charAt(0).toUpperCase() + t.slice(1)}
          </button>
        ))}
      </div>

      {tab === "setup" && (
        <div className="space-y-5">
          <button onClick={generate}
            className="bg-blue-600 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-blue-700">
            Generate new wallet
          </button>
          {mnemonic && (
            <div className="bg-yellow-50 border border-yellow-300 rounded-xl p-4">
              <p className="text-xs text-yellow-800 font-semibold mb-2">Write down your mnemonic — it will not be shown again.</p>
              <p className="font-mono text-sm break-all">{mnemonic}</p>
            </div>
          )}
          <hr />
          <div className="space-y-2">
            <p className="text-sm font-medium text-gray-700">Import existing wallet</p>
            <textarea value={importInput} onChange={e => setImportInput(e.target.value)}
              placeholder="Enter 12-word mnemonic…" rows={3}
              className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm font-mono" />
            <button onClick={importWallet}
              className="bg-gray-700 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-gray-800">
              Import
            </button>
          </div>
        </div>
      )}

      {tab === "addresses" && (
        <div className="space-y-4">
          <div className="flex items-center gap-3">
            <label className="text-sm text-gray-600">Index</label>
            <input type="number" min={0} value={addrIndex}
              onChange={e => setAddrIndex(Number(e.target.value))}
              className="w-24 border border-gray-300 rounded-lg px-3 py-1.5 text-sm" />
            <button onClick={deriveAddresses}
              className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-blue-700">
              Derive
            </button>
          </div>
          {ethAddr && (
            <div className="space-y-2">
              <AddrRow label="ETH" addr={ethAddr} />
              <AddrRow label="BTC" addr={btcAddr} />
            </div>
          )}
        </div>
      )}

      {tab === "sign" && (
        <div className="space-y-3">
          <p className="text-sm text-gray-600">Sign ETH transaction offline (EIP-155)</p>
          {(Object.keys(tx) as (keyof EthTx)[]).map(f => (
            <div key={f} className="flex items-center gap-3">
              <label className="text-xs text-gray-500 w-24 shrink-0">{f}</label>
              <input value={String(tx[f])}
                onChange={e => setTx({ ...tx, [f]: ["chain_id","nonce","gas_limit"].includes(f) ? Number(e.target.value) : e.target.value })}
                className="flex-1 border border-gray-300 rounded-lg px-3 py-1.5 text-sm font-mono" />
            </div>
          ))}
          <button onClick={signTx}
            className="bg-blue-600 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-blue-700">
            Sign transaction
          </button>
          {signedTx && (
            <div className="bg-gray-100 rounded-lg p-4">
              <p className="text-xs text-gray-500 mb-1">Signed tx hex</p>
              <pre className="text-xs font-mono break-all whitespace-pre-wrap">{signedTx}</pre>
              <button onClick={() => navigator.clipboard.writeText(signedTx)}
                className="mt-2 text-xs text-blue-600 hover:underline">Copy</button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

function AddrRow({ label, addr }: { label: string; addr: string }) {
  return (
    <div className="bg-white border rounded-xl p-4 flex justify-between items-center shadow-sm">
      <div>
        <p className="text-xs text-gray-500 font-medium">{label}</p>
        <p className="text-sm font-mono break-all">{addr}</p>
      </div>
      <button onClick={() => navigator.clipboard.writeText(addr)}
        className="text-xs text-blue-600 hover:underline ml-4 shrink-0">Copy</button>
    </div>
  );
}
