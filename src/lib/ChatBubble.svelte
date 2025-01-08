<script lang="ts">
    import type { ChatMessage } from './types/message.type';
    export let message: ChatMessage ; // 聊天的信息

    console.log("-------message------",message);

    //export let username = "Support Agent";
    export let suport = true;

    //export let msg = "Thank you for the information. Allow me a moment to pull up your order details.";
    //export let timeSeen = "13:08";
    
</script>
  
<!-- 
<div class="messageContainer">
    <div class="bubble" style="align-self: {suport ? 'flex-end' : 'flex-start'};">
        <span class="name">{username}<br /></span>
        <span class="message">{msg}</span>
        <p class="seen">Time {timeSeen}</p>
    </div>
</div>
-->

<div class="chat-message my-2">
    <div class="flex items-end px-5 justify-{suport ? 'end': 'start'}">
       <div class="flex flex-col space-y-1  order-{suport ? 1 : 2} items-{suport ? 'end': 'start'}" style="max-width: 75%;">
        <p style="display: block;text-align: center;width: 100%; margin-bottom: 0; font-size: smaller;" >
          Time {message.createTime}
        </p>  
        <div class="px-4 py-2 rounded-md inline-bloc mx-1 {suport? 'bg-emerald-200 text-black' : 'bg-gray-300 text-gray-600'}">
            <span >
                {message.content}
            </span>
          </div>
       </div>
       <img src="{suport?"/longmao.png":"/jiqimao.jpg"}" alt="My profile" class="w-10 h-10 rounded-full order-1">
    </div>
 </div>
 
 <!-- 
 <div class="chat-message">
    <div class="flex items-end px-3 {suport ? 'justify-end': 'justify-start'}">
       <div class="flex flex-col space-y-1 order-1 {suport ? 'items-start': 'items-end'}" style="max-width: 60%;">
        <p style="display: block;text-align: center;font-size: small;width: 100%; margin-bottom: 0; ">Time {timeSeen}</p> 
          <div class="px-4 py-2 rounded-lg inline-block rounded-br-none bg-blue-600 text-white mx-1">
            <span >
                {msg}
            </span>
          </div>
       </div>
       <img src="https://images.unsplash.com/photo-1590031905470-a1a1feacbb0b?ixlib=rb-1.2.1&amp;ixid=eyJhcHBfaWQiOjEyMDd9&amp;auto=format&amp;fit=facearea&amp;facepad=3&amp;w=144&amp;h=144" alt="My profile" class="w-6 h-6 rounded-full order-2">
    </div>
 </div>
 -->


  <style>
  .bubble {
    background-color: #DCF8C6;
    border-radius: 10px;
    margin-bottom: 10px;
    padding: 6px;
    max-width: 60%;
    box-sizing: border-box;
  }
  
  .name {
    display: inline;
    position: relative;
    font-variant: small-caps;
    font-size: 1.25rem;
    text-decoration: underline;
    font-weight: bold;
    opacity: .65;
  }
  
  .name, .message, .seen {
    margin: .25em 1em .25em 1em;
  }
  
  .messageContainer {
    display: inline-flex;
    flex-direction: column;
    padding: 1em;
  }
  
  .message{
    display: block;
    margin: 10px 10% 10px 10%;
  }
  
  .seen{
    text-align: right;
    font-size: small;
    opacity: .5
  }
  
  </style>