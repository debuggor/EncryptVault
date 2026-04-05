import { createContext, useContext, useState, ReactNode } from "react";

type Page = "encrypt" | "vault" | "wallet" | "qr" | "settings";

interface AppContextType {
  isUnlocked: boolean;
  setUnlocked: (v: boolean) => void;
  currentPage: Page;
  setPage: (p: Page) => void;
}

const AppContext = createContext<AppContextType | null>(null);

export function AppProvider({ children }: { children: ReactNode }) {
  const [isUnlocked, setUnlocked] = useState(false);
  const [currentPage, setPage] = useState<Page>("encrypt");
  return (
    <AppContext.Provider value={{ isUnlocked, setUnlocked, currentPage, setPage }}>
      {children}
    </AppContext.Provider>
  );
}

export function useApp() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useApp must be used within AppProvider");
  return ctx;
}
