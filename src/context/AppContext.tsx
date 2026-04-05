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
    // Determine effective mode (system → follow OS)
    const effectiveMode =
      theme === "system"
        ? window.matchMedia("(prefers-color-scheme: dark)").matches
          ? "night"
          : "light"
        : theme;

    // Toggle dark class
    const html = document.documentElement;
    if (effectiveMode === "night") {
      html.classList.add("dark");
    } else {
      html.classList.remove("dark");
    }

    // Persist choice
    localStorage.setItem("encryptvault-theme", theme);

    // System mode live updates
    if (theme === "system") {
      const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
      const handleChange = () => {
        const effective = mediaQuery.matches ? "night" : "light";
        if (effective === "night") {
          html.classList.add("dark");
        } else {
          html.classList.remove("dark");
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
