import init, { clock_png } from './rust_clock.js';

async function run() {
    await init();
    const mainImg = document.getElementById('mainImg');
    (function loop() {
        const now = new Date();
        const h = now.getHours();
        const m = now.getMinutes();
        const s = now.getSeconds();
        mainImg.src = clock_png(h, m, s);
        setTimeout(loop, 1000 - Date.now() % 1000);
    })();
}
run();

