import { waitForServer } from "./helpers/api";
import { seedTestData, cleanup } from "./helpers/seed";
import { STORAGE_URL, FRONTEND_URL } from "./helpers/constants";

export default async function globalSetup() {
  console.log("[global-setup] Waiting for storage server…");
  await waitForServer(`${STORAGE_URL}/api/v1/posts`, 30_000);
  console.log("[global-setup] Storage server is ready.");

  console.log("[global-setup] Cleaning up previous test data…");
  await cleanup();

  console.log("[global-setup] Seeding test data…");
  const data = await seedTestData();

  // Pass seed UIDs to tests via env vars
  process.env.E2E_POST1_UID = data.post1Uid;
  process.env.E2E_POST2_UID = data.post2Uid;
  process.env.E2E_ABOUT_UID = data.aboutUid;

  console.log("[global-setup] Waiting for frontend server…");
  await waitForServer(FRONTEND_URL, 60_000);
  console.log("[global-setup] Frontend server is ready. Setup complete.");
}
