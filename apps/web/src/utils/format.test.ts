import { describe, it, expect } from 'vitest';
import {
  formatCurrency,
  formatPercentage,
  formatNumber,
  formatDate,
  timeRangeLabel,
} from './format';

describe('formatCurrency', () => {
  it('formats values below 1K with 2 decimals and the currency suffix', () => {
    expect(formatCurrency(42.5, 'USD')).toBe('42.50 USD');
    expect(formatCurrency(0, 'CNY')).toBe('0.00 CNY');
  });

  it('uses USD as the default currency', () => {
    expect(formatCurrency(12.345)).toBe('12.35 USD');
  });

  it('formats values >= 1000 and < 1_000_000 with a K suffix', () => {
    expect(formatCurrency(1500)).toBe('1.50K USD');
    expect(formatCurrency(999_999.99)).toBe('1000.00K USD');
  });

  it('formats values >= 1_000_000 with an M suffix', () => {
    expect(formatCurrency(1_000_000)).toBe('1.00M USD');
    expect(formatCurrency(2_500_000, 'EUR')).toBe('2.50M EUR');
  });

  it('selects the magnitude branch exactly at the thresholds (>= is inclusive)', () => {
    // The implementation uses `>=`, so the boundary itself switches the suffix.
    expect(formatCurrency(1000)).toBe('1.00K USD');
    expect(formatCurrency(999.999)).toBe('1000.00 USD');
    expect(formatCurrency(1_000_000)).toBe('1.00M USD');
    expect(formatCurrency(999_999.99)).toBe('1000.00K USD');
  });
});

describe('formatPercentage', () => {
  it('rounds to one decimal place and appends the percent sign', () => {
    expect(formatPercentage(42.34)).toBe('42.3%');
    expect(formatPercentage(42.36)).toBe('42.4%');
    expect(formatPercentage(0)).toBe('0.0%');
    expect(formatPercentage(100)).toBe('100.0%');
  });

  it('handles negative values', () => {
    expect(formatPercentage(-5.2)).toBe('-5.2%');
  });
});

describe('formatNumber', () => {
  it('formats small numbers as plain strings', () => {
    expect(formatNumber(0)).toBe('0');
    expect(formatNumber(42)).toBe('42');
  });

  it('formats values >= 1000 and < 1_000_000 with a K suffix (1 decimal)', () => {
    expect(formatNumber(1500)).toBe('1.5K');
    expect(formatNumber(999_999)).toBe('1000.0K');
  });

  it('formats values >= 1_000_000 with an M suffix (1 decimal)', () => {
    expect(formatNumber(1_000_000)).toBe('1.0M');
    expect(formatNumber(2_400_000)).toBe('2.4M');
  });

  it('switches magnitude exactly at the inclusive thresholds', () => {
    expect(formatNumber(1000)).toBe('1.0K');
    expect(formatNumber(999)).toBe('999');
    expect(formatNumber(1_000_000)).toBe('1.0M');
  });
});

describe('formatDate', () => {
  it('formats a valid ISO date string into a localized zh-CN date', () => {
    const out = formatDate('2026-06-21T14:30:00');
    // zh-CN short month + numeric day -> "6月21日" is part of the output.
    expect(out).toContain('6月21日');
    expect(out.length).toBeGreaterThan(0);
  });

  it('produces a non-empty string for a valid date rather than throwing', () => {
    const out = formatDate('2026-01-01T00:00:00');
    expect(typeof out).toBe('string');
    expect(out).toContain('1月1日');
  });

  it('does not throw on an unparseable date (Invalid Date is localized)', () => {
    // The implementation does not validate; this locks the no-throw contract
    // so a future "throw on invalid" change is an intentional, reviewed break.
    expect(() => formatDate('not-a-real-date')).not.toThrow();
  });
});

describe('timeRangeLabel', () => {
  it.each([
    ['recent', '近期'],
    ['today', '今日'],
    ['week', '本周'],
    ['month', '本月'],
  ])('maps the known range %s -> %s', (range, expected) => {
    expect(timeRangeLabel(range)).toBe(expected);
  });

  it('passes through unknown ranges unchanged (fallback branch)', () => {
    expect(timeRangeLabel('year')).toBe('year');
    expect(timeRangeLabel('')).toBe('');
  });
});
