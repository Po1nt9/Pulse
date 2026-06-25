import { describe, it, expect } from 'vitest';
import {
  formatCurrency,
  formatPercentage,
  formatNumber,
  formatDate,
  timeRangeLabel,
} from './format';

describe('formatCurrency', () => {
  it('formats values below 1000 with two decimals and currency suffix', () => {
    expect(formatCurrency(999.99)).toBe('999.99 USD');
    expect(formatCurrency(0)).toBe('0.00 USD');
    expect(formatCurrency(12.345)).toBe('12.35 USD');
  });

  it('formats thousands with K suffix', () => {
    expect(formatCurrency(1000)).toBe('1.00K USD');
    expect(formatCurrency(1500)).toBe('1.50K USD');
    expect(formatCurrency(999999)).toBe('1000.00K USD');
  });

  it('formats millions with M suffix', () => {
    expect(formatCurrency(1000000)).toBe('1.00M USD');
    expect(formatCurrency(1234567)).toBe('1.23M USD');
  });

  it('uses the provided currency code', () => {
    expect(formatCurrency(500, 'CNY')).toBe('500.00 CNY');
    expect(formatCurrency(2000, 'EUR')).toBe('2.00K EUR');
  });

  it('defaults currency to USD', () => {
    expect(formatCurrency(10)).toBe('10.00 USD');
  });

  it('renders negatives in the plain branch regardless of magnitude', () => {
    // The thresholds use `value >= 1000`, so negative values never match the
    // K/M branches even when their absolute value is large. Pin this behavior
    // so a future change to the threshold logic is caught.
    expect(formatCurrency(-50)).toBe('-50.00 USD');
    expect(formatCurrency(-5000)).toBe('-5000.00 USD');
    expect(formatCurrency(-1000000)).toBe('-1000000.00 USD');
  });
});

describe('formatPercentage', () => {
  it('formats with one decimal and percent sign', () => {
    expect(formatPercentage(60)).toBe('60.0%');
    expect(formatPercentage(60.25)).toBe('60.3%');
    expect(formatPercentage(0)).toBe('0.0%');
  });

  it('rounds half up', () => {
    expect(formatPercentage(33.35)).toBe('33.4%');
  });
});

describe('formatNumber', () => {
  it('returns plain string for values below 1000', () => {
    expect(formatNumber(999)).toBe('999');
    expect(formatNumber(0)).toBe('0');
    expect(formatNumber(42)).toBe('42');
  });

  it('formats thousands with K suffix (one decimal)', () => {
    expect(formatNumber(1000)).toBe('1.0K');
    expect(formatNumber(1500)).toBe('1.5K');
  });

  it('formats millions with M suffix (one decimal)', () => {
    expect(formatNumber(1000000)).toBe('1.0M');
    expect(formatNumber(1234567)).toBe('1.2M');
  });
});

describe('formatDate', () => {
  it('returns a non-empty localized string for a valid ISO date', () => {
    const out = formatDate('2026-06-21T10:30:00Z');
    expect(typeof out).toBe('string');
    expect(out.length).toBeGreaterThan(0);
    // Should not contain the raw ISO timestamp verbatim.
    expect(out).not.toContain('2026-06-21T10:30:00Z');
  });

  it('does not throw for an unparseable date', () => {
    expect(() => formatDate('not-a-date')).not.toThrow();
  });
});

describe('timeRangeLabel', () => {
  it('returns the Chinese label for known ranges', () => {
    expect(timeRangeLabel('recent')).toBe('近期');
    expect(timeRangeLabel('today')).toBe('今日');
    expect(timeRangeLabel('week')).toBe('本周');
    expect(timeRangeLabel('month')).toBe('本月');
  });

  it('falls back to the raw range for unknown keys', () => {
    expect(timeRangeLabel('year')).toBe('year');
    expect(timeRangeLabel('')).toBe('');
  });
});
