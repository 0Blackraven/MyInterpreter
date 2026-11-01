import express from "express";
import cors from "cors";
import { spawn } from "child_process";
import path from "path";
import { fileURLToPath } from "url";
import dotenv from "dotenv";

dotenv.config();
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const app = express();
const PORT = 3001;
let url = "";

if (process.env.URL == undefined) {
  url = "*";
}else{
  url = process.env.URL;
}

app.use(cors({
  origin: url,
  methods: ["GET", "POST"],
}));
app.use(express.json());

const INTERPRETER_PATH = path.resolve(
  __dirname,
  "..",
  "..",
  "interpreter",
  "target",
  "release",
  "interpreter.exe"
);

app.post("/api/run", (req, res) => {
  const { code } = req.body;

  if (!code || typeof code !== "string") {
    res.status(400).json({ error: "Code is required" });
    return;
  }

  let child;
  try {
    child = spawn(INTERPRETER_PATH, [], { stdio: ["pipe", "pipe", "pipe"] });
  } catch (err) {
    res.status(500).json({ error: "Failed to spawn interpreter" });
    return;
  }

  let stdout = "";
  let stderr = "";

  child.stdout.on("data", (data: Buffer) => {
    stdout += data.toString();
  });

  child.stderr.on("data", (data: Buffer) => {
    stderr += data.toString();
  });

  child.on("close", (code) => {
    if (stderr) {
      console.error("Interpreter stderr:", stderr);
    }
    try {
      const result = JSON.parse(stdout.trim());
      res.json(result);
    } catch {
      res.status(500).json({
        success: false,
        output: stdout,
        error: stderr || "Failed to parse interpreter output",
      });
    }
  });

  child.on("error", (err) => {
    res.status(500).json({ error: err.message });
  });

  child.stdin.write(code);
  child.stdin.end();
});

app.get("/health", (_req, res) => {
  res.json({ status: "ok" });
});

app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});