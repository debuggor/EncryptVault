import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ErrorBanner from "../components/ErrorBanner";

export default function QRPage() {
  const [tab, setTab] = useState<"generate" | "decode">("generate");
  const [content, setContent] = useState("");
  const [qrUrl, setQrUrl] = useState("");
  const [decoded, setDecoded] = useState("");
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const videoRef = useRef<HTMLVideoElement>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const rafRef = useRef<number | null>(null);
  const fileRef = useRef<HTMLInputElement>(null);

  async function generateQr() {
    try {
      setQrUrl(await invoke<string>("cmd_generate_qr", { content }));
    } catch (err) {
      setError(String(err));
    }
  }

  async function decodeFile() {
    const file = fileRef.current?.files?.[0];
    if (!file) return;
    const filePath = (file as File & { path?: string }).path ?? file.name;
    try {
      setDecoded(await invoke<string>("cmd_decode_qr_file", { filePath }));
    } catch (err) {
      setError(String(err));
    }
  }

  async function startCamera() {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: "environment" },
      });
      streamRef.current = stream;
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
      }
      setScanning(true);
    } catch (err) {
      setError(String(err));
    }
  }

  function stopCamera() {
    if (rafRef.current) cancelAnimationFrame(rafRef.current);
    streamRef.current?.getTracks().forEach((t) => t.stop());
    streamRef.current = null;
    setScanning(false);
  }

  useEffect(() => {
    if (!scanning) return;
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d")!;

    function loop() {
      const video = videoRef.current;
      if (video && video.readyState >= 2) {
        canvas.width = video.videoWidth;
        canvas.height = video.videoHeight;
        ctx.drawImage(video, 0, 0);
        const frame = ctx.getImageData(0, 0, canvas.width, canvas.height);
        invoke<string | null>("cmd_decode_qr_frame", {
          rgba: Array.from(frame.data),
          width: frame.width,
          height: frame.height,
        }).then((result) => {
          if (result) {
            setDecoded(result);
            stopCamera();
          } else {
            rafRef.current = requestAnimationFrame(loop);
          }
        });
      } else {
        rafRef.current = requestAnimationFrame(loop);
      }
    }
    rafRef.current = requestAnimationFrame(loop);
    return () => {
      if (rafRef.current) cancelAnimationFrame(rafRef.current);
    };
  }, [scanning]);

  useEffect(() => () => stopCamera(), []);

  return (
    <div className="max-w-2xl">
      <h2 className="text-xl font-bold text-gray-900 dark:text-gray-50 mb-6">
        QR Code
      </h2>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />

      <div className="flex gap-2 mb-6">
        {(["generate", "decode"] as const).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${tab === t ? "bg-blue-600 text-white" : "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"}`}
          >
            {t.charAt(0).toUpperCase() + t.slice(1)}
          </button>
        ))}
      </div>

      {tab === "generate" && (
        <div className="space-y-4">
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            placeholder="Text or URL to encode…"
            rows={3}
            className="w-full border border-gray-300 dark:border-gray-700 rounded-lg px-4 py-2 text-sm bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-50"
          />
          <button
            onClick={generateQr}
            className="bg-blue-600 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-blue-700"
          >
            Generate QR
          </button>
          {qrUrl && (
            <div className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-4 flex flex-col items-center gap-3">
              <img src={qrUrl} alt="QR code" className="w-48 h-48" />
              <a
                href={qrUrl}
                download="qrcode.png"
                className="text-xs text-blue-600 hover:underline"
              >
                Download PNG
              </a>
            </div>
          )}
        </div>
      )}

      {tab === "decode" && (
        <div className="space-y-4">
          <div className="flex gap-2 flex-wrap">
            <label className="bg-gray-700 text-white px-4 py-2 rounded-lg text-sm font-medium cursor-pointer hover:bg-gray-800">
              Open image file
              <input
                ref={fileRef}
                type="file"
                accept="image/*"
                className="hidden"
                onChange={decodeFile}
              />
            </label>
            <button
              onClick={scanning ? stopCamera : startCamera}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${scanning ? "bg-red-600 text-white hover:bg-red-700" : "bg-blue-600 text-white hover:bg-blue-700"}`}
            >
              {scanning ? "Stop camera" : "Use camera"}
            </button>
          </div>

          {scanning && (
            <div className="rounded-xl overflow-hidden border bg-black">
              <video
                ref={videoRef}
                autoPlay
                playsInline
                muted
                className="w-full"
              />
            </div>
          )}

          {decoded && (
            <div className="bg-gray-100 dark:bg-gray-800 rounded-xl p-4">
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-1 font-medium">
                Decoded content
              </p>
              <p className="text-sm font-mono break-all text-gray-900 dark:text-gray-50">
                {decoded}
              </p>
              <button
                onClick={() => navigator.clipboard.writeText(decoded)}
                className="mt-2 text-xs text-blue-600 hover:underline"
              >
                Copy
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
