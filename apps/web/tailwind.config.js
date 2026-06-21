/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        glass: {
          bg: 'rgba(35, 45, 60, 0.82)',
          border: 'rgba(255, 255, 255, 0.08)',
          hover: 'rgba(255, 255, 255, 0.12)',
        },
        accent: {
          DEFAULT: '#0EA5E9',
          dim: 'rgba(14, 165, 233, 0.15)',
        },
        status: {
          ok: '#34c759',
          warning: '#F59E0B',
          danger: '#EF4444',
        },
        surface: {
          DEFAULT: '#232D3C',
          elevated: '#2A3545',
        }
      },
      fontFamily: {
        sans: ['Inter', '-apple-system', 'BlinkMacSystemFont', '"PingFang SC"', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      backdropBlur: {
        glass: '40px',
      },
      animation: {
        'popup-in': 'popupIn 0.3s cubic-bezier(0.16, 1, 0.3, 1)',
        'grow-up': 'growUp 0.5s cubic-bezier(0.16, 1, 0.3, 1)',
        'pulse-danger': 'pulseDanger 2s ease-in-out infinite',
      },
      keyframes: {
        popupIn: {
          '0%': { opacity: '0', transform: 'scale(0.95) translateY(8px)', filter: 'blur(4px)' },
          '100%': { opacity: '1', transform: 'scale(1) translateY(0)', filter: 'blur(0)' },
        },
        growUp: {
          '0%': { transform: 'scaleY(0)' },
          '100%': { transform: 'scaleY(1)' },
        },
        pulseDanger: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.6' },
        },
      },
    },
  },
  plugins: [],
}
