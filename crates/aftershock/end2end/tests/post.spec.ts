import { test, expect } from "@playwright/test";
import { SITE_TITLE, SEL, MSG } from "../helpers/constants";

const post1Uid = () => process.env.E2E_POST1_UID!;

test.describe("Post detail page", () => {
  test("navigate from home to post detail", async ({ page }) => {
    await page.goto("/");
    // Click the title link (inside h2) rather than any text match
    await page.locator('h2 a:has-text("E2E 测试文章：晨光")').click();
    await expect(page).toHaveURL(new RegExp(`/posts/${post1Uid()}`));
  });

  test("post page title contains site name and h1 shows post title", async ({
    page,
  }) => {
    await page.goto(`/posts/${post1Uid()}`);
    // <title> always contains the site name
    await expect(page).toHaveTitle(new RegExp(SITE_TITLE));
    // The article <h1> shows the post title
    await expect(page.locator("article h1").first()).toHaveText(
      "E2E 测试文章：晨光",
    );
  });

  test("article has time element with datetime attribute", async ({ page }) => {
    await page.goto(`/posts/${post1Uid()}`);

    const article = page.locator(SEL.article);
    const time = article.locator("time").first();
    await expect(time).toBeVisible();
    // datetime attribute should be ISO-8601-like
    await expect(time).toHaveAttribute("datetime", /.+/);
    // display text should be YYYY-MM-DD format
    await expect(time).toHaveText(/^\d{4}-\d{2}-\d{2}/);
  });

  test("article tags show with # prefix and link to archive", async ({
    page,
  }) => {
    await page.goto(`/posts/${post1Uid()}`);

    const article = page.locator(SEL.article);

    const e2eTag = article.locator('a[href="/tags/e2e-test"]');
    await expect(e2eTag).toBeVisible();
    await expect(e2eTag).toHaveText("#e2e-test");

    const morningTag = article.locator('a[href="/tags/晨光"]');
    await expect(morningTag).toBeVisible();
    await expect(morningTag).toHaveText("#晨光");
  });

  test("article prose renders markdown as HTML", async ({ page }) => {
    await page.goto(`/posts/${post1Uid()}`);

    const prose = page.locator(SEL.article).locator(SEL.prose);
    await expect(prose).toBeVisible();

    // The seed body has **晨光** which should render as <strong>
    await expect(prose.locator("strong")).toContainText("晨光");
    // Also has two paragraphs separated by blank line
    const paragraphs = prose.locator("p");
    expect(await paragraphs.count()).toBeGreaterThanOrEqual(2);
  });

  test("article ends with fin marker and CC license", async ({ page }) => {
    await page.goto(`/posts/${post1Uid()}`);

    const article = page.locator(SEL.article);

    // "fin" marker
    await expect(article.getByText("fin")).toBeVisible();

    // CC license link
    const license = article.locator(SEL.licenseLink);
    await expect(license).toBeVisible();
    await expect(license).toHaveAttribute(
      "href",
      "https://creativecommons.org/licenses/by-nc-sa/4.0/",
    );
    await expect(license).toHaveAttribute("target", "_blank");
    await expect(license).toHaveAttribute("rel", /noopener/);

    // 4 CC icons (cc, by, nc, sa)
    const icons = license.locator("img");
    expect(await icons.count()).toBe(4);
  });

  test("non-existent post shows error message in MessageBox", async ({
    page,
  }) => {
    await page.goto("/posts/non-existent-uid-12345");
    // Should show the specific error message from consts.rs
    await expect(page.locator(SEL.main)).toContainText(MSG.loadFailure);
    // Should NOT have an article
    await expect(page.locator(SEL.article)).toHaveCount(0);
    // Should have the MessageBox container
    await expect(page.locator(SEL.messageBox)).toBeVisible();
  });
});
