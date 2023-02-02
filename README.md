# roblox-RPC

rich presence for roblox (finally)

this is pretty much automatic except that you need to be signed into roblox in firefox (cause im too lazy to impl in anything else)

## project structure 
- [`/src/`](./src) : Code base for the rpc part
- [`/roblox-offset/`](./roblox-offset) : get's the offset for current place
- [`/roblox-starter/`](./roblox-starter) : puppeteer to start roblox
- [`/roblox-updater/`](./roblox-updater) : automates starting and getting the offset and then writing into a file

## caveats
- i use puppeteer to launch roblox, cause they use some crap to make it really diffult otherwise, and frankly, im too lazy.
- roblox-starter is written using `DENO` not `NODE` please keep in mind if you try to run lol.
- requires you to be signed into roblox in firefox. (might add additional browsers later, or if anyone makes a pull request)
