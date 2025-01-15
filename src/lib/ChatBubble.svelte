<script lang="ts">
    import type { Message4Chat } from './types/message.type';
    export let message: Message4Chat ; // 聊天的信息
    console.log("-------message------",message);
    export let suport = true;

    function parseMdstrong(str:string) {
        var strong_text = str.match(/(\*\*|__)(.+?)(\*\*|__)/g);//匹配加粗文本，正则中(.+?)的？是为了避免过度匹配把多个加粗匹配成一个
        if (strong_text != null) {// 如果有这个样式
            for (var i = 0; i < strong_text.length; i++) {
                str = str.replace(strong_text[i], "<strong>" + strong_text[i].match(/[^(**)^(__)]+/g)[0] + "</strong>");// 替换成 html 标签
            }
        }
        return str;
    }
</script>

<div class="chat-message my-2">
    <div class="flex items-end px-5 justify-{suport ? 'end': 'start'}">
       <div class="flex flex-col space-y-1  order-{suport ? 1 : 2} items-{suport ? 'end': 'start'}" style="max-width: 75%;">
        <p style="display: block;text-align: center;width: 100%; margin-bottom: 0; font-size: smaller;" >
          Time {message.createTime}
        </p>  
        <div class="px-4 py-2 rounded-md inline-bloc mx-1 {suport? 'bg-blue-500 text-white' : 'bg-gray-50 text-black'}">
            <span style="white-space: pre-wrap;">
                {@html parseMdstrong(message.content)}
            </span>
          </div>
       </div>
        <!--
       <img src="{suport?"/longmao.png":"/jiqimao.jpg"}" alt="My profile" class="w-10 h-10 rounded-full order-1">
        -->
    </div>
 </div>

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