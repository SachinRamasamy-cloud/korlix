#!/usr/bin/env node

const readline = require("readline");
const { spawnSync } = require("child_process");
const path = require("path");
const fs = require("fs");

function askProjectName(defaultValue = "my-korlix-app") {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  return new Promise((resolve) => {
    rl.question(`Project name (${defaultValue}): `, (answer) => {
      rl.close();
      resolve(answer.trim() || defaultValue);
    });
  });
}

function printUsage() {
  console.error("");
  console.error("Usage:");
  console.error("  npm create korlix@latest");
  console.error("  npm create korlix@latest my-app");
  console.error("  npx --prefer-online create-korlix@latest my-app");
  console.error("");
}

function validateProjectName(projectName) {
  if (!projectName) {
    return;
  }

  if (projectName.startsWith("-")) {
    console.error("");
    console.error(`Error: invalid project name: ${projectName}`);
    console.error("");
    console.error("This looks like an npm/npx option, not a Korlix project name.");
    console.error("");
    console.error("Correct command:");
    console.error("  npx --prefer-online create-korlix@latest my-app");
    console.error("");
    console.error("Wrong command:");
    console.error("  npx create-korlix@latest --prefer-online");
    printUsage();
    process.exit(1);
  }
}

function runNodeScript(scriptPath, args, cwd = process.cwd()) {
  const result = spawnSync(process.execPath, [scriptPath, ...args], {
    cwd,
    stdio: "inherit",
    shell: false,
    windowsHide: true
  });

  if (result.error) {
    console.error(result.error);
    process.exit(1);
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function runNpm(args, cwd = process.cwd()) {
  let command;
  let finalArgs;

  const npmExecPath = process.env.npm_execpath;

  // Best method:
  // npm/npx gives us npm's JS file path.
  // Run that through Node instead of directly spawning npm.cmd.
  if (
    npmExecPath &&
    fs.existsSync(npmExecPath) &&
    !npmExecPath.endsWith(".cmd") &&
    !npmExecPath.endsWith(".bat")
  ) {
    command = process.execPath;
    finalArgs = [npmExecPath, ...args];
  } else if (process.platform === "win32") {
    // Windows fallback.
    // Do not spawn npm.cmd directly.
    command = process.env.ComSpec || "cmd.exe";
    finalArgs = ["/d", "/s", "/c", "npm", ...args];
  } else {
    // Linux/macOS.
    command = "npm";
    finalArgs = args;
  }

  const result = spawnSync(command, finalArgs, {
    cwd,
    stdio: "inherit",
    shell: false,
    windowsHide: true,
    env: process.env
  });

  if (result.error) {
    console.error(result.error);
    process.exit(1);
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function writeJson(filePath, data) {
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2) + "\n");
}

function findKorlixCliScript() {
  return require.resolve("korlix/bin/korlix.js", {
    paths: [__dirname, process.cwd()]
  });
}

async function main() {
  console.log("");
  console.log("Korlix - frontend-first language");
  console.log("");

  let projectName = process.argv[2];

  validateProjectName(projectName);

  if (!projectName) {
    projectName = await askProjectName();
    validateProjectName(projectName);
  }

  const projectPath = path.resolve(process.cwd(), projectName);

  if (fs.existsSync(projectPath)) {
    console.error("");
    console.error(`Error: folder already exists: ${projectName}`);
    process.exit(1);
  }

  console.log(`Creating Korlix app: ${projectName}`);
  console.log("");

  const korlixCliScript = findKorlixCliScript();

  runNodeScript(korlixCliScript, ["new", projectName], process.cwd());

  const configPath = path.join(projectPath, "korlix.config.json");

  if (fs.existsSync(configPath)) {
    const config = JSON.parse(fs.readFileSync(configPath, "utf8"));
    config.mode = "spa";
    writeJson(configPath, config);
  }

  const packagePath = path.join(projectPath, "package.json");

  if (fs.existsSync(packagePath)) {
    const pkg = JSON.parse(fs.readFileSync(packagePath, "utf8"));

    pkg.scripts = {
      dev: "korlix dev",
      build: "korlix build",
      preview: "korlix preview",
      check: "korlix check"
    };

    pkg.devDependencies = {
      ...(pkg.devDependencies || {}),
      korlix: "0.1.1"
    };

    writeJson(packagePath, pkg);
  }

  console.log("");
  console.log("Korlix app created");
  console.log("");
  console.log("Installing dependencies...");
  console.log("");

  runNpm(["install"], projectPath);

  console.log("");
  console.log("Starting dev server...");
  console.log("");

  runNpm(["run", "dev"], projectPath);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});