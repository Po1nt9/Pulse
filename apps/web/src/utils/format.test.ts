import { describe, it, expect } from 'vitest';
import {
  formatCurrency,
  formatPercentage,
  formatNumber,
  formatDate,
  timeRangeLabel,
} from './format';

// These formatters render balance / usage figures across the whole UI and
// contain explicit magnitude thresholds (1M, 1K) plus a date parser and a
// label lookup with fallback behaviour. Boundary regressions would surface
// as silently wrong figures to the user.

describe('formatCurrency', () => {
  it('uses the millions suffix at and above 1,000,000', () => {
    expect(formatCurrency(1000000)).toBe('1.00M USD');
    expect(formatCurrency(2500000)).toBe('2.50M USD');
  });

  it('uses the thousands suffix in [1,000, 1,000,000)', () => {
    expect(formatCurrency(1000)).toBe('1.00K USD');
    expect(formatCurrency(12500)).toBe('12.50K USD');
  });

  it('renders the raw value below 1,000', () => {
    expect(formatCurrency(0)).toBe('0.00 USD');
    expect(formatCurrency(999.999)).toBe('1000.00 USD');
    expect(formatCurrency(12.5)).toBe('12.50 USD');
  });

  it('honours a custom currency argument', () => {
    expect(formatCurrency(1500, 'CNY')).toBe('1.50K CNY');
  });

  it('treats the 1,000,000 boundary as millions, not thousands', () => {
    expect(formatCurrency(1000000)).not.toContain('K');
    expect(formatCurrency(999999.99)).toContain('K');
  });
});

describe('formatPercentage', () => {
  it('formats to one decimal place with a percent sign', () => {
    expect(formatPercentage(42)).toBe('42.0%');
    expect(formatPercentage(42.56)).toBe('42.6%');
    expect(formatPercentage(0)).toBe('0.0%');
  });
});

describe('formatNumber', () => {
  it('uses the millions suffix at and above 1,000,000', () => {
    expect(formatNumber(1000000)).toBe('1.0M');
    expect(formatNumber(2300000)).toBe('2.3M');
  });

  it('uses the thousands suffix in [1,000, 1,000,000)', () => {
    expect(formatNumber(1000)).toBe('1.0K');
    expect(formatNumber(12500)).toBe('12.5K');
  });

  it('renders the raw value below 1,000', () => {
    expect(formatNumber(999)).toBe('999');
    expect(formatNumber(0)).toBe('0');
  });
});

describe('formatDate', () => {
  it('returns a formatted string for a valid ISO date', () => {
    const result = formatDate('2026-06-21T10:30:00Z');
    expect(typeof result).toBe('string');
    expect(result.length).toBeGreaterThan(0);
    // Locale-dependent formatting must never surface "Invalid Date" for valid input.
    expect(result).not.toBe('Invalid Date');
  });

  it('surfaces an invalid date deterministically', () => {
    // Parsing edge case: a malformed timestamp produces an Invalid Date, and
    // toLocaleDateString on it returns the literal "Invalid Date" string.
    expect(formatDate('not-a-date')).toBe('Invalid Date');
  });
});

describe('timeRangeLabel', () => {
  it('returns the localized label for known ranges', () => {
    expect(timeRangeLabel('recent')).toBe('近期');
    expect(timeRangeLabel('today')).toBe('今日');
    expect(timeRangeLabel('week')).toBe('本周');
    expect(timeRangeLabel('month')).toBe('本月');
  });

  it('falls back to the raw range for unknown values', () => {
    expect(timeRangeLabel('quarter')).toBe('quarter');
    expect(timeRangeLabel('')).toBe('');
  });
});
