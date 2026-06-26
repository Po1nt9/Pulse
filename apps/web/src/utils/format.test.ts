import { describe, it, expect } from 'vitest';
import {
  formatCurrency,
  formatNumber,
  formatPercentage,
  timeRangeLabel,
} from './format';

// Note: formatDate() is intentionally not tested here — it relies on
// toLocaleDateString('zh-CN', ...) whose output depends on the host ICU/Node
// version and timezone, making exact-value assertions non-deterministic.

describe('formatCurrency', () => {
  it('formats values below the 1K threshold with two decimals', () => {
    expect(formatCurrency(0)).toBe('0.00 USD');
    expect(formatCurrency(999.99)).toBe('999.99 USD');
  });

  it('switches to the K suffix exactly at 1000', () => {
    expect(formatCurrency(1000)).toBe('1.00K USD');
    expect(formatCurrency(1500)).toBe('1.50K USD');
  });

  it('switches to the M suffix exactly at 1,000,000', () => {
    expect(formatCurrency(1000000)).toBe('1.00M USD');
    expect(formatCurrency(2500000)).toBe('2.50M USD');
  });

  it('honours a custom currency argument', () => {
    expect(formatCurrency(5000, 'CNY')).toBe('5.00K CNY');
  });

  it('renders negatives in the plain branch (never K/M)', () => {
    // DeepSeek computes used = granted + topped_up - remaining, which can go
    // negative; the guard `value >= 1000` is false for negatives, so they must
    // not be formatted with a K/M suffix.
    expect(formatCurrency(-500)).toBe('-500.00 USD');
    expect(formatCurrency(-1500)).toBe('-1500.00 USD');
  });
});

describe('formatNumber', () => {
  it('returns the raw integer string below 1000', () => {
    expect(formatNumber(0)).toBe('0');
    expect(formatNumber(999)).toBe('999');
  });

  it('switches to the K suffix exactly at 1000', () => {
    expect(formatNumber(1000)).toBe('1.0K');
    expect(formatNumber(1500)).toBe('1.5K');
  });

  it('switches to the M suffix exactly at 1,000,000', () => {
    expect(formatNumber(1000000)).toBe('1.0M');
    expect(formatNumber(2500000)).toBe('2.5M');
  });
});

describe('formatPercentage', () => {
  it('always emits exactly one decimal place', () => {
    expect(formatPercentage(60)).toBe('60.0%');
    expect(formatPercentage(7)).toBe('7.0%');
    expect(formatPercentage(0)).toBe('0.0%');
    expect(formatPercentage(100)).toBe('100.0%');
  });

  it('truncates/rounds to one decimal', () => {
    expect(formatPercentage(33.333)).toBe('33.3%');
    expect(formatPercentage(12.5)).toBe('12.5%');
  });
});

describe('timeRangeLabel', () => {
  it('maps known range keys to their Chinese labels', () => {
    expect(timeRangeLabel('recent')).toBe('近期');
    expect(timeRangeLabel('today')).toBe('今日');
    expect(timeRangeLabel('week')).toBe('本周');
    expect(timeRangeLabel('month')).toBe('本月');
  });

  it('passes unknown ranges through unchanged (no undefined leak)', () => {
    expect(timeRangeLabel('custom')).toBe('custom');
    expect(timeRangeLabel('')).toBe('');
  });
});
