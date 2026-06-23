#!/usr/bin/env node

const { spawnSync } = require("child_process");
const path = require("path");
const fs = require("fs");

function getBinaryName() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "linux" && arch === "x64") {
    return "korlix-linux-x64";
  }

  if (platform === "win32" && arch === "x64") {
    return "korlix-win32-x64.exe";
  }

  console.error(`Unsupported platform: ${platform}-${arch}`);
  console.error("Supported platforms:");
  console.error("  linux-x64");
  console.error("  win32-x64");
  process.exit(1);
}

const binaryPath = path.join(__dirname, getBinaryName());

if (!fs.existsSync(binaryPath)) {
  console.error("Korlix binary not found.");
  console.error("Expected:", binaryPath);
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit"
});

process.exit(result.status ?? 1);
