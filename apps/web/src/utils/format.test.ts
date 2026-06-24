import { describe, it, expect } from 'vitest';
import {
  formatCurrency,
  formatPercentage,
  formatNumber,
  formatDate,
  timeRangeLabel,
} from './format';

describe('formatCurrency', () => {
  it('formats values below 1K with two decimals and currency suffix', () => {
    expect(formatCurrency(999.99)).toBe('999.99 USD');
    expect(formatCurrency(0)).toBe('0.00 USD');
  });

  it('uses the provided currency code', () => {
    expect(formatCurrency(0, 'CNY')).toBe('0.00 CNY');
    expect(formatCurrency(1234.567, 'EUR')).toBe('1.23K EUR');
  });

  it('switches to the K suffix at exactly 1000', () => {
    expect(formatCurrency(1000)).toBe('1.00K USD');
    expect(formatCurrency(1500)).toBe('1.50K USD');
  });

  it('keeps the K suffix up to (but not including) 1,000,000', () => {
    // 999999.99 / 1000 = 999.99999 -> toFixed(2) = "1000.00"
    expect(formatCurrency(999999.99)).toBe('1000.00K USD');
  });

  it('switches to the M suffix at exactly 1,000,000', () => {
    expect(formatCurrency(1000000)).toBe('1.00M USD');
    expect(formatCurrency(2500000)).toBe('2.50M USD');
  });

  it('handles negative values without throwing', () => {
    expect(formatCurrency(-500)).toBe('-500.00 USD');
  });
});

describe('formatPercentage', () => {
  it('appends a percent sign with one decimal', () => {
    expect(formatPercentage(0)).toBe('0.0%');
    expect(formatPercentage(100)).toBe('100.0%');
    expect(formatPercentage(33)).toBe('33.0%');
  });

  it('rounds to one decimal place', () => {
    expect(formatPercentage(42.56)).toBe('42.6%');
    expect(formatPercentage(-5.24)).toBe('-5.2%');
  });
});

describe('formatNumber', () => {
  it('returns the plain string for values below 1000', () => {
    expect(formatNumber(0)).toBe('0');
    expect(formatNumber(999)).toBe('999');
    expect(formatNumber(-50)).toBe('-50');
  });

  it('switches to the K suffix at exactly 1000', () => {
    expect(formatNumber(1000)).toBe('1.0K');
  });

  it('keeps the K suffix up to (but not including) 1,000,000', () => {
    // 999999 / 1000 = 999.999 -> toFixed(1) = "1000.0"
    expect(formatNumber(999999)).toBe('1000.0K');
  });

  it('switches to the M suffix at exactly 1,000,000', () => {
    expect(formatNumber(1000000)).toBe('1.0M');
    expect(formatNumber(1234567)).toBe('1.2M');
  });
});

describe('formatDate', () => {
  it('returns a formatted, non-empty string for a valid ISO date', () => {
    const result = formatDate('2026-06-21T10:30:00Z');
    // The exact output is timezone-dependent, so we only assert that parsing
    // succeeded: the result is a non-empty string that is not the sentinel
    // "Invalid Date".
    expect(typeof result).toBe('string');
    expect(result.length).toBeGreaterThan(0);
    expect(result).not.toBe('Invalid Date');
  });

  it('degrades to "Invalid Date" for unparseable input instead of throwing', () => {
    expect(formatDate('not-a-real-date')).toBe('Invalid Date');
    expect(formatDate('')).toBe('Invalid Date');
  });
});

describe('timeRangeLabel', () => {
  it('maps known range keys to their localized labels', () => {
    expect(timeRangeLabel('recent')).toBe('近期');
    expect(timeRangeLabel('today')).toBe('今日');
    expect(timeRangeLabel('week')).toBe('本周');
    expect(timeRangeLabel('month')).toBe('本月');
  });

  it('falls back to the raw input for unknown keys', () => {
    expect(timeRangeLabel('unknown')).toBe('unknown');
    expect(timeRangeLabel('')).toBe('');
  });
});
