import { describe, it, expect } from 'vitest'
import {
  formatCurrency,
  formatPercentage,
  formatNumber,
  timeRangeLabel,
} from './format'

describe('formatCurrency', () => {
  it('formats values below 1000 with two decimals and currency suffix', () => {
    expect(formatCurrency(0)).toBe('0.00 USD')
    expect(formatCurrency(42.5)).toBe('42.50 USD')
    expect(formatCurrency(999.999)).toBe('1000.00 USD') // toFixed(2) rounds up
  })

  it('uses the exact 1000 boundary as the K tier', () => {
    // Boundary: 999.99 stays in the base tier, 1000 enters the K tier.
    expect(formatCurrency(999.99)).toBe('999.99 USD')
    expect(formatCurrency(1000)).toBe('1.00K USD')
  })

  it('formats the K tier with two decimals', () => {
    expect(formatCurrency(1500)).toBe('1.50K USD')
    expect(formatCurrency(999999)).toBe('1000.00K USD')
  })

  it('uses the exact 1000000 boundary as the M tier', () => {
    expect(formatCurrency(999999.99)).toBe('1000.00K USD')
    expect(formatCurrency(1000000)).toBe('1.00M USD')
  })

  it('formats the M tier with two decimals', () => {
    expect(formatCurrency(1500000)).toBe('1.50M USD')
    expect(formatCurrency(12_345_678)).toBe('12.35M USD')
  })

  it('honours a custom currency argument', () => {
    expect(formatCurrency(1000, 'CNY')).toBe('1.00K CNY')
    expect(formatCurrency(2_000_000, 'EUR')).toBe('2.00M EUR')
  })

  it('passes negative values through the base tier formatting', () => {
    expect(formatCurrency(-50)).toBe('-50.00 USD')
  })
})

describe('formatPercentage', () => {
  it('appends a single-decimal percentage', () => {
    expect(formatPercentage(30)).toBe('30.0%')
    expect(formatPercentage(0)).toBe('0.0%')
  })

  it('rounds to one decimal place', () => {
    expect(formatPercentage(33.333)).toBe('33.3%')
    expect(formatPercentage(99.95)).toBe('100.0%')
  })

  it('preserves the sign for negative values', () => {
    expect(formatPercentage(-5)).toBe('-5.0%')
  })
})

describe('formatNumber', () => {
  it('returns the raw string for values below 1000', () => {
    expect(formatNumber(0)).toBe('0')
    expect(formatNumber(42)).toBe('42')
    expect(formatNumber(999)).toBe('999')
  })

  it('uses the exact 1000 boundary as the K tier', () => {
    expect(formatNumber(1000)).toBe('1.0K')
    expect(formatNumber(1234)).toBe('1.2K')
  })

  it('uses the exact 1000000 boundary as the M tier', () => {
    expect(formatNumber(1000000)).toBe('1.0M')
    expect(formatNumber(2_500_000)).toBe('2.5M')
  })
})

describe('timeRangeLabel', () => {
  it('maps known range keys to their Chinese labels', () => {
    expect(timeRangeLabel('recent')).toBe('近期')
    expect(timeRangeLabel('today')).toBe('今日')
    expect(timeRangeLabel('week')).toBe('本周')
    expect(timeRangeLabel('month')).toBe('本月')
  })

  it('returns the input unchanged for unknown ranges', () => {
    expect(timeRangeLabel('quarter')).toBe('quarter')
    expect(timeRangeLabel('')).toBe('')
  })
})
