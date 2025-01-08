

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


AcKe：ALTAKioNowgHFo1gEWYT4QqRZW

SeKe：2676be91677d4532be42eeea7328fe66



rust 读取 ymal配置文件 https://segmentfault.com/a/1190000044952130
后期可以考虑将配置信息放到配置文件中。

URL编码
https://blog.csdn.net/bbdxf/article/details/137524206


reqwest模块实战
https://juejin.cn/post/7226177081197068346


--------response-------200 OK
--------response-------"{\"error_code\":100,\"error_msg\":\"Invalid parameter\"}"


