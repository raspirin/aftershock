import { test, expect } from "@playwright/test";
import { SEL, MSG } from "../helpers/constants";

test.describe("About page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/about");
  });

  test("about page displays prose content from seeded page", async ({ page }) => {
    const prose = page.locator(SEL.prose);
    await expect(prose).toBeVisible();
    await expect(prose).toContainText("关于页面的内容");
    await expect(prose).toContainText("E2E 测试专用");
  });

  test("about page wraps content in serif font container", async ({ page }) => {
    // ContentSerif wrapper: div with font-af-serif class
    const serifWrapper = page.locator("div.font-af-serif.font-medium");
    await expect(serifWrapper).toBeVisible();
    // prose should be inside serif wrapper
    await expect(serifWrapper.locator(SEL.prose)).toBeVisible();
  });

  test("about page has no article tag and no fin/license", async ({ page }) => {
    await expect(page.locator(SEL.article)).toHaveCount(0);
    await expect(page.locator(SEL.licenseLink)).toHaveCount(0);
  });

  test("about page still shows header and navigation", async ({ page }) => {
    await expect(page.locator(SEL.header)).toBeVisible();
    await expect(page.locator(SEL.nav)).toBeVisible();
  });
});
