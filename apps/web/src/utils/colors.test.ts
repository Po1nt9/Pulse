import { describe, it, expect } from 'vitest'
import { getStatusColor, getStatusClass, getStatusDotClass } from './colors'

// The three helpers share identical threshold logic (>=90 danger, >=70 warning,
// else ok). The boundary values below pin that contract so a future refactor
// cannot silently shift a threshold or swap a status bucket.
describe('getStatusColor', () => {
  it('returns red for the 90 danger boundary and above', () => {
    expect(getStatusColor(90)).toBe('#EF4444')
    expect(getStatusColor(100)).toBe('#EF4444')
  })

  it('returns amber for the 70 warning boundary up to just below 90', () => {
    expect(getStatusColor(70)).toBe('#F59E0B')
    expect(getStatusColor(89.99)).toBe('#F59E0B')
  })

  it('returns green below the 70 warning boundary', () => {
    expect(getStatusColor(69.99)).toBe('#34c759')
    expect(getStatusColor(0)).toBe('#34c759')
  })
})

describe('getStatusClass', () => {
  it('returns the danger class at and above 90', () => {
    expect(getStatusClass(90)).toBe('text-status-danger')
    expect(getStatusClass(95)).toBe('text-status-danger')
  })

  it('returns the warning class at and above 70', () => {
    expect(getStatusClass(70)).toBe('text-status-warning')
    expect(getStatusClass(89)).toBe('text-status-warning')
  })

  it('returns the ok class below 70', () => {
    expect(getStatusClass(69.99)).toBe('text-status-ok')
    expect(getStatusClass(0)).toBe('text-status-ok')
  })
})

describe('getStatusDotClass', () => {
  it('returns the danger dot class at and above 90', () => {
    expect(getStatusDotClass(90)).toBe('bg-status-danger')
    expect(getStatusDotClass(100)).toBe('bg-status-danger')
  })

  it('returns the warning dot class at and above 70', () => {
    expect(getStatusDotClass(70)).toBe('bg-status-warning')
    expect(getStatusDotClass(89.5)).toBe('bg-status-warning')
  })

  it('returns the ok dot class below 70', () => {
    expect(getStatusDotClass(69.99)).toBe('bg-status-ok')
    expect(getStatusDotClass(0)).toBe('bg-status-ok')
  })
})
