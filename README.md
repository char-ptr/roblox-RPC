# roblox-RPC

rich presence for roblox (finally)

this is pretty much automatic except that you need to be signed into roblox in firefox (cause im too lazy to impl in anything else)

## how does this work?
Roblox internally stores an integer at a static place in memory which correlates to what the current place id is, all this application does is read it. So this means that you **cannot** get banned for using this. 

## project structure 
- [`/src/`](./src) : Code base for the rpc part
- [`/roblox-offset/`](./roblox-offset) : gets the offset for current place
- [`/roblox-starter/`](./roblox-starter) : puppeteer to start roblox
- [`/roblox-updater/`](./roblox-updater) : automates starting and getting the offset and then writing into a file

## caveats
- i use puppeteer to launch roblox, cause they use some crap to make it really diffult otherwise, and frankly, im too lazy.
- roblox-starter is written using `DENO` not `NODE` please keep in mind if you try to run lol.
- requires you to be signed into roblox in firefox. (might add additional browsers later, or if anyone makes a pull request)
- every time roblox updates the address changes, so you either need to wait for update (might put it on a server), or update yourself.

### updater issues
if you try to run the updater and for whatever reason chrome doesn't allow you to open always, run the chr.reg and you should now be given the prompt
(if you're not on windows, do your own research for how to enable lol)


## image
![image](https://user-images.githubusercontent.com/44528100/216448240-e3f4024c-0048-4ea7-ac9f-778c8c5f44fd.png)
