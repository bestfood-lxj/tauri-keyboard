const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {});



async function speak(options) {
    return invoke("plugin:tts|speak", {
        payload: {
            text: options.text,
            language:null,
            voiceId: null,
            rate: 1.0,
            pitch: 1.0,
            volume: 1.0,
            queueMode: "flush",
        },
    });
}
async function getVoices(language) {
    const { voices } = await invoke("plugin:tts|get_voices", {
        payload: { language: null },
    });
    return voices;
}
window.debug_dev={
    runGetVoices:()=>{
        getVoices().then(res=>{
            document.querySelector("#voicesId").innerText=JSON.stringify(res)
        })
    },
    runSpeak:()=>{
        speak({text: "one two three",})
    },
}