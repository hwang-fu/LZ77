/**
 * Tests for LZ77 compression algorithm.
 *
 * Run with: npx tsx src/lz77.test.ts
 */

import {
  compress,
  decompress,
  decompressToString,
  encodeTokens,
  decodeTokens,
  compressToBinary,
  decompressFromBinary,
} from "./index";

interface TestResult {
  name: string;
  passed: boolean;
  error?: string;
}

const results: TestResult[] = [];

function test(name: string, fn: () => void): void {
  try {
    fn();
    results.push({ name, passed: true });
    console.log(`✓ ${name}`);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    results.push({ name, passed: false, error: message });
    console.log(`✗ ${name}`);
    console.log(`  Error: ${message}`);
  }
}

function assertEqual<T>(actual: T, expected: T, message?: string): void {
  const actualStr = JSON.stringify(actual);
  const expectedStr = JSON.stringify(expected);
  if (actualStr !== expectedStr) {
    throw new Error(
      message || `Expected ${expectedStr}, got ${actualStr}`
    );
  }
}

function assertArrayEqual(actual: Uint8Array, expected: Uint8Array): void {
  if (actual.length !== expected.length) {
    throw new Error(
      `Length mismatch: expected ${expected.length}, got ${actual.length}`
    );
  }
  for (let i = 0; i < actual.length; i++) {
    if (actual[i] !== expected[i]) {
      throw new Error(`Mismatch at index ${i}: expected ${expected[i]}, got ${actual[i]}`);
    }
  }
}

// Test cases
console.log("\n=== LZ77 Compression Tests ===\n");

test("compress and decompress simple string", () => {
  const original = "abracadabra";
  const tokens = compress(original);
  const result = decompressToString(tokens);
  assertEqual(result, original);
});

test("compress and decompress with repetition", () => {
  const original = "aaaaaaaaaa";
  const tokens = compress(original);
  const result = decompressToString(tokens);
  assertEqual(result, original);
});

test("compress and decompress long repeated pattern", () => {
  const original = "the quick brown fox jumps over the lazy dog. the quick brown fox jumps over the lazy dog.";
  const tokens = compress(original);
  const result = decompressToString(tokens);
  assertEqual(result, original);
});

test("compress and decompress binary data", () => {
  const original = new Uint8Array([0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3]);
  const tokens = compress(original);
  const result = decompress(tokens);
  assertArrayEqual(result, original);
});

test("compress and decompress empty input", () => {
  const original = "";
  const tokens = compress(original);
  const result = decompressToString(tokens);
  assertEqual(result, original);
});

test("compress and decompress single character", () => {
  const original = "x";
  const tokens = compress(original);
  const result = decompressToString(tokens);
  assertEqual(result, original);
});

test("binary encoding round-trip", () => {
  const original = "Hello, World! Hello, World!";
  const binary = compressToBinary(original);
  const result = new TextDecoder().decode(decompressFromBinary(binary));
  assertEqual(result, original);
});

test("token encoding and decoding", () => {
  const original = "abcabcabc";
  const tokens = compress(original);
  const encoded = encodeTokens(tokens);
  const decoded = decodeTokens(encoded);
  assertEqual(decoded, tokens);
});

test("compression reduces size for repetitive data", () => {
  const original = "abcdefgh".repeat(100);
  const tokens = compress(original);
  const binary = encodeTokens(tokens);

  // Binary representation should be smaller than original
  if (binary.length >= original.length) {
    throw new Error(
      `Compression did not reduce size: ${binary.length} >= ${original.length}`
    );
  }
  console.log(`    Original: ${original.length} bytes, Compressed: ${binary.length} bytes`);
});

test("custom window size option", () => {
  const original = "the cat sat on the mat and the cat sat on the hat";
  const tokens = compress(original, { windowSize: 1024 });
  const result = decompressToString(tokens);
  assertEqual(result, original);
});

test("handles all byte values", () => {
  const original = new Uint8Array(256);
  for (let i = 0; i < 256; i++) {
    original[i] = i;
  }
  const tokens = compress(original);
  const result = decompress(tokens);
  assertArrayEqual(result, original);
});

test("handles unicode strings", () => {
  const original = "Hello 世界!";
  const tokens = compress(original);
  const result = decompressToString(tokens);
  assertEqual(result, original);
});

// Summary
console.log("\n=== Summary ===\n");
const passed = results.filter((r) => r.passed).length;
const failed = results.filter((r) => !r.passed).length;
console.log(`Passed: ${passed}/${results.length}`);
console.log(`Failed: ${failed}/${results.length}`);

if (failed > 0) {
  console.log("\nFailed tests:");
  results
    .filter((r) => !r.passed)
    .forEach((r) => console.log(`  - ${r.name}: ${r.error}`));
  process.exit(1);
}

