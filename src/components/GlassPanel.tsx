import React from 'react';
import { cn } from '../utils/cn';

interface GlassPanelProps {
  children: React.ReactNode;
  className?: string;
  padding?: 'none' | 'sm' | 'md' | 'lg';
  onClick?: React.MouseEventHandler<HTMLDivElement>;
}

export function GlassPanel({ children, className, padding = 'md', onClick }: GlassPanelProps) {
  const paddingClasses = {
    none: '',
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6',
  };

  return (
    <div className={cn('glass-panel', paddingClasses[padding], className)} onClick={onClick}>
      {children}
    </div>
  );
}
