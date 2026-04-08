import { test, expect } from "@playwright/test";
import { SITE_TITLE, SEL, MSG } from "../helpers/constants";

const post1Uid = () => process.env.E2E_POST1_UID!;

test.describe("Navigation", () => {
  test("full journey: home → post → site title → home → about → home", async ({
    page,
  }) => {
    // 1. Start at home
    await page.goto("/");
    await expect(page.locator(SEL.siteTitle)).toBeVisible();
    // Verify we are on home by checking year headings exist
    await expect(page.locator(SEL.yearHeading).first()).toBeVisible();

    // 2. Click a post title to go to post detail
    await page.locator('h2 a:has-text("E2E 测试文章：晨光")').click();
    await expect(page).toHaveURL(new RegExp(`/posts/`));
    await expect(page.locator(SEL.article)).toBeVisible();

    // 3. Click site title to go back to home
    await page.locator(SEL.siteTitle).click();
    await expect(page).toHaveURL(/\/$/);
    await expect(page.locator(SEL.yearHeading).first()).toBeVisible();

    // 4. Click "关于" to go to about page
    await page.locator(SEL.nav).getByRole("link", { name: "关于" }).click();
    await expect(page).toHaveURL(/\/about/);
    await expect(page.locator(SEL.prose)).toBeVisible();

    // 5. Click "主页" to go back home
    await page.locator(SEL.nav).getByRole("link", { name: "主页" }).click();
    await expect(page).toHaveURL(/\/$/);
    await expect(page.locator(SEL.yearHeading).first()).toBeVisible();
  });

  test("tag navigation: post → tag archive → another post", async ({
    page,
  }) => {
    // 1. Go to post detail
    await page.goto(`/posts/${post1Uid()}`);
    await expect(page.locator(SEL.article)).toBeVisible();

    // 2. Click the shared tag to go to archive
    await page
      .locator(SEL.article)
      .locator('a[href="/tags/e2e-test"]')
      .click();
    await expect(page).toHaveURL(/\/tags\/e2e-test/);
    // Verify archive heading
    await expect(page.locator(SEL.yearHeading).first()).toHaveText(
      "#e2e-test",
    );

    // 3. Click the other post from the archive
    await page.locator('h2 a:has-text("E2E 测试文章：暮色")').click();
    await expect(page).toHaveURL(/\/posts\//);
    await expect(page.locator("article h1").first()).toHaveText(
      "E2E 测试文章：暮色",
    );
  });

  test("404 route shows not-found error message", async ({ page }) => {
    await page.goto("/this-route-does-not-exist");
    // The fallback route uses MSG_DATA_NOT_FOUND
    await expect(page.locator(SEL.main)).toContainText(MSG.notFound);
    await expect(page.locator(SEL.messageBox)).toBeVisible();
  });

  test("header and nav persist across all pages", async ({ page }) => {
    // Check header/nav on home
    await page.goto("/");
    await expect(page.locator(SEL.header)).toBeVisible();
    await expect(page.locator(SEL.nav)).toBeVisible();

    // Check header/nav on post detail
    await page.goto(`/posts/${post1Uid()}`);
    await expect(page.locator(SEL.header)).toBeVisible();
    await expect(page.locator(SEL.nav)).toBeVisible();

    // Check header/nav on about
    await page.goto("/about");
    await expect(page.locator(SEL.header)).toBeVisible();
    await expect(page.locator(SEL.nav)).toBeVisible();

    // Check header/nav on tag archive
    await page.goto("/tags/e2e-test");
    await expect(page.locator(SEL.header)).toBeVisible();
    await expect(page.locator(SEL.nav)).toBeVisible();
  });
});
