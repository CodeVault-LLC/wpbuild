#!/usr/bin/env node

import { runWpBuild } from "./index.js";

async function run() {
  const processResult = await runWpBuild();

  process.exit(processResult.exitCode ?? 0);
}

void run();
