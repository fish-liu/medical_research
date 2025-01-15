
import { defaultMessage } from "$lib/store";
import dayjs from 'dayjs';

export function load() {
    // 通过该函数，重定向到timeline页面
    // 参考文章： https://www.luxiangdong.com/2023/04/25/sveltekit2/
    // 官方文档： https://kit.svelte.js.cn/docs/routing

    console.log("----redirect-----");
    /*
    defaultMessage.set({
        role:"assistant",
        content:"欢迎咨询智能AI",
        createTime: dayjs().format('HH:mm:ss')
    });
    */
}
