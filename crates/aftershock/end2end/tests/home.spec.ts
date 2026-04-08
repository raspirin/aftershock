import { test, expect } from "@playwright/test";
import { SITE_TITLE, SEL } from "../helpers/constants";

test.describe("Home page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("header shows site title link with correct href", async ({ page }) => {
    const titleLink = page.locator(SEL.siteTitle);
    await expect(titleLink).toBeVisible();
    await expect(titleLink).toHaveText(SITE_TITLE);
    await expect(titleLink).toHaveAttribute("href", "/");
  });

  test("header has bottom border line", async ({ page }) => {
    await expect(page.locator(SEL.headerLine)).toBeVisible();
  });

  test("navigation has 主页 and 关于 links with correct hrefs", async ({ page }) => {
    const nav = page.locator(SEL.nav);
    const homeLink = nav.getByRole("link", { name: "主页" });
    const aboutLink = nav.getByRole("link", { name: "关于" });

    await expect(homeLink).toBeVisible();
    await expect(homeLink).toHaveAttribute("href", "/");

    await expect(aboutLink).toBeVisible();
    await expect(aboutLink).toHaveAttribute("href", "/about");
  });

  test("posts are grouped by year headings in descending order", async ({ page }) => {
    const yearHeadings = page.locator(SEL.yearHeading);
    const count = await yearHeadings.count();
    expect(count).toBeGreaterThan(0);

    const years: number[] = [];
    for (let i = 0; i < count; i++) {
      const text = await yearHeadings.nth(i).textContent();
      expect(text).toMatch(/^\d{4}$/);
      years.push(parseInt(text!, 10));
    }

    // Verify descending order
    for (let i = 1; i < years.length; i++) {
      expect(years[i]).toBeLessThanOrEqual(years[i - 1]);
    }
  });

  test("each post entry has time with datetime attr, title link, and tag links", async ({
    page,
  }) => {
    const firstSection = page.locator("section").first();

    // time element with machine-friendly datetime attribute
    const time = firstSection.locator("time").first();
    await expect(time).toBeVisible();
    await expect(time).toHaveAttribute("datetime", /.+/);
    // time text should show abbreviated month + day pattern (e.g. "Apr 07")
    await expect(time).toHaveText(/^[A-Z][a-z]{2}\s+\d{1,2}$/);

    // title link inside h2
    await expect(firstSection.locator("h2 a").first()).toBeVisible();

    // tag links with # prefix
    const firstTag = firstSection.locator(SEL.tagLink).first();
    await expect(firstTag).toBeVisible();
    const tagText = await firstTag.textContent();
    expect(tagText).toMatch(/^#.+/);
  });

  test("seeded posts are visible with clickable summaries", async ({ page }) => {
    await expect(page.getByText("E2E 测试文章：晨光")).toBeVisible();
    await expect(page.getByText("E2E 测试文章：暮色")).toBeVisible();

    // Summaries visible
    const summary1 = page.getByText("晨光测试摘要");
    const summary2 = page.getByText("暮色测试摘要");
    await expect(summary1).toBeVisible();
    await expect(summary2).toBeVisible();

    // Summaries are wrapped in <a> links pointing to the post
    const summaryLink1 = page.locator('a:has-text("晨光测试摘要")');
    await expect(summaryLink1).toHaveAttribute("href", /\/posts\/.+/);
  });

  test("seeded posts have e2e-test tag displayed as #e2e-test", async ({ page }) => {
    const tagLinks = page.locator('a[href="/tags/e2e-test"]');
    const count = await tagLinks.count();
    expect(count).toBeGreaterThanOrEqual(2); // both seeded posts

    // Verify # prefix
    await expect(tagLinks.first()).toHaveText("#e2e-test");
  });
});
