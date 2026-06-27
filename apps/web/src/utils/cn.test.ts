import { describe, it, expect } from 'vitest';
import { cn } from './cn';

describe('cn', () => {
  it('joins plain class names with spaces', () => {
    expect(cn('px-2', 'py-1')).toBe('px-2 py-1');
  });

  it('ignores falsy inputs (undefined / null / false)', () => {
    expect(cn('base', undefined, null, false, 'tail')).toBe('base tail');
  });

  it('resolves conflicting tailwind classes via twMerge (last wins)', () => {
    // twMerge is the load-bearing behavior — dropping it would cause
    // conflicting utility classes to render unreliably across the UI.
    expect(cn('px-2', 'px-4')).toBe('px-4');
    expect(cn('text-red-500', 'text-blue-500')).toBe('text-blue-500');
  });

  it('keeps non-conflicting classes and merges conflicts', () => {
    expect(cn('px-2 py-1', 'px-4')).toBe('py-1 px-4');
  });

  it('handles conditional class objects and arrays (clsx semantics)', () => {
    // Use non-conflicting utilities so clsx's conditional logic is observable
    // after twMerge (conflicting pairs like hidden+flex would be de-duped).
    expect(cn('base', { 'font-bold': true, 'italic': false }, ['rounded'])).toBe(
      'base font-bold rounded',
    );
  });
});
