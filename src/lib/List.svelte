<script lang="ts">
    const user = {
        id: 123,
        name: 'Lara Croft',
        avatar: 'https://images.unsplash.com/photo-1438761681033-6461ffad8d80',
        lastMessage: {
            id: 333,
            message: 'What does the fox say?',
            timestamp: '11:23',
        },
        undreadMessages: 2,
    };
</script>

<section class="flex m-2">
    <div class="w-2/12 py-3">
        <img class="w-12 h-12 rounded-full" src="{ user.avatar }" alt="avatar">
    </div>

    <article class="w-10/12 py-2 border-b border-gray-200">
        <div class="flex justify-between">
            <h2 class="font-semibold">{ user.name }</h2>
            <p class="text-xs text-gray-400">{ user.lastMessage.timestamp } </p>
        </div>
        <div class="flex justify-between items-center">
            <p class="text-gray-500">{ user.lastMessage.message }</p>
            {#if user.undreadMessages}
                <p class="rounded-full bg-blue-500 text-white text-xs w-5 h-5 flex items-center justify-center">
                    { user.undreadMessages }
                </p>
            {/if}
        </div>
    </article>
</section>