import {
  createContext,
  useContext,
  useState,
  ReactNode,
  useEffect,
} from "react";

type Page = "encrypt" | "vault" | "wallet" | "qr" | "settings";
export type Theme = "light" | "night" | "system";

interface AppContextType {
  isUnlocked: boolean;
  setUnlocked: (v: boolean) => void;
  currentPage: Page;
  setPage: (p: Page) => void;
  theme: Theme;
  setTheme: (t: Theme) => void;
}

const AppContext = createContext<AppContextType | null>(null);

export function AppProvider({ children }: { children: ReactNode }) {
  const [isUnlocked, setUnlocked] = useState(false);
  const [currentPage, setPage] = useState<Page>("encrypt");
  const [theme, setTheme] = useState<Theme>(() => {
    const stored = localStorage.getItem("encryptvault-theme");
    if (stored === "light" || stored === "night" || stored === "system") {
      return stored;
    }
    return "system";
  });

  useEffect(() => {
    // Persist theme choice
    localStorage.setItem("encryptvault-theme", theme);

    // Determine effective mode
    const effectiveMode =
      theme === "system"
        ? window.matchMedia("(prefers-color-scheme: dark)").matches
          ? "night"
          : "light"
        : theme;

    // Apply dark class
    if (effectiveMode === "night") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }

    // Listen for system preference changes (only when theme === "system")
    if (theme === "system") {
      const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
      const handleChange = () => {
        if (mediaQuery.matches) {
          document.documentElement.classList.add("dark");
        } else {
          document.documentElement.classList.remove("dark");
        }
      };
      mediaQuery.addEventListener("change", handleChange);
      return () => mediaQuery.removeEventListener("change", handleChange);
    }
  }, [theme]);

  return (
    <AppContext.Provider
      value={{ isUnlocked, setUnlocked, currentPage, setPage, theme, setTheme }}
    >
      {children}
    </AppContext.Provider>
  );
}

export function useApp() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useApp must be used within AppProvider");
  return ctx;
}
