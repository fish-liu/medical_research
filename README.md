

命令行： cargo create-tauri-app

# Tauri + SvelteKit + TypeScript


安装tainwind 


pnpm install -D tailwindcss postcss autoprefixer


npx tailwindcss init  (该命令生成tailwind.config.js)


配置tailwind.config.js


content: ['./src/**/*.{svelte,js,ts}', './public/index.html'],


创建 postcss.config.js

在src目录下创建 app.css文件

内容如下：
@tailwind base;
@tailwind components;
@tailwind utilities;

在 routes目录下的+layout.svelte文件中引入该文件

