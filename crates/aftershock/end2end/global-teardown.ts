import { cleanup } from "./helpers/seed";

export default async function globalTeardown() {
  console.log("[global-teardown] Cleaning up test data…");
  await cleanup();
  console.log("[global-teardown] Done.");
}
