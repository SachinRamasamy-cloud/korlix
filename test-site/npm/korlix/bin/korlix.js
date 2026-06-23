#!/usr/bin/env node

const { spawnSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const platform = process.platform;
const arch = process.arch;

let binaryName = "korlix";

if (platform === "win32") {
  binaryName = "korlix.exe";
}

const binaryPath = path.join(__dirname, binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error("Korlix binary not found.");
  console.error("Expected:", binaryPath);
  console.error("");
  console.error("This npm package must include the compiled Korlix binary.");
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit"
});

process.exit(result.status ?? 1);

