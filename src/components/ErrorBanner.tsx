interface Props {
  message: string | null;
  onDismiss: () => void;
}

export default function ErrorBanner({ message, onDismiss }: Props) {
  if (!message) return null;
  return (
    <div className="bg-red-100 dark:bg-red-900/20 border border-red-400 dark:border-red-800 text-red-800 dark:text-red-300 px-4 py-3 rounded flex justify-between items-center mb-4">
      <span className="text-sm">{message}</span>
      <button
        onClick={onDismiss}
        className="ml-4 font-bold text-red-600 dark:text-red-400"
      >
        ×
      </button>
    </div>
  );
}
