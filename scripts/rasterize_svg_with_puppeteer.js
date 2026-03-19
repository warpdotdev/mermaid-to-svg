#!/usr/bin/env node

const fs = require('fs');

function parseViewBox(svg) {
  const m = svg.match(/\bviewBox\s*=\s*"([^"]+)"/i);
  if (!m) return null;
  const parts = m[1].trim().split(/[\s,]+/).map(Number);
  if (parts.length !== 4 || parts.some((n) => !Number.isFinite(n))) return null;
  return { x: parts[0], y: parts[1], width: parts[2], height: parts[3] };
}

function parseNumericAttr(svg, attr) {
  const m = svg.match(new RegExp(`\\b${attr}\\s*=\\s*"([^\"]+)"`, 'i'));
  if (!m) return null;
  const raw = m[1].trim();
  // Ignore percentages; we need a concrete pixel size.
  if (raw.endsWith('%')) return null;
  // Accept bare numbers or values with px.
  const num = Number(raw.replace(/px$/i, ''));
  if (!Number.isFinite(num)) return null;
  return num;
}

function inferSvgSize(svg) {
  const vb = parseViewBox(svg);
  if (vb && vb.width > 0 && vb.height > 0) {
    return { width: vb.width, height: vb.height };
  }

  const width = parseNumericAttr(svg, 'width');
  const height = parseNumericAttr(svg, 'height');
  if (width && height && width > 0 && height > 0) {
    return { width, height };
  }

  // SVG default intrinsic size per spec.
  return { width: 300, height: 150 };
}

async function main() {
  const [inputSvgPath, outputPngPath, scaleRaw, background] = process.argv.slice(2);
  if (!inputSvgPath || !outputPngPath) {
    console.error('usage: rasterize_svg_with_puppeteer.js <input.svg> <output.png> [scale] [background]');
    process.exit(2);
  }

  const scale = scaleRaw ? Number(scaleRaw) : 2;
  if (!Number.isFinite(scale) || scale <= 0) {
    console.error(`invalid scale: ${scaleRaw}`);
    process.exit(2);
  }

  const svg = fs.readFileSync(inputSvgPath, 'utf8');
  const { width, height } = inferSvgSize(svg);
  const viewportWidth = Math.max(1, Math.ceil(width));
  const viewportHeight = Math.max(1, Math.ceil(height));

  let puppeteer;
  try {
    puppeteer = require('puppeteer');
  } catch (e) {
    console.error('failed to require puppeteer (is NODE_PATH set?):', e && e.message ? e.message : e);
    process.exit(1);
  }

  const bg = background || 'white';

  const browser = await puppeteer.launch({
    headless: 'new',
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
  });

  try {
    const page = await browser.newPage();
    await page.setViewport({
      width: viewportWidth,
      height: viewportHeight,
      deviceScaleFactor: scale,
    });

    // Inline the SVG so its internal <style> applies and there are no fetches.
    await page.setContent(
      `<!doctype html><html><head><meta charset="utf-8"/></head><body style="margin:0;background:${bg};">${svg}</body></html>`,
      { waitUntil: 'load' }
    );

    // Force a deterministic pixel size for the root SVG.
    await page.evaluate(
      (w, h) => {
        const el = document.querySelector('svg');
        if (!el) return;
        el.setAttribute('width', String(w));
        el.setAttribute('height', String(h));
      },
      viewportWidth,
      viewportHeight
    );

    const handle = await page.$('svg');
    if (!handle) {
      console.error('no <svg> element found in input');
      process.exit(1);
    }

    const clip = await handle.boundingBox();
    if (!clip) {
      console.error('failed to compute bounding box for <svg>');
      process.exit(1);
    }

    await page.screenshot({ path: outputPngPath, clip });
    await page.close();
  } finally {
    await browser.close();
  }
}

main().catch((e) => {
  console.error(e && e.stack ? e.stack : e);
  process.exit(1);
});
