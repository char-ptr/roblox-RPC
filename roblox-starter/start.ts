import puppeteer from "https://deno.land/x/puppeteer@16.2.0/mod.ts";
import { sleep } from "https://deno.land/x/sleep@v1.2.1/mod.ts";
import {ensureDir} from "https://deno.land/std@0.175.0/fs/mod.ts";
let first_time = true;
await ensureDir("./data").then(()=>{

    console.log("disabling first time");
    first_time = false;
})

const browser = await puppeteer.launch({headless: false, args:["--window-size=350,350"], userDataDir: "./data"});
const page = await browser.newPage();
await page.setCookie({
    name: ".ROBLOSECURITY",
    value: Deno.args[0],
    domain: ".roblox.com",
    expires: 2618604509000,
    path: "/",
    httpOnly:true,
    sameSite:"None",
    secure:true,
});
await page.goto(`https://www.roblox.com/games/${Deno.args[1]}`);
console.log("Waiting for button");

let but = await page.waitForSelector("button[data-testid='play-button']",{ timeout: 12e3 });
await sleep(2)
console.log("Clicking button");
await but?.click();
await but?.click();
console.log("if first time press allow always")
await sleep(first_time ? 60 : 4);
browser.close();