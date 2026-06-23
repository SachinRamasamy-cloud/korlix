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

function run(command, args, cwd = process.cwd()) {
  const result = spawnSync(command, args, {
    cwd,
    stdio: "inherit",
    shell: process.platform === "win32"
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function korlixCommand() {
  const localCli = path.resolve(__dirname, "../../korlix/bin/korlix.js");
  if (fs.existsSync(localCli)) {
    return {
      command: process.execPath,
      args: [localCli]
    };
  }

  try {
    return {
      command: process.execPath,
      args: [require.resolve("korlix/bin/korlix.js")]
    };
  } catch {
    return { command: "korlix", args: [] };
  }
}

function korlixPackageSpec() {
  if (process.env.KORLIX_PACKAGE) {
    return process.env.KORLIX_PACKAGE;
  }

  const localPackage = path.resolve(__dirname, "../../korlix");
  if (fs.existsSync(path.join(localPackage, "package.json"))) {
    return `file:${localPackage}`;
  }

  return "^0.1.0";
}

function writeJson(filePath, data) {
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2) + "\n");
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

  const korlix = korlixCommand();
  run(korlix.command, [...korlix.args, "new", projectName]);

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
      korlix: korlixPackageSpec()
    };

    writeJson(packagePath, pkg);
  }

  console.log("");
  console.log("Korlix app created");
  console.log("");
  console.log("Next steps:");
  console.log(`  cd ${projectName}`);
  console.log("  npm install");
  console.log("  npm run dev");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
