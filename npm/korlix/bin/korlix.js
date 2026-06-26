#!/usr/bin/env node

const { spawnSync } = require("child_process");
const path = require("path");
const fs = require("fs");

function getBinaryName() {
  if (process.platform === "linux" && process.arch === "x64") {
    return "korlix-linux-x64";
  }

  if (process.platform === "win32" && process.arch === "x64") {
    return "korlix-win32-x64.exe";
  }

  console.error(`Unsupported platform: ${process.platform}-${process.arch}`);
  console.error("Supported platforms: linux-x64, win32-x64");
  process.exit(1);
}

const binaryPath = path.join(__dirname, getBinaryName());

if (!fs.existsSync(binaryPath)) {
  console.error("Korlix binary not found.");
  console.error("Expected:", binaryPath);
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
  shell: false
});

process.exit(result.status ?? 1);
