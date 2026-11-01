import { useState, useEffect, useRef } from "react";
import Editor from "@monaco-editor/react";
import { Button } from "@/components/ui/button";
import { Play, RotateCcw } from "lucide-react";

interface CodeEditorProps {
  onRun: (code: string) => void;
  isRunning: boolean;
}

const DEFAULT_CODE = `// Lox Interpreter
let x = 10;
let y = 20;
print x + y;

let a = 5;
let b = 3;
print a * b;
print a / b;

if (x > y) {
    print "x is greater than y";
} else {
    print "y is greater than or equal to x";
}

let i = 0;
while (i < 3) {
    print i;
    i = i + 1;
}

fn greet(name) {
    print "Hello, " + name;
}
greet("World");
`;

export function CodeEditor({ onRun, isRunning }: CodeEditorProps) {
  const [code, setCode] = useState(DEFAULT_CODE);
  const [editorWidth, setEditorWidth] = useState(0);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function updateSize() {
      if (containerRef.current) {
        setEditorWidth(containerRef.current.offsetWidth);
      }
    }
    updateSize();
    window.addEventListener("resize", updateSize);
    return () => window.removeEventListener("resize", updateSize);
  }, []);

  function handleRun() {
    onRun(code);
  }

  function handleReset() {
    setCode(DEFAULT_CODE);
  }

  return (
    <div ref={containerRef} className="flex flex-col h-full bg-[#1e1e1e]">
      <div className="flex items-center justify-between px-3 py-3 bg-[#2d2d30] border-b border-gray-700 shrink-0">
        <span className="text-base text-white font-medium">editor.lox</span>
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handleReset}
            disabled={isRunning}
            className="bg-gray-700 hover:bg-gray-600 text-white border-gray-600 h-8 text-sm"
          >
            <RotateCcw className="w-4 h-4 mr-1" />
            Reset
          </Button>
          <Button
            variant="default"
            size="sm"
            onClick={handleRun}
            disabled={isRunning}
            className="bg-blue-600 hover:bg-blue-500 h-8 text-sm"
          >
            <Play className="w-4 h-4 mr-1" />
            {isRunning ? "Running..." : "Run"}
          </Button>
        </div>
      </div>
      <div className="flex-1 overflow-hidden">
        <Editor
          height="100%"
          width={editorWidth}
          defaultLanguage="lox"
          theme="vs-dark"
          value={code}
          onChange={(value) => setCode(value || "")}
          beforeMount={(monaco) => {
            monaco.languages.register({ id: "lox" });

            monaco.languages.setMonarchTokensProvider("lox", {
              tokenizer: {
                root: [
                  [/[ \t]*/, ""],
                  [/\/\/.*/, "comment"],
                  [/"([^"\\]|\\.)*"/, "string"],
                  [/\b(let|fn|class|if|else|while|for|return|print|true|false|null|this|super|and|or)\b/, "keyword"],
                  [/\b(\d+)\b/, "number"],
                  [/[a-zA-Z_]\w*/, "identifier"],
                  [/[+\-*/%=<>!]+/, "operator"],
                  [/[{}()[\];,.]/, "delimiter"],
                ],
              },
            });

            monaco.editor.defineTheme("lox-theme", {
              base: "vs-dark",
              inherit: true,
              rules: [
                { token: "comment", foreground: "6A9955" },
                { token: "keyword", foreground: "C586C0" },
                { token: "string", foreground: "CE9178" },
                { token: "number", foreground: "B5CEA8" },
                { token: "identifier", foreground: "9CDCFE" },
                { token: "operator", foreground: "D4D4D4" },
              ],
              colors: {
                "editor.background": "#1e1e1e",
                "editor.foreground": "#ffffff",
              },
            });
          }}
          options={{
            minimap: { enabled: false },
            fontSize: 18,
            wordWrap: "on",
            scrollBeyondLastLine: false,
            automaticLayout: true,
            tabSize: 2,
            insertSpaces: true,
            quickSuggestions: false,
            parameterHints: { enabled: false },
            folding: false,
            lineNumbers: "on",
          }}
        />
      </div>
    </div>
  );
}

export default CodeEditor;