# Dot32 intro

The port of my love2d intro into rust. 

Press `ctrl`+`r` on windows or `cmd`+`r` on mac to replay the animation. (This is currently dissabled, but it can be reenabled by setting RESTARTABLE in lib.rs to true)

<img width="912" alt="Screen Shot 2022-05-15 at 13 48 22" src="https://user-images.githubusercontent.com/61964090/168459582-38e43c84-8312-462d-8010-85e50251589c.png">

![image of intro animation](https://user-images.githubusercontent.com/61964090/168785042-728b8934-35aa-4af1-9c49-8634f00d8ce3.gif)

Installation of the intro is as simple as 
```toml
[Dependancies]
dot32_intro = { git = "https://github.com/Dot32IsCool/dot32-intro-rs"}
```
Add `use dot32_intro::*;` to the top of your file and add the `Intro` plugin to your bevy app. The order in which plugins/systems are added appears to matter, so play around with that. <br>

Lastly, add a folder called fonts to your assets folder, with PT_Sans inside if you want the text to show (which you should, what's the point otherwise)

I've no idea why you might want to use my intro library but maybe this could be useful for anyone wishing to make their own intro library 🗿