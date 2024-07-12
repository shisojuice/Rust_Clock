import init, { clock_png } from './rust_clock.js';

async function run() {
    await init();
    setInterval(()=>{
        const now = new Date();
        const h = now.getHours();
        const m = now.getMinutes();
        const s = now.getSeconds();
        document.getElementById("mainImg").src = clock_png(h,m,s);
    }, 1000);
}
run();

