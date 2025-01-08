<script lang="ts">
    import { chatMessages } from '../store';
    import Message from './Message.svelte';
    import { beforeUpdate, afterUpdate } from 'svelte';

    let newMessage: string = '';
    let timeline : HTMLElement;
    let autoscroll: boolean;

    beforeUpdate(() => {
        autoscroll = timeline && (timeline.offsetHeight + timeline.scrollTop) > (timeline.scrollHeight - 20);
    });

    afterUpdate(() => {
        if (autoscroll) {
            timeline.scrollTo(0, timeline.scrollHeight);
        }
    })

    function sendMessage(newMessage: string) {
        $chatMessages = [...$chatMessages, {
            id: 123,
            recipient: 223,
            message: newMessage,
            timestamp: new Date(),
            }
        ];
    }

    function sendKeyHandler(event) {
        if (event.key === 'Enter') {
            const text = event.target.value;
            if (!text) {
                return;
            }

            sendMessage(text);

            event.target.value = '';
        }
    }
</script>

<main class="flex flex-col h-screen">
    <header class="flex justify-between w-full bg-blue-500 text-white">
        <div class="flex p-2">
            <span class="material-icons text-white py-4">west</span>
            <div class="px-2">
                <img class="w-12 h-12 rounded-full" src="https://images.unsplash.com/photo-1438761681033-6461ffad8d80" alt="avatar">
            </div>
            <div class="px-2">
                <h2>Lara Croft</h2>
                <p class="text-xs">Last seen at Yesterday</p>
            </div>
        </div>
        <div class="py-5">
            <span class="material-icons px-2 transform -rotate-45">attach_file</span>
            <span class="material-icons px-2">more_vert</span>
        </div>
    </header>

    <section class="flex-grow overflow-y-auto" bind:this={ timeline }>
        {#each $chatMessages as chat}
            <Message message="{ chat }"></Message>
        {/each}
    </section>
    <footer class="flex flex-row  items-center  bottom-0 my-2 w-full">
        <div class="ml-2 flex flex-row border-gray items-center w-full border rounded-3xl h-12 px-2">
            <button class="focus:outline-none flex items-center justify-center h-10 w-10 hover:text-red-600 text-red-400 ml-1">
                <span class="material-icons px-2">mic</span>
            </button>
            <div class="w-full">
                <input
                    type="text"
                    id="message"
                    bind:value={ newMessage }
                    on:keydown={ sendKeyHandler }
                    class="border rounded-2xl border-transparent w-full focus:outline-none text-sm h-10 flex items-center"
                    placeholder="Type your message....">
            </div>
            <div class="flex flex-row">
                <button class="focus:outline-none flex items-center justify-center h-10 w-8 hover:text-blue-600  text-blue-400">
                    <span class="material-icons px-2">attach_file</span>
                </button>
                <button
                    id="capture"
                    class="focus:outline-none flex items-center justify-center h-10 w-8 hover:text-green-600 text-green-400 ml-1 mr-2">
                    <span class="material-icons px-2">photo_camera</span>
                </button>
              </div>
            </div>
            <div class="ml-3 mr-2">
                <button
                    id="other"
                    on:click={() => sendMessage(newMessage) }
                    class="flex items-center justify-center h-10 w-10 rounded-full  hover:text-blue-900 text-blue-500 text-white focus:outline-none">
                    <span class="material-icons px-2">send</span>
              </button>
            </div>
    </footer>
</main>