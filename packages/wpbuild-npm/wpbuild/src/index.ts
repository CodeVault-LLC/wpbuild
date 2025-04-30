import {
  execa,
  type Options as ExecaOptions,
  type ExecaReturnValue,
} from "execa";
import { fileURLToPath } from "node:url";
import { getExePath } from "./getExePath.js";

/**
 * Runs `wpbuild` with the provided options as a JavaScript object.
 *
 * @param options - The options to pass to `wpbuild`.
 * These get transformed into an array strings.
 * - Values that are `true` will be passed as flags (`--flag`).
 * - Values that are `false` or `null` will be ignored.
 * - All other values will be passed as options (`--option value`).
 *
 * @param execaOptions - Options to pass to {@link execa}.
 */
export async function runWpBuild(
  execaOptions?: ExecaOptions
): Promise<ExecaReturnValue<string>>;
/**
 * Runs the `wpbuild` with the provided arguments.
 *
 * @param args - The arguments to pass to `wpbuild`.
 * These should be in an array of string format.
 * Every option and their value should be its own entry in the array.
 *
 * @param execaOptions - Options to pass to {@link execa}.
 *
 * @returns A promise that resolves when the `wpbuild` has finished running.
 *
 * @example
 * Options with values
 * ```typescript
 * await runWpBuild(["--tag", "1.0.0", "--config", "github"]);
 * ```
 *
 * @example
 * Boolean flags
 * ```typescript
 * await runWpBuild(["--unreleased", "--topo-order"]);
 * ```
 *
 * @example
 * Combining options and flags
 * ```typescript
 * await runWpBuild(["--tag", "1.0.0", "--config", "github", "--topo-order"]);
 * ```
 */
export async function runWpBuild(
  execaOptions?: ExecaOptions
): Promise<ExecaReturnValue<string>>;
export async function runWpBuild(
  execaOptions?: ExecaOptions
): Promise<ExecaReturnValue<string>> {
  const exePath = await getExePath();

  return execa(fileURLToPath(exePath), [], {
    stdio: "inherit",
    ...execaOptions,
  });
}
