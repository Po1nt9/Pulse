import { describe, it, expect } from 'vitest';
import { getStatusColor, getStatusClass, getStatusDotClass } from './colors';

describe('getStatusColor (hex thresholds)', () => {
  it('returns the danger color at and above 90% usage', () => {
    expect(getStatusColor(90)).toBe('#EF4444');
    expect(getStatusColor(100)).toBe('#EF4444');
  });

  it('returns the warning color in the [70, 90) band', () => {
    expect(getStatusColor(70)).toBe('#F59E0B');
    expect(getStatusColor(89.999)).toBe('#F59E0B');
  });

  it('returns the ok color below 70%', () => {
    expect(getStatusColor(69.999)).toBe('#34c759');
    expect(getStatusColor(0)).toBe('#34c759');
  });

  it('treats the exact thresholds as inclusive boundaries', () => {
    // Off-by-one here would mislabel a provider's health in the UI.
    expect(getStatusColor(89.999)).toBe('#F59E0B');
    expect(getStatusColor(90)).toBe('#EF4444');
    expect(getStatusColor(69.999)).toBe('#34c759');
    expect(getStatusColor(70)).toBe('#F59E0B');
  });
});

describe('getStatusClass (tailwind text classes)', () => {
  it('maps usage bands to status text classes', () => {
    expect(getStatusClass(95)).toBe('text-status-danger');
    expect(getStatusClass(75)).toBe('text-status-warning');
    expect(getStatusClass(50)).toBe('text-status-ok');
  });

  it('switches class exactly at the 70 and 90 boundaries', () => {
    expect(getStatusClass(69.999)).toBe('text-status-ok');
    expect(getStatusClass(70)).toBe('text-status-warning');
    expect(getStatusClass(89.999)).toBe('text-status-warning');
    expect(getStatusClass(90)).toBe('text-status-danger');
  });
});

describe('getStatusDotClass (tailwind bg classes)', () => {
  it('maps usage bands to status dot bg classes', () => {
    expect(getStatusDotClass(95)).toBe('bg-status-danger');
    expect(getStatusDotClass(75)).toBe('bg-status-warning');
    expect(getStatusDotClass(50)).toBe('bg-status-ok');
  });

  it('switches dot class exactly at the 70 and 90 boundaries', () => {
    expect(getStatusDotClass(69.999)).toBe('bg-status-ok');
    expect(getStatusDotClass(70)).toBe('bg-status-warning');
    expect(getStatusDotClass(89.999)).toBe('bg-status-warning');
    expect(getStatusDotClass(90)).toBe('bg-status-danger');
  });
});
