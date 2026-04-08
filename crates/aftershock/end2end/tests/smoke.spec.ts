import { test, expect } from "@playwright/test";
import { SITE_TITLE, SEL } from "../helpers/constants";

test.describe("Smoke tests", () => {
  test("homepage returns 200 and has correct lang attribute", async ({ page }) => {
    const response = await page.goto("/");
    expect(response?.status()).toBe(200);
    await expect(page.locator("html")).toHaveAttribute("lang", "zh-CN");
  });

  test("page title contains site name", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle(SITE_TITLE);
  });

  test("meta generator tag present", async ({ page }) => {
    await page.goto("/");
    const generator = page.locator('meta[name="generator"]');
    await expect(generator).toHaveAttribute("content", /^aftershock v/);
  });

  test("meta charset and viewport present", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator('meta[charset="utf-8"]')).toHaveCount(1);
    await expect(
      page.locator('meta[name="viewport"]'),
    ).toHaveAttribute("content", /width=device-width/);
  });

  test("stylesheet loaded from /pkg/aftershock.css", async ({ page }) => {
    await page.goto("/");
    const stylesheet = page.locator('link[rel="stylesheet"][href="/pkg/aftershock.css"]');
    await expect(stylesheet).toHaveCount(1);
  });

  test("no broken CSS or WASM resources", async ({ page }) => {
    const errors: string[] = [];

    page.on("response", (response) => {
      const url = response.url();
      const status = response.status();
      if (
        (url.endsWith(".css") || url.endsWith(".wasm") || url.endsWith(".js")) &&
        status >= 400
      ) {
        errors.push(`${status} ${url}`);
      }
    });

    await page.goto("/", { waitUntil: "networkidle" });
    expect(errors).toEqual([]);
  });
});
