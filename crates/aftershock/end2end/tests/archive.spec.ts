import { test, expect } from "@playwright/test";
import { SEL, MSG } from "../helpers/constants";

test.describe("Archive / Tag pages", () => {
  test("clicking a tag navigates to tag archive with # heading", async ({
    page,
  }) => {
    await page.goto("/");
    await page.locator('a[href="/tags/e2e-test"]').first().click();
    await expect(page).toHaveURL(/\/tags\/e2e-test/);

    // The section heading should be "#e2e-test" (tag with # prefix)
    const heading = page.locator(SEL.yearHeading).first();
    await expect(heading).toHaveText("#e2e-test");
  });

  test("shared tag shows both seeded posts", async ({ page }) => {
    await page.goto("/tags/e2e-test");

    await expect(page.getByText("E2E 测试文章：晨光")).toBeVisible();
    await expect(page.getByText("E2E 测试文章：暮色")).toBeVisible();
  });

  test("unique tag shows only its post", async ({ page }) => {
    await page.goto("/tags/晨光");
    await expect(page.getByText("E2E 测试文章：晨光")).toBeVisible();
    await expect(page.getByText("E2E 测试文章：暮色")).not.toBeVisible();
  });

  test("archive page does NOT show summaries (unlike home page)", async ({
    page,
  }) => {
    await page.goto("/tags/e2e-test");
    // Summaries should NOT appear on archive pages (with_summary=false)
    await expect(page.getByText("晨光测试摘要")).not.toBeVisible();
    await expect(page.getByText("暮色测试摘要")).not.toBeVisible();
  });

  test("archive posts have time, title link, and tags", async ({ page }) => {
    await page.goto("/tags/e2e-test");

    // time element with datetime attr and "Mon DD" format
    const time = page.locator("section time").first();
    await expect(time).toBeVisible();
    await expect(time).toHaveAttribute("datetime", /.+/);
    await expect(time).toHaveText(/^[A-Z][a-z]{2}\s+\d{1,2}$/);

    // title link
    const titleLink = page.locator("section h2 a").first();
    await expect(titleLink).toBeVisible();
    await expect(titleLink).toHaveAttribute("href", /^\/posts\//);

    // tag with # prefix
    const tag = page.locator(SEL.tagLink).first();
    const tagText = await tag.textContent();
    expect(tagText).toMatch(/^#/);
  });

  test("archive page uses flat list (single section, no year grouping)", async ({
    page,
  }) => {
    await page.goto("/tags/e2e-test");
    // Should have exactly one section (all posts under the tag heading)
    const sections = page.locator("section");
    expect(await sections.count()).toBe(1);
  });

  test("non-existent tag shows placeholder or empty state", async ({
    page,
  }) => {
    await page.goto("/tags/non-existent-tag-xyz");
    const main = page.locator(SEL.main);
    await expect(main).toBeVisible();
    // When tag has no posts, the archive page shows the placeholder message
    // or loads an empty result — we just verify something meaningful is shown
    const text = await main.textContent();
    expect(text!.length).toBeGreaterThan(0);
  });
});
