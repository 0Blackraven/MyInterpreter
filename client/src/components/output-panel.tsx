import { useEffect, useState } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { AlertCircle, CheckCircle } from "lucide-react";
import { cn } from "@/lib/utils";

interface OutputLine {
  type: "output" | "error";
  content: string;
}

interface OutputPanelProps {
  output: string;
  error: string | null;
  success: boolean;
}

export function OutputPanel({ output, error, success }: OutputPanelProps) {
  const [scrollHeight, setScrollHeight] = useState("0px");

  useEffect(() => {
    function updateHeight() {
      setScrollHeight(`${window.innerHeight - 120}px`);
    }
    updateHeight();
    window.addEventListener("resize", updateHeight);
    return () => window.removeEventListener("resize", updateHeight);
  }, []);

  const lines: OutputLine[] = [];

  if (output) {
    output.split("\n").forEach((line) => {
      if (line.trim()) {
        lines.push({ type: "output", content: line });
      }
    });
  }

  if (error) {
    lines.push({ type: "error", content: error });
  }

  return (
    <div className="flex flex-col h-full bg-[#1e1e1e]">
      <div className="flex items-center justify-between px-3 py-3 bg-[#2d2d30] border-b border-gray-700 shrink-0">
        <span className="text-base text-white font-medium">output.log</span>
        <div className="flex items-center gap-2">
          {success && lines.length > 0 && (
            <div className="flex items-center gap-1 text-green-400">
              <CheckCircle className="w-4 h-4" />
              Success
            </div>
          )}
          {error && (
            <div className="flex items-center gap-1 text-red-500">
              <AlertCircle className="w-4 h-4" />
              Error
            </div>
          )}
        </div>
      </div>
      <div
        className="flex-1 overflow-hidden"
        style={{ height: scrollHeight }}
      >
        <ScrollArea className="h-full w-full">
          <div className="p-4 font-mono text-base space-y-1">
            {lines.length === 0 ? (
              <div className="text-gray-500 italic text-base">
                // No output yet. Click "Run" to execute your code.
              </div>
            ) : (
              lines.map((line, i) => (
                <div
                  key={i}
                  className={cn(
                    "whitespace-pre-wrap break-words",
                    line.type === "error" && "text-red-400"
                  )}
                >
                  {line.type === "error" && (
                    <AlertCircle className="inline w-5 h-5 mr-1" />
                  )}
                  {line.content}
                </div>
              ))
            )}
          </div>
        </ScrollArea>
      </div>
    </div>
  );
}

export default OutputPanel;