import{r as l,u as D,j as e,T as M,a as W,S as P,b as Y,l as E,d as $,w as A,p as O}from"./index-DktWzuCK.js";/* empty css              */import{b as I}from"./buy-token-COKkF4j7.js";const U="/rugPullCasino/assets/menu-games-BtoIk64x.png",R="/rugPullCasino/assets/header-modal-classic-fBrCVDio.png",q="/rugPullCasino/assets/header-modal-daily-aKDi6lDP.png",B="/rugPullCasino/assets/header-modal-weekly-CU66eEYY.png",C="/rugPullCasino/assets/accept-xAwI-Yti.png",F=({modalOpen:n,setModalOpen:o,onClose:c,message:r,selectedGame:t,setSelectedGame:i,showTicket:m,userAddress:g})=>{function a(y){return localStorage.getItem(y)}const[b,d]=l.useState(!1),[v,h]=l.useState(!1),S=a("referrerWallet"),{value:L,sendPlayDailyLottery:p}=D(S),u=()=>{d(!1),h(!1),i(null),o(!1)},x=()=>{const y=document.querySelector(".modal-overlay"),j=document.querySelector(".modal-content");y&&j&&(j.classList.add("slide-out"),setTimeout(()=>{o(!1),d(!1),c()},300))};return n?e.jsxs("div",{className:"modal-overlay",children:[e.jsx("button",{className:"modal-close",onClick:x,children:"×"}),e.jsxs("div",{className:"modal-content",children:[e.jsx("div",{className:"modal-title",children:t==="Classic Lottery"?e.jsx("img",{src:R,alt:"Mark"}):t==="Daily Lottery"?e.jsx("img",{src:q,alt:"Mark"}):t==="Weekly Lottery"?e.jsx("img",{src:B,alt:"Mark"}):""}),b?t!=="Classic Lottery"?e.jsx("div",{className:"modal-content",children:e.jsxs("div",{className:"modal-info",children:[m?e.jsx("img",{src:t==="Daily Lottery"?M:W,alt:"ticket",className:"ticket-img"}):e.jsx(P,{}),e.jsx("button",{className:"button-accept",onClick:()=>{u()},children:e.jsx("img",{src:C,alt:"Accept",className:"accept-img"})})]})}):v?e.jsx("div",{className:"modal-content",children:e.jsxs("div",{className:"modal-info",children:[e.jsx("img",{src:Y,alt:"ticket",className:"ticket-img"}),e.jsx("button",{className:"button-accept",onClick:()=>{u()},children:e.jsx("img",{src:C,alt:"Accept",className:"accept-img"})})]})}):e.jsx("div",{className:"modal-content",children:e.jsxs("div",{className:"modal-info",children:[e.jsx("p",{className:"modal-message",children:"Put in your numbers"}),e.jsx("input",{type:"number",className:"modal-input"}),e.jsx("button",{className:"button-accept",onClick:()=>{h(!0)},children:e.jsx("img",{src:C,alt:"Accept",className:"accept-img"})})]})}):e.jsx("div",{className:"modal-info",children:t==="Daily Lottery"&&g===""?e.jsxs(e.Fragment,{children:[e.jsx("p",{className:"modal-message",children:"To play, you need to have your wallet connected."}),e.jsx("button",{className:"button-accept",onClick:()=>{u()},children:e.jsx("img",{src:C,alt:"Accept",className:"accept-img"})})]}):t==="Daily Lottery"&&g!==""?e.jsxs(e.Fragment,{children:[e.jsx("p",{className:"modal-message",children:r()}),e.jsxs("p",{className:"modal-message",children:[L.data_burn_percentage,"% of the Jackpot will be used to burn RPC tokens"]}),e.jsx("button",{className:"button-token",onClick:()=>{t==="Daily Lottery"&&p(),d(!0)},children:e.jsx("img",{src:I,alt:"BUY TOKEN",className:"buy-token-img"})})]}):""})]})]}):null},J=({modalOpen:n,setModalOpen:o})=>{const c=()=>{const r=document.querySelector(".modal-overlay"),t=document.querySelector(".modal-content");r&&t&&(r.classList.add("fade-out"),t.classList.add("slide-out"),setTimeout(()=>{o(!1)},300))};return n?e.jsxs("div",{className:"modal-overlay",children:[e.jsx("button",{className:"modal-close",onClick:c,children:"×"}),e.jsx("div",{className:"modal-content modal-referral",children:e.jsxs("div",{className:"modal-info",children:[e.jsx("div",{className:"header-modal",children:"Connect the wallet"}),e.jsx("hr",{}),"You have to connect the wallet to be able to play"]})})]}):null},z=()=>{function n(s){return localStorage.getItem(s)}const[o,c]=l.useState(!1),[r,t]=l.useState(!1),[i,m]=l.useState(null),g=n("referrerWallet"),{value:a,userAddress:b}=D(g),[d,v]=l.useState(""),[h,S]=l.useState(""),[L,p]=l.useState(!1),u=s=>{m(s),c(!0),p(!1)};let x="";x=n("referrerWallet"),l.useEffect(()=>{if(a&&a.data_next_lottery){const s=new Date(a.data_next_lottery*1e3),N=s.getUTCDate().toString().padStart(2,"0"),f=(s.getUTCMonth()+1).toString().padStart(2,"0"),k=s.getUTCFullYear(),w=`${N}/${f}/${k}`,T=s.getHours(),_=s.getMinutes();v(w),S(`${T}:${_}`)}},[a]),l.useEffect(()=>{a&&a.data_buyed_tickets&&p(!0)},[a.data_buyed_tickets]);const y=()=>{c(!1),m(null)},j=()=>{switch(i){case"Classic Lottery":return"You are playing the classic lottery";case"Daily Lottery":return`You are playing the daily lottery, Price: $ ${a.data_play_amount} Ton`;case"Weekly Lottery":return"You are playing the weekly lottery";default:return"You are playing the slot machine"}};return e.jsxs("div",{className:"menu-games",children:[e.jsx("img",{src:U,alt:"MENU GAMES",className:"sub-title menu-games-sub-title"}),E.map(s=>{var N,f,k;return e.jsxs("div",{className:`container-menu-game  ${s.disabled?"disabled-game":""}`,children:[e.jsxs("div",{className:"info-menu-game",children:[e.jsx("img",{className:"img-menu-game",src:s.name==="Daily Lottery"?$:A}),e.jsxs("div",{className:"info",children:[e.jsx("p",{className:"game-summary",children:s.summary}),s.name==="Daily Lottery"?e.jsxs(e.Fragment,{children:[e.jsxs("p",{className:"data-game",children:["Jackpot: $ ",a.data_balance," Ton"]}),e.jsxs("p",{className:"data-game",children:["Closing date: ",d]}),e.jsxs("p",{className:"data-game",children:["Time remaining: ",h]}),e.jsxs("p",{className:"data-game",children:["Winner Date: ",(N=a.data_winner)==null?void 0:N.date]}),e.jsx("p",{className:"data-game",children:"Winner Address: "}),e.jsx("p",{className:"data-game url-data-game",children:(f=a.data_winner)==null?void 0:f.address}),e.jsxs("p",{className:"data-game",children:["Winner Jackpot: ",(k=a.data_winner)==null?void 0:k.jackpot]}),e.jsxs("p",{className:"data-game",children:["Your tickets: ",a.data_buyed_tickets]}),e.jsx("p",{className:"data-game",children:"Referred wallet:"}),e.jsx("p",{className:"data-game url-data-game",children:x})]}):""]})]}),e.jsx("button",{className:"button-play-little",onClick:()=>u(s.name),children:e.jsx("img",{src:O,alt:"PLAY",className:"img-play-little",width:"30%"})})]})}),e.jsx(F,{modalOpen:o,setModalOpen:c,onClose:y,title:i||"",message:j,selectedGame:i,setSelectedGame:m,showTicket:L,userAddress:b}),e.jsx(J,{modalOpen:r,setModalOpen:t})]})};export{z as default};
