import { STORAGE_URL } from "./constants";

export interface NewPost {
  title: string;
  kind: "post" | "page";
  body: string;
  tags: string[];
  published: boolean;
  summary?: string;
}

export interface Post {
  uid: string;
  kind: string;
  created_at: number;
  updated_at: number;
  title: string;
  tags: string[];
  body: string;
  summary: string | null;
  published: boolean;
}

async function request<T>(
  path: string,
  init?: RequestInit,
): Promise<T> {
  const res = await fetch(`${STORAGE_URL}${path}`, {
    headers: { "Content-Type": "application/json" },
    ...init,
  });
  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new Error(
      `API ${init?.method ?? "GET"} ${path} → ${res.status}: ${text}`,
    );
  }
  return res.json() as Promise<T>;
}

export function createPost(data: NewPost): Promise<Post> {
  return request<Post>("/api/v1/posts", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

export function createPage(data: NewPost): Promise<Post> {
  return request<Post>("/api/v1/pages", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

export async function deletePost(uid: string): Promise<void> {
  try {
    await request(`/api/v1/posts/uid/${uid}`, { method: "DELETE" });
  } catch {
    // ignore – may already be deleted
  }
}

export async function deletePage(uid: string): Promise<void> {
  try {
    await request(`/api/v1/pages/uid/${uid}`, { method: "DELETE" });
  } catch {
    // ignore – may already be deleted
  }
}

/**
 * Poll a URL until it responds with a 2xx, or time out.
 */
export async function waitForServer(
  url: string,
  timeoutMs = 30_000,
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const res = await fetch(url, { method: "GET" });
      if (res.ok) return;
    } catch {
      // not ready yet
    }
    await new Promise((r) => setTimeout(r, 500));
  }
  throw new Error(`Server at ${url} did not become ready within ${timeoutMs}ms`);
}
