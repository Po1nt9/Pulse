import { describe, it, expect } from 'vitest';
import {
  formatCurrency,
  formatPercentage,
  formatNumber,
  formatDate,
  timeRangeLabel,
} from './format';

describe('formatCurrency', () => {
  it('uses the default USD currency when none is provided', () => {
    expect(formatCurrency(0)).toBe('0.00 USD');
  });

  it('honors an explicit currency code', () => {
    expect(formatCurrency(12.5, 'CNY')).toBe('12.50 CNY');
  });

  it('formats plain values below the 1K threshold with two decimals', () => {
    expect(formatCurrency(999.99)).toBe('999.99 USD');
  });

  it('switches to the K suffix exactly at 1000', () => {
    // Boundary: 1000 is the first value rendered with the K suffix.
    expect(formatCurrency(1000)).toBe('1.00K USD');
  });

  it('formats values just below 1K without the K suffix', () => {
    expect(formatCurrency(999.999)).toBe('1000.00 USD');
  });

  it('switches to the M suffix exactly at 1,000,000', () => {
    // Boundary: 1,000,000 is the first value rendered with the M suffix.
    expect(formatCurrency(1000000)).toBe('1.00M USD');
  });

  it('keeps the K suffix for values just below 1M', () => {
    expect(formatCurrency(999999.99)).toBe('1000.00K USD');
  });

  it('rounds the mantissa to two decimals in the K band', () => {
    expect(formatCurrency(1500)).toBe('1.50K USD');
  });

  it('rounds the mantissa to two decimals in the M band', () => {
    expect(formatCurrency(2_500_000)).toBe('2.50M USD');
  });
});

describe('formatPercentage', () => {
  it('appends a percent sign and keeps one decimal', () => {
    expect(formatPercentage(60)).toBe('60.0%');
  });

  it('rounds to one decimal place', () => {
    expect(formatPercentage(60.25)).toBe('60.3%');
  });

  it('renders zero as 0.0%', () => {
    expect(formatPercentage(0)).toBe('0.0%');
  });
});

describe('formatNumber', () => {
  it('renders small integers without a suffix', () => {
    expect(formatNumber(999)).toBe('999');
  });

  it('switches to the K suffix exactly at 1000 with one decimal', () => {
    expect(formatNumber(1000)).toBe('1.0K');
  });

  it('switches to the M suffix exactly at 1,000,000 with one decimal', () => {
    expect(formatNumber(1000000)).toBe('1.0M');
  });

  it('keeps the K suffix for values just below 1M', () => {
    expect(formatNumber(999999)).toBe('1000.0K');
  });
});

describe('timeRangeLabel', () => {
  it('maps each known range key to its localized label', () => {
    expect(timeRangeLabel('recent')).toBe('近期');
    expect(timeRangeLabel('today')).toBe('今日');
    expect(timeRangeLabel('week')).toBe('本周');
    expect(timeRangeLabel('month')).toBe('本月');
  });

  it('returns the input unchanged for unknown range keys', () => {
    // Fallback contract: unknown keys must pass through so the UI never
    // silently shows an empty label.
    expect(timeRangeLabel('quarter')).toBe('quarter');
    expect(timeRangeLabel('')).toBe('');
  });
});

describe('formatDate', () => {
  // formatDate delegates to Date#toLocaleDateString('zh-CN', ...), whose
  // hour/minute output depends on the host timezone. We therefore only
  // assert deterministic structural properties here — exact-format
  // assertions would be flaky across environments (CI UTC vs local).
  it('returns a non-empty string for a valid ISO date', () => {
    const result = formatDate('2026-06-16T10:30:00Z');
    expect(typeof result).toBe('string');
    expect(result.length).toBeGreaterThan(0);
  });

  it('produces the Invalid Date sentinel for garbage input', () => {
    // new Date('not-a-date').toLocaleDateString(...) returns "Invalid Date".
    expect(formatDate('not-a-date')).toBe('Invalid Date');
  });
});
