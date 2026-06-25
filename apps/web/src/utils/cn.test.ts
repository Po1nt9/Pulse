import { describe, it, expect } from 'vitest';
import { cn } from './cn';

describe('cn', () => {
  it('joins multiple class names with a space', () => {
    expect(cn('foo', 'bar')).toBe('foo bar');
  });

  it('filters out falsy values', () => {
    expect(cn('a', false, null, undefined, '', 'b')).toBe('a b');
  });

  it('resolves tailwind conflicts keeping the last definition', () => {
    expect(cn('px-2', 'px-4')).toBe('px-4');
    expect(cn('text-sm', 'text-lg')).toBe('text-lg');
  });

  it('keeps non-conflicting classes together with conflict resolution', () => {
    expect(cn('px-2 py-1', 'px-4')).toBe('py-1 px-4');
  });

  it('handles conditional object syntax from clsx', () => {
    expect(cn({ hidden: false, visible: true }, 'block')).toBe('visible block');
  });

  it('returns empty string for no inputs', () => {
    expect(cn()).toBe('');
  });
});
