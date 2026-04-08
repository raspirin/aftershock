import { createPost, createPage, deletePost, deletePage, type Post } from "./api";

/** UIDs of everything we seed — used for cleanup. */
export interface SeedData {
  post1Uid: string;
  post2Uid: string;
  aboutUid: string;
}

let seeded: SeedData | null = null;

export async function seedTestData(): Promise<SeedData> {
  const post1 = await createPost({
    title: "E2E 测试文章：晨光",
    kind: "post",
    body: "<p>这是 <strong>晨光</strong> 的正文内容。用于端到端自动测试。</p><p>第二段落。</p>",
    tags: ["e2e-test", "晨光"],
    published: true,
    summary: "晨光测试摘要",
  });

  const post2 = await createPost({
    title: "E2E 测试文章：暮色",
    kind: "post",
    body: "<p>这是 <strong>暮色</strong> 的正文内容。用于端到端自动测试。</p><p>第二段落。</p>",
    tags: ["e2e-test", "暮色"],
    published: true,
    summary: "暮色测试摘要",
  });

  const about = await createPage({
    title: "about",
    kind: "page",
    body: "<p>这是关于页面的内容。E2E 测试专用。</p>",
    tags: [],
    published: true,
  });

  seeded = {
    post1Uid: post1.uid,
    post2Uid: post2.uid,
    aboutUid: about.uid,
  };

  console.log("[seed] Created test data:", seeded);
  return seeded;
}

export async function cleanup(): Promise<void> {
  if (seeded) {
    console.log("[seed] Cleaning up seeded data:", seeded);
    await deletePost(seeded.post1Uid);
    await deletePost(seeded.post2Uid);
    await deletePage(seeded.aboutUid);
    seeded = null;
  }

  // Also try to clean up by well-known uid in case previous run left data
  await deletePage("about");
}

export function getSeedData(): SeedData | null {
  return seeded;
}
