import { test, expect } from '@playwright/test';

test.describe('Dashboard UI', () => {
  test.beforeEach(async ({ page }) => {
    // Mock the /api/config endpoint
    await page.route('**/api/config', (route) =>
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          page_title: 'Test Dashboard',
          gatus_url: 'http://mock',
          refresh_interval_ms: 60000,
        }),
      })
    );

    // Mock the /api/statuses endpoint with mixed status services
    await page.route('**/api/statuses', (route) =>
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { name: 'Web App', key: 'core_web', results: [{ success: true }] },
          { name: 'API', key: 'core_api', results: [{ success: true }] },
          { name: 'Database', key: 'core_db', results: [{ success: false }] },
          { name: 'Cache', key: 'core_cache', results: [{ success: true }] },
          { name: 'CDN', key: 'infra_cdn', results: [{ success: true }] },
          { name: 'DNS', key: 'infra_dns', results: [{ success: true }] },
          { name: 'Load Balancer', key: 'infra_lb', results: [{ success: false }] },
        ]),
      })
    );

    await page.goto('/');
    // Wait for hexagons to render
    await page.waitForSelector('.hexagon');
  });

  test('renders hexagons with clip-path polygon', async ({ page }) => {
    const hexagon = page.locator('.hexagon').first();
    const clipPath = await hexagon.evaluate(
      (el) => getComputedStyle(el).clipPath
    );
    // clip-path should be a polygon (browser may normalize the format)
    expect(clipPath).toContain('polygon');
  });

  test('applies green class for successful services', async ({ page }) => {
    const greenHexagons = page.locator('.hexagon-green');
    await expect(greenHexagons).toHaveCount(5);
  });

  test('applies red class for failing services', async ({ page }) => {
    const redHexagons = page.locator('.hexagon-red');
    await expect(redHexagons).toHaveCount(2);
  });

  test('displays service names inside hexagons', async ({ page }) => {
    const names = await page.locator('.hexagon-name').allTextContents();
    expect(names).toContain('Web App');
    expect(names).toContain('Database');
    expect(names).toContain('CDN');
  });

  test('groups services by prefix', async ({ page }) => {
    const groupNames = await page
      .locator('.group-section summary')
      .allTextContents();
    expect(groupNames).toContain('Core');
    expect(groupNames).toContain('Infra');
  });

  test('group sections are collapsible', async ({ page }) => {
    const details = page.locator('.group-section').first();
    await expect(details).toHaveAttribute('open', '');

    // Click summary to collapse
    await details.locator('summary').click();
    await expect(details).not.toHaveAttribute('open', '');

    // Click again to expand
    await details.locator('summary').click();
    await expect(details).toHaveAttribute('open', '');
  });

  test('hexagon grid uses centered rows', async ({ page }) => {
    const rows = page.locator('.hex-row');
    const count = await rows.count();
    expect(count).toBeGreaterThan(0);

    // Verify rows use flex centering
    const justifyContent = await rows
      .first()
      .evaluate((el) => getComputedStyle(el).justifyContent);
    expect(justifyContent).toBe('center');
  });

  test('offset rows have translateX transform', async ({ page }) => {
    const offsetRow = page.locator('.hex-row-offset').first();
    const count = await page.locator('.hex-row-offset').count();
    if (count > 0) {
      const transform = await offsetRow.evaluate(
        (el) => getComputedStyle(el).transform
      );
      // transform should be a matrix (translated)
      expect(transform).not.toBe('none');
    }
  });

  test('sets page title from config', async ({ page }) => {
    await expect(page).toHaveTitle('Test Dashboard');
  });

  test('shows last updated timestamp', async ({ page }) => {
    const lastUpdated = page.locator('#last-updated');
    await expect(lastUpdated).toContainText('Last updated:');
  });
});

test.describe('Dark mode glow effects', () => {
  test.beforeEach(async ({ page }) => {
    await page.emulateMedia({ colorScheme: 'dark' });

    await page.route('**/api/config', (route) =>
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          page_title: 'Test',
          gatus_url: 'http://mock',
          refresh_interval_ms: 60000,
        }),
      })
    );

    await page.route('**/api/statuses', (route) =>
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { name: 'Up', key: 'test_up', results: [{ success: true }] },
          { name: 'Down', key: 'test_down', results: [{ success: false }] },
        ]),
      })
    );

    await page.goto('/');
    await page.waitForSelector('.hexagon');
  });

  test('green hexagons have drop-shadow in dark mode', async ({ page }) => {
    const filter = await page
      .locator('.hexagon-green')
      .first()
      .evaluate((el) => getComputedStyle(el).filter);
    expect(filter).toContain('drop-shadow');
  });

  test('red hexagons have animation in dark mode', async ({ page }) => {
    const animation = await page
      .locator('.hexagon-red')
      .first()
      .evaluate((el) => getComputedStyle(el).animationName);
    expect(animation).toBe('pulse-red');
  });
});

test.describe('Responsive scaling', () => {
  test('hexagons scale with viewport', async ({ browser }) => {
    // Test at mobile size
    const mobilePage = await browser.newPage({
      viewport: { width: 375, height: 812 },
    });
    await mobilePage.route('**/api/config', (route) =>
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          page_title: 'Test',
          gatus_url: 'http://mock',
          refresh_interval_ms: 60000,
        }),
      })
    );
    await mobilePage.route('**/api/statuses', (route) =>
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { name: 'Svc', key: 'a_svc', results: [{ success: true }] },
        ]),
      })
    );
    await mobilePage.goto('/');
    await mobilePage.waitForSelector('.hexagon');
    const mobileWidth = await mobilePage
      .locator('.hexagon')
      .first()
      .evaluate((el) => el.getBoundingClientRect().width);

    // Test at desktop size
    const desktopPage = await browser.newPage({
      viewport: { width: 1920, height: 1080 },
    });
    await desktopPage.route('**/api/config', (route) =>
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          page_title: 'Test',
          gatus_url: 'http://mock',
          refresh_interval_ms: 60000,
        }),
      })
    );
    await desktopPage.route('**/api/statuses', (route) =>
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { name: 'Svc', key: 'a_svc', results: [{ success: true }] },
        ]),
      })
    );
    await desktopPage.goto('/');
    await desktopPage.waitForSelector('.hexagon');
    const desktopWidth = await desktopPage
      .locator('.hexagon')
      .first()
      .evaluate((el) => el.getBoundingClientRect().width);

    // Desktop hexagons should be larger than mobile
    expect(desktopWidth).toBeGreaterThan(mobileWidth);
    // Mobile should be at minimum (90px)
    expect(mobileWidth).toBeCloseTo(90, -1);

    await mobilePage.close();
    await desktopPage.close();
  });
});
