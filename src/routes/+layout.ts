// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

import { invoke } from "@tauri-apps/api/core";
import { chatType,defaultMessage } from "$lib/store";
import dayjs from 'dayjs';
export async function load() {

    let local_chat_type = localStorage.getItem('chat_type');
    // console.log("---localStorage chat_type----",local_chat_type);
    if(local_chat_type == null ){
        let chat_type:string = await invoke("get_chat_type");

        // console.log("---admin chat_type----",chat_type);
        if(chat_type != ""){
            chatType.set(chat_type);
            localStorage.setItem("chat_type",chat_type);
            let content:string;
            if(chat_type =='1'){
                content = "你好 ~我是MedConsult，你的AI医疗助手。问诊过程中，除了详细的病症描述，也可以告诉我你的情绪，我会尽我所能理解你的~"
            }else {
                content = "你好\~我是MedConsult，你的AI医疗助手。问诊过程中，请详细地告诉我你的症状，我会给予你相应的医疗建议，感谢配合~"
            }
            defaultMessage.set({
                role:"assistant",
                content:content,
                createTime:dayjs().format('HH:mm:ss')
            })
        }
    }else{
        chatType.set(local_chat_type);

        let content:string;
        if(local_chat_type =='1'){
            content = "你好 ~我是MedConsult，你的AI医疗助手。问诊过程中，除了详细的病症描述，也可以告诉我你的情绪，我会尽我所能理解你的~"
        }else {
            content = "你好\~我是MedConsult，你的AI医疗助手。问诊过程中，请详细地告诉我你的症状，我会给予你相应的医疗建议，感谢配合~"
        }
        defaultMessage.set({
            role:"assistant",
            content:content,
            createTime:dayjs().format('HH:mm:ss')
        })
    }

}

