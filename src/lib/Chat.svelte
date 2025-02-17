<script lang="ts">
    import dayjs from 'dayjs';
    import { invoke } from "@tauri-apps/api/core";
    import ChatBubble from './ChatBubble.svelte';
    import {onMount} from 'svelte'

    // 文章参考： https://stackblitz.com/edit/vitejs-vite-xv9eqmxq?file=src%2Fmain.js
    import { defaultMessage,chatMessages } from "$lib/store";
    import type { Message4Chat } from './types/message.type';

    let newMessage: string = '';
    let timeline : HTMLElement;
    let chatHtml: HTMLElement;

    // 是否在加载数据
    let loading = false;

    onMount(() => {
		console.log('the component has mounted');
        //alert(chatHtml.scrollHeight);
	});

    // 监听键盘事件
    function sendKeyHandler(event: { key: string; target: { value: string; }; }) {
        if (event.key === 'Enter') {
            const text = event.target.value;

            if (!text || text.trim().length == 0) {
                return;
            }

            // 添加聊天信息
            addMessage("user",text.trim());
            // 调用机器人
            chat4robot();

            event.target.value = '';
        }
    }

    // 点击发送按钮事件
    async function sendMessage() {
        console.log("-----send-----",newMessage);
        //alert(chatHtml.scrollHeight);
        if(newMessage.trim().length == 0){
            return;
        }

        // 添加聊天信息
        addMessage("user",newMessage.trim());
        // 调用机器人
        chat4robot();

        newMessage = "";
    }

    // 跟机器人聊天
    async function chat4robot() {
        console.log("------chat4robot--------");

        let sendMessages = [];
        for (let i = 0; i < $chatMessages.length; i++) {
            let message:Message4Chat = $chatMessages[i];
            sendMessages[i] = {
                "role":message.role,
                "content":message.content
            }
        }

        /*
        loading = true;
        setTimeout(() => {
            loading = false;
        },3000)
        */
        // 调用 api
        
        loading = true;
        await invoke("chat",{messages:sendMessages}).then((res:any)=>{
            loading = false;
            console.log("2222222:",res);
            if(res.code == 200){
                let content = res.data;
                // 添加聊天信息
                addMessage("assistant",content);
            }
        });
    }

    // 添加聊天信息
    function addMessage(role:string,newMessage: string){
        $chatMessages = [...$chatMessages, {
            role: role,
            content: newMessage.trim(),
            createTime: dayjs().format('HH:mm:ss'),
            }
        ];

        // 滚动条至最下面
        refresh4height();
    }

    // 刷新聊天区的滚动条
    function refresh4height(){
        // https://cloud.tencent.com/developer/ask/sof/1224460
        setTimeout(() => {
            //alert(chatHtml.scrollHeight);
            timeline.scrollTop = timeline.scrollHeight;
            //console.log("---scrollTop--222--",timeline.scrollTop);
        },0)
    }
    
    function focusHandle(event:any){

        //refresh4height();
        // 手机端只能使用 alert来弹信息查看，目前没找到更好的版本，好像是有一个js文件，vconsole.js把，有时间研究
        //alert(chatHtml.scrollHeight);
        //alert(timeline.scrollHeight);
        
        setTimeout(() => {
            //alert("resert");
            //alert(chatHtml.scrollHeight);
            //alert(timeline.scrollHeight);
            timeline.scrollTop = timeline.scrollHeight;
        },100)
    }
    /* https://svelte.dev/playground/8b974ea483c648fba362a1e9f3dbc29f?version=5.18.0
    function resize(element:any){
        alert("88888");
    }
    */
</script>

<div class="flex flex-col h-screen overflow-hidden" bind:this={chatHtml}>
    <!-- 头部欢迎语 -->
    <div class="flex-none h-12 text-center bg-gray-100" >
        <div class="h-full" style="font-size: 28px;">
            您的医疗问诊 <span class="text-sky-400 {loading ? "": "hidden"}" style="font-size: 14px; margin-left: 20px;">数据加载中...</span>
        </div>
    </div>

    <!-- 聊天信息区 -->
    <div class="flex-1 overflow-y-auto" bind:this={ timeline }>

        <ChatBubble message = "{$defaultMessage}" suport={false} />

        {#each $chatMessages as chat}
            <ChatBubble message="{ chat }" suport={ chat.role=='user' ? true:false} />
        {/each}
    </div>

    <!-- 信息输入区 -->
    <div class="flex-none">
        <div class="border-t-2 border-gray-200 flex items-end px-3 pt-2 mb-2 sm:mb-0" style="padding-bottom: 10px;">
            <!--  语音功能先不实现
            <div class="flex-none w-8">
                <span class="inset-y-0 flex items-center">
                    <button type="button" class="inline-flex items-center justify-center rounded-full h-12 w-12 transition duration-500 ease-in-out text-gray-500 hover:bg-gray-300 focus:outline-none">
                       <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="h-6 w-6 text-gray-600">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"></path>
                       </svg>
                    </button>
                  </span>
            </div>
            -->
            <div class="flex-1 px-1">
                <div class="input-box" contenteditable="true" bind:innerHTML={newMessage} on:focus={ focusHandle }></div>
            </div>

            <div class="flex-none w-18">
                <button type="button"  on:click={() => sendMessage() } 
                    class="inline-flex items-center justify-center rounded-lg px-4 py-2 transition duration-500 ease-in-out text-white bg-green-600 hover:bg-green-500 focus:outline-none">
                     <span class="font-bold">发送</span>
                  </button>
            </div>

            <!--
            <div class="relative flex">
                 
                <span class="absolute inset-y-0 flex items-center">
                  <button type="button" class="inline-flex items-center justify-center rounded-full h-12 w-12 transition duration-500 ease-in-out text-gray-500 hover:bg-gray-300 focus:outline-none">
                     <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="h-6 w-6 text-gray-600">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"></path>
                     </svg>
                  </button>
                </span>
                <input type="text" class="w-full focus:outline-none focus:placeholder-gray-400 text-gray-600 placeholder-gray-600 pl-12 bg-gray-200 rounded-md py-3"
                    bind:value={ newMessage }
                    on:keydown={ sendKeyHandler }
                    placeholder="请输入您的信息!" >
                -->
                <!-- 
                <input type="text" class="w-full focus:outline-none focus:placeholder-gray-400 text-gray-600 placeholder-gray-600 pl-2 bg-gray-200 rounded-md py-3 multiline-input" 
                    bind:value={ newMessage }
                    on:keydown={ sendKeyHandler }
                    on:focus={ focusHandle }
                    placeholder="请输入您的信息!" >
                
                <textarea
                    class="w-full text-slate-600 bg-slate-100 border border-transparent hover:border-slate-200 appearance-none rounded px-3.5 py-2.5 outline-none focus:bg-white focus:border-indigo-400 focus:ring-2 focus:ring-indigo-100"
                    name="message"
                    id="message"
                    rows="2"
                    placeholder="Your request..." 
                    required
                    ></textarea>

               <div class="absolute right-0 items-center inset-y-0 sm:flex">
                  
                  <button type="button"  on:click={() => sendMessage() } class="inline-flex items-center justify-center rounded-lg px-4 py-3 transition duration-500 ease-in-out text-white bg-blue-400 hover:bg-blue-400 focus:outline-none">
                     <span class="font-bold">发送</span>
                  </button>
               </div>
               
            </div>
            -->
         </div>
    </div>
</div> 

<style>
.input-box{
    border: 1px solid #ccc;
    width: 100%;
    height: max-content;
    /*min-height: 20px;
    line-height: 20px;
    color: white;*/
    padding: 0.5rem;
    outline: none;
    /*background: hsl(222, 22%, 20%);*/
    overflow: hidden;
    bottom: 1rem;
    right: 1rem;
    border-radius: 0.25rem;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    word-break:break-all;  /* 强制换行*/
}

.input-box[contenteditable]:empty::before {
  content: "请输入您的信息!";
  color: gray;
}

/*
.input-box:hover{
    border-color: hsl(222, 22%, 30%);
}

.input-box:focus{
    border-color: hsl(222, 22%, 40%);
}
*/
</style>