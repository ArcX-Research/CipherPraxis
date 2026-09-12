#!/usr/bin/env node
// Headless startup regression checks against dist/, served with production cache headers.
// Requires Playwright; PLAYWRIGHT_MODULE and CHROME_BIN can point to an external installation.
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const http = require('node:http');
const path = require('node:path');
const { chromium, devices } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');

async function main() {
  const dist = process.argv[2] ? path.resolve(process.argv[2]) : path.resolve(__dirname, '../dist');
  const html = await fs.readFile(path.join(dist, 'index.html'), 'utf8');
  const asset = (suffix) => html.match(new RegExp(`/pkg/[\\w.]+${suffix}`))[0];
  const jsUrl = asset('\\.js');
  const wasmUrl = asset('\\.wasm');
  const cssUrl = asset('\\.css');
  const wasm = await fs.readFile(path.join(dist, wasmUrl));
  const js = await fs.readFile(path.join(dist, jsUrl), 'utf8');
  const css = await fs.readFile(path.join(dist, cssUrl));
  const hits = [];
  let legacy = false;

  // A previous release with the old, mutable URLs. An inert WASM custom section makes
  // its bytes different while keeping a valid app for priming the browser's real cache.
  const legacyWasm = Buffer.concat([wasm, Buffer.from([0, 4, 3, 111, 108, 100])]);
  const legacyJs = js.replaceAll(path.basename(wasmUrl), 'praxis_bg.wasm');
  const legacyHtml = html.replaceAll(jsUrl, '/pkg/praxis.js?v=old')
    .replaceAll(wasmUrl, '/pkg/praxis_bg.wasm').replaceAll(cssUrl, '/styles.css');

  const server = http.createServer(async (req, res) => {
    const pathname = new URL(req.url, 'http://localhost').pathname;
    hits.push(pathname);
    try {
      let body;
      let type = 'text/html; charset=utf-8';
      if (pathname.startsWith('/pkg/') || pathname === '/styles.css') {
        if (legacy && pathname === '/pkg/praxis_bg.wasm') body = legacyWasm;
        else if (legacy && pathname === '/pkg/praxis.js') body = legacyJs;
        else if (legacy && pathname === '/styles.css') body = css;
        else body = await fs.readFile(path.join(dist, pathname));
        type = pathname.endsWith('.wasm') ? 'application/wasm'
          : pathname.endsWith('.css') ? 'text/css' : 'text/javascript';
      } else if (path.extname(pathname)) {
        body = await fs.readFile(path.join(dist, pathname));
        type = pathname.endsWith('.svg') ? 'image/svg+xml' : 'application/octet-stream';
      } else {
        body = legacy ? legacyHtml : html;
      }
      res.writeHead(200, {
        'Content-Type': type,
        'Content-Length': Buffer.byteLength(body),
        'Cache-Control': pathname.startsWith('/pkg/')
          ? 'public, max-age=31536000, immutable' : 'no-cache',
        'X-Content-Type-Options': 'nosniff',
      });
      res.end(body);
    } catch {
      res.writeHead(404, { 'Content-Type': 'text/plain', 'Cache-Control': 'no-store' });
      res.end('Not found');
    }
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const base = `http://127.0.0.1:${server.address().port}`;
  let browser;
  let checks = 0;

  async function ready(page) {
    await page.waitForFunction(() => !document.getElementById('boot')
      && document.querySelector('#main h1')?.textContent.trim(), null, { timeout: 30000 });
    assert.equal(await page.locator('#main h1').isVisible(), true);
  }

  async function check(label, options, run) {
    const context = await browser.newContext(options);
    try {
      await run(await context.newPage());
      checks += 1;
      console.log(`✓ ${label}`);
    } finally {
      await context.close();
      legacy = false;
    }
  }

  try {
    browser = await chromium.launch({ headless: true, executablePath: process.env.CHROME_BIN });
    const mobile = devices['Pixel 7'];
    const desktop = { viewport: { width: 1440, height: 1000 } };
    for (const [label, options] of [['mobile', mobile], ['desktop', desktop]]) {
      await check(`${label}: direct loads, navigation, history and cached reloads`, options, async page => {
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        const routes = ['/', '/ciphers', '/ciphers/vigenere', '/labs/affine-lab', '/find?q=vigenere', '/missing/route/extra'];
        for (const route of routes) {
          await page.goto(base + route);
          await ready(page);
          assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
          await page.reload();
          await ready(page);
        }
        await page.goto(base);
        await ready(page);
        await page.locator('.hero-actions a[href="/labs"]').click();
        await page.waitForURL(base + '/labs');
        await page.locator('#main h1').filter({ hasText: 'Interactive Labs' }).waitFor();
        await page.goBack();
        await page.locator('#main #welcome-h').waitFor();
        await page.goForward();
        await page.locator('#main h1').filter({ hasText: 'Interactive Labs' }).waitFor();
        assert.deepEqual(errors, []);
      });

      await check(`${label}: update with an older WASM already cached`, options, async page => {
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        legacy = true;
        await page.goto(base);
        await ready(page);
        let start = hits.length;
        await page.goto(base + '/ciphers');
        await ready(page);
        assert.equal(hits.slice(start).includes('/pkg/praxis_bg.wasm'), false, 'legacy WASM must be cached');
        legacy = false;
        start = hits.length;
        await page.goto(base);
        await ready(page);
        assert.ok(hits.slice(start).includes(wasmUrl), 'updated page must request its fingerprinted WASM');
        assert.ok(hits.slice(start).includes(jsUrl), 'updated page must request its matching loader');
        const loaded = await page.evaluate(() => performance.getEntriesByType('resource').map(r => r.name));
        assert.equal(loaded.some(url => url.endsWith('/pkg/praxis_bg.wasm')), false);
        start = hits.length;
        await page.goto(base + '/ciphers');
        await ready(page);
        assert.equal(hits.slice(start).includes(wasmUrl), false, 'new WASM should also use the browser cache');
        assert.deepEqual(errors, []);
      });
    }

    for (const [label, url, response] of [
      ['failed JavaScript download', jsUrl, null],
      ['failed WASM download', wasmUrl, null],
      ['invalid cached WASM response', wasmUrl, { body: 'invalid WASM', contentType: 'application/wasm' }],
    ]) {
      await check(`${label}: visible error and working retry`, mobile, async page => {
        let failing = true;
        await page.route(base + url, route => {
          if (!failing) return route.continue();
          return response ? route.fulfill(response) : route.abort('failed');
        });
        await page.goto(base);
        await page.locator('.boot-failed .boot-retry').waitFor();
        assert.equal(await page.locator('.boot-text').textContent(), 'Cipher Praxis could not load');
        failing = false;
        await page.locator('.boot-retry').click();
        await ready(page);
      });
    }

    await check('asynchronous route failure retains the recovery screen', mobile, async page => {
      await page.addInitScript(() => {
        const create = Document.prototype.createElement;
        Document.prototype.createElement = function (name, ...args) {
          if (name === 'section' && document.title.startsWith('Overview')) {
            throw new Error('Simulated routed view failure');
          }
          return create.call(this, name, ...args);
        };
      });
      await page.goto(base);
      await page.locator('.boot-failed .boot-retry').waitFor();
      assert.equal(await page.locator('.site-header').count(), 1);
      assert.equal(await page.locator('.site-footer').count(), 1);
      assert.equal(await page.locator('#main h1').count(), 0);
      assert.equal(await page.locator('#boot').isVisible(), true);
    });

    await check('slow download offers retry and still completes when data arrives', mobile, async page => {
      let release;
      const gate = new Promise(resolve => { release = resolve; });
      await page.route(base + wasmUrl, async route => {
        await gate;
        await route.continue();
      });
      try {
        await page.goto(base, { waitUntil: 'domcontentloaded' });
        await page.locator('.boot-retry').waitFor({ timeout: 20000 });
        assert.equal(await page.locator('.boot-text').textContent(), 'Still loading Cipher Praxis…');
        release();
        await ready(page);
      } finally {
        release();
      }
    });

    await check('CPU-throttled mobile startup and repeated reloads', mobile, async page => {
      const session = await page.context().newCDPSession(page);
      await session.send('Emulation.setCPUThrottlingRate', { rate: 6 });
      for (let i = 0; i < 3; i++) {
        await page.goto(base);
        await ready(page);
      }
    });
    console.log(`${checks} startup scenarios passed`);
  } finally {
    if (browser) await browser.close();
    server.closeAllConnections();
    await new Promise(resolve => server.close(resolve));
  }
}

main().catch(error => { console.error(error); process.exitCode = 1; });
