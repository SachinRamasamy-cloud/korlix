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

function runNodeScript(scriptPath, args, cwd = process.cwd()) {
  const result = spawnSync(process.execPath, [scriptPath, ...args], {
    cwd,
    stdio: "inherit",
    shell: false
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function runNpm(args, cwd = process.cwd()) {
  const npmCommand = process.platform === "win32" ? "npm.cmd" : "npm";

  const result = spawnSync(npmCommand, args, {
    cwd,
    stdio: "inherit",
    shell: process.platform === "win32"
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function writeJson(filePath, data) {
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2) + "\n");
}

function findKorlixCliScript() {
  const packageJsonPath = require.resolve("korlix/package.json", {
    paths: [path.resolve(__dirname, "..")]
  });

  return path.join(path.dirname(packageJsonPath), "bin", "korlix.js");
}

async function main() {
  console.log("");
  console.log("Korlix - frontend-first language");
  console.log("");

  let projectName = process.argv[2];

  if (!projectName) {
    projectName = await askProjectName();
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

  runNodeScript(korlixCliScript, ["new", projectName]);

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
      korlix: process.env.KORLIX_PACKAGE || "^0.1.1"
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