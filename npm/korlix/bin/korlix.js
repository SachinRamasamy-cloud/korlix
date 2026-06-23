#!/usr/bin/env node

const { spawnSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const binaryName = process.platform === "win32" ? "korlix.exe" : "korlix";
const binaryPath = path.join(__dirname, binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error("Korlix binary not found:");
  console.error(binaryPath);
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit"
});

process.exit(result.status ?? 1);
