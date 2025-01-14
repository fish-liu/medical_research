<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { chatType,defaultMessage } from "$lib/store";
    import dayjs from 'dayjs';

    let radios = [
        {
            name: "高共情",
            chatType:"1",
            description: "以患者为中心进行治疗和沟通.",
        },
        {
            name: "低共情",
            chatType:"0",
            description: "以任务为中心进行治疗和沟通.",
        },
    ];

    let checkedValue:string ;
    async function checkRadio(value:string){
        checkedValue = value;

        await invoke("set_chat_type",{'chatType':value}).then((res)=>{
            console.log("----set_chat_type  res----",res)

            $chatType = value;
            localStorage.setItem('chat_type',value);

            let content:string;
            if(value =='1'){
                content = "你好 ~我是MedConsult，你的AI医疗助手。问诊过程中，除了详细的病症描述，也可以告诉我你的情绪，我会尽我所能理解你的~"
            }else {
                content = "你好\~我是MedConsult，你的AI医疗助手。问诊过程中，请详细地告诉我你的症状，我会给予你相应的医疗建议，感谢配合~"
            }

            $defaultMessage = {
                role:"assistant",
                content:content,
                createTime: dayjs().format('HH:mm:ss')
            };
        });
    }

</script>

<div class="flex flex-col h-screen overflow-hidden">
    <!-- 头部欢迎语 -->
    <div class="flex-none h-12 text-center bg-gray-100" >
        <div class="h-full" style="font-size: 28px;">
            医疗问诊系统
        </div>
    </div>

    <div class="flex-1 max-w-2xl mx-auto px-4 py-8"  >
        <h2 class="text-gray-800 font-medium">请选择您的医疗类型</h2>
        <ul class="mt-6 space-y-3">
            {#each radios as item, idx}
                <li>
                    <label for={item.name} class="block relative">
                        <input id={item.name} type="radio"
                               checked={checkedValue === item.chatType ? true : false}
                               on:click={() => checkRadio(item.chatType)}
                               name="payment"
                               class="sr-only peer" />
                        <div class="w-full p-5 cursor-pointer rounded-lg border bg-white shadow-sm ring-indigo-600 peer-checked:ring-2 duration-200">
                            <div class="pl-7">
                                <h3 class="leading-none text-gray-800 font-medium">
                                    {item.name}
                                </h3>
                                <p class="mt-1 text-sm text-gray-600">
                                    {item.description}
                                </p>
                            </div>
                        </div>
                        <span class="block absolute top-5 left-5 border peer-checked:border-[5px] peer-checked:border-indigo-600 w-4 h-4 rounded-full"></span>
                    </label>
                </li>
            {/each}
        </ul>
    </div>

</div>


