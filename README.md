# Rustor
A .txt wordlist checker made in RUST. 

<br />
<br />

# About
Say you have a text file full of words. This script will allow you to check the file for words you have in a list, if they don't exist in words.txt fle, it will add the word(s) to the file and then sort them all alphabetically.

<br />
<br />

# Credits
> Made with ChatGPT and some tweaks by me.

ChatGPT as of Jan9 - Jan30 has gotten worse and almost unbareable compared to when it was released. So this was actually a challenge to put together. And I had to get pretty creative with my prompts. It also would in the middle of outputting code just stop, I would be able to type "continue" and it would know EXACTLY where it stopped at and continue writting the code. As of Jan9-Jan30 it doesn't do that and will just start re writing the entire code again thus breaking at the same spot again. You now have to be very specific and mention where to continue from, and it will make an attempt at doing so with no formatting or anything. No tabs and all that either.

Overall, chatGPT has gotten pretty bad lately and I don't think I may continue tinkering with it when involving code. Maybe just stick with questions that can teach me stuff instead of trying to get it to make some code for me to work off of.



<br />
<br />

# Bugs
- Sometimes at the end of the file on the last word, One of the words if it isn't found in words.txt will be appended to the end of the last word over and over again untill you remove them and run the script again. I don't know why this happens or how it happens but If you keep running the script and it keeps saying "adding <word> to list" each time the script is ran, this is what I'm talking about. If you know Rust and can fix this or if you would love to contribute, feel free to tinker away!
