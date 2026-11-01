import { useState } from "react";
import { CodeEditor } from "./components/code-editor";
import { OutputPanel } from "./components/output-panel";

interface RunResponse {
  success: boolean;
  output: string;
  error: string | null;
}

function App() {
  const [isRunning, setIsRunning] = useState(false);
  const [result, setResult] = useState<RunResponse | null>(null);

  async function handleRun(code: string) {
    setIsRunning(true);
    setResult(null);

    try {
      const response = await fetch("http://localhost:3001/api/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code }),
      });

      if (!response.ok) {
        throw new Error(`Server error: ${response.status}`);
      }

      const data: RunResponse = await response.json();
      setResult(data);
    } catch (err) {
      setResult({
        success: false,
        output: "",
        error: err instanceof Error ? err.message : "Unknown error",
      });
    } finally {
      setIsRunning(false);
    }
  }

  return (
    <div className="h-screen w-screen flex flex-col bg-background overflow-hidden text-white">
      <header className="flex items-center justify-center h-14 bg-[#2d2d30] border-b border-gray-700 shrink-0">
        <h1 className="text-white font-bold text-2xl tracking-wider">RLOX</h1>
      </header>

      <main className="flex-1 flex flex-row overflow-hidden w-full h-[calc(100vh-3.5rem)]">
        <div className="w-1/2 h-full flex flex-col overflow-hidden border-r border-gray-700">
          <CodeEditor onRun={handleRun} isRunning={isRunning} />
        </div>
        <div className="w-1/2 h-full flex flex-col overflow-hidden">
          <OutputPanel
            output={result?.output || ""}
            error={result?.error ?? null}
            success={result?.success || false}
          />
        </div>
      </main>
    </div>
  );
}

export default App;