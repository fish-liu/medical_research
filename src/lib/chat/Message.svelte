<script lang="ts">
    export let message;
</script>

<section class="pt-2">
    <article class="px-3 py-1 flex" class:justify-end="{ message.recipient === 223 }">
        <div class="shadow rounded w-3/4" class:bg-blue-100="{ message.recipient === 223 }" class:bg-white="{ message.recipient !== 223 }">
            {#if message.attachment }
                <img class="rounded-md p-1" src="{ message.attachment.src }" alt="{ message.attachment.type }">
            {/if}
            <h1 class="px-3">{ message.message }</h1>
            <p class="text-right text-xs text-gray-500 p-1"> { message.timestamp }</p>
        </div>
    </article>
</section>

<style lang="postcss">

</style>