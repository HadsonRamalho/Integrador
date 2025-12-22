import React, { useState } from "react";
import { usePwaInstall } from "@/contexts/PwaInstallContext";
import { Card, CardContent, CardTitle } from "../ui/card";
import { Button } from "../ui/button";

const InstallPrompt: React.FC = () => {
  const { showPrompt, promptInstall } = usePwaInstall();
  const [prompt, setPrompt] = useState(true);

  if (!showPrompt || !prompt) return null;

  return (
      <Card className="fixed bottom-4 left-1/2 transform -translate-x-1/2 bg-white p-4 shadow-lg border rounded-lg z-50 w-[80svw] h-[15svh] border-[hsl(var(--primary))] border-[1px]">
        <CardTitle>Gostaria de instalar o MaqExpress?</CardTitle>
        <CardContent className="mt-2 flex justify-between">
        <Button
          className="ml-2 text-gray-500 bg-white border-[hsl(var(--primary))] border-[0.5px]"
          onClick={() => {setPrompt(false)}}
        >
          Agora não
        </Button>
        <Button className="bg-[hsl(var(--primary))] text-white px-4 py-2 rounded" onClick={promptInstall}>
          Instalar
        </Button>
        </CardContent>
      </Card>
  );
};

export default InstallPrompt;
