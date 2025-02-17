<script lang="ts">
    import "../app.css";
    import { chatType } from "$lib/store";
    import Chat from '$lib/Chat.svelte'
    import ChatIndex from '$lib/ChatIndex.svelte';

</script>

<div>
    {#if $chatType == ''  }
        <ChatIndex/>
    {/if}

    {#if $chatType != ''}
        <Chat />
    {/if}
</div>

<!-- 
<aside class="flex grid-cols-1 md:grid-cols-2">
    <div class="w-full md:w-1/4">
        <List></List>
    </div>
    <div class="hidden md:w-3/4 md:block">
        <Timeline></Timeline>
    </div>
</aside>

-->

