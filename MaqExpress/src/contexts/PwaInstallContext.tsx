import React, { createContext, useContext, useEffect, useState } from "react";

interface PwaInstallContextType {
  showPrompt: boolean;
  promptInstall: () => void;
}

const PwaInstallContext = createContext<PwaInstallContextType | null>(null);

export const PwaInstallProvider = ({ children }: { children: React.ReactNode }) => {
  const [deferredPrompt, setDeferredPrompt] = useState<Event | null>(null);
  const [showPrompt, setShowPrompt] = useState(false);

  useEffect(() => {
    const handler = (e: Event) => {
      e.preventDefault();
      setDeferredPrompt(e);
      setShowPrompt(true);
    };

    window.addEventListener("beforeinstallprompt", handler);
    return () => window.removeEventListener("beforeinstallprompt", handler);
  }, []);

  const promptInstall = () => {
    if (deferredPrompt) {
      (deferredPrompt as any).prompt();
      (deferredPrompt as any).userChoice.then((choiceResult: any) => {
        if (choiceResult.outcome === "accepted") {
          console.log("Usuário aceitou instalar o app.");
        } else {
          console.log("Usuário recusou a instalação.");
        }
        setDeferredPrompt(null);
        setShowPrompt(false);
      });
    }
  };

  return (
    <PwaInstallContext.Provider value={{ showPrompt, promptInstall }}>
      {children}
    </PwaInstallContext.Provider>
  );
};

export const usePwaInstall = () => {
  const context = useContext(PwaInstallContext);
  if (!context) throw new Error("usePwaInstall deve ser usado dentro de PwaInstallProvider");
  return context;
};
