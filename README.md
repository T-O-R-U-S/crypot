# A garbage implementation of an encryption algorithm I devised
### Also, it's broken. I pushed it to Github so I can continue work on my mac later in the day, I would not want normal people seeing this code :(
Basic idea:
- Take UTF-8 user-input string
- Take UTF-8 file
- For (index, char) in UTF-8 enumerated file:
    Return (charcode of char) * (charcode of user-input string\[index])

Why it doesn't work:
  - As it turns out, multiplying 100 by 100 returns a value GREATER than 255... however, I 
  cannot write to files using the standard library unless I am using utf-8. I have not
  found a way to encode utf-16 into utf-8 so I am processing it directly using u8 chars