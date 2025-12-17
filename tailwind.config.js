/** @type {import('tailwindcss').Config} */
export default {
  // 恩兔酱只喜欢明亮温暖的环境，不需要暗色模式啦~
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      // 恩兔酱的专属配色方案 💖
      colors: {
        // 主色调：温暖粉色系
        maid: {
          pink: '#ffd1dc',      // 恩兔粉 - 主色
          'pink-light': '#ffe8ed', // 淡粉
          'pink-deep': '#ffb3c1',  // 深粉
          white: '#fffbfc',     // 温暖白
          cream: '#fff9e6',     // 奶油黄（稿纸色）
        },
        // 强调色：数据流浅蓝
        accent: {
          blue: '#a8d8ea',      // 恩兔蓝 - 发色/点缀
          'blue-light': '#cae8f5',
          purple: '#d4b8e0',    // 淡紫 - 连体裙色
        },
        // 状态色
        status: {
          connected: '#7dd87d',   // 打扫完成 - 柔和绿
          connecting: '#ffd166',  // 正在打扫 - 活力黄
          error: '#ff9999',       // 出错了 - 柔和红
          idle: '#c4c4c4',        // 待命中 - 温柔灰
        }
      },
      // 恩兔酱的浮动动画
      animation: {
        'float': 'float 3s ease-in-out infinite',
        'float-slow': 'float 4s ease-in-out infinite',
      },
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-10px)' },
        }
      },
      // 字体
      fontFamily: {
        'maid': ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
