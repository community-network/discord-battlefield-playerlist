# Server playerlist bot

Shows the playerlist of your server in Discord.

![image](https://user-images.githubusercontent.com/22680656/213012714-c458dd08-a61e-45c4-9edf-9e5179354d19.png)

```py
# battlefield server to follow
server_name = '[BoB]#1 EU All CQ'
# discord token to use
token = 'SECRET_DISCORD_TOKEN'
# discord channel id it has to post the messages to
channel = 106456457126165306
# messages it will edit for showing serverinfo
# (used if bot is restarted, should be empty on first startup)
messages = [
    1064877261444087869,
    1064877262756909086,
    1064877263901958194,
    1064877265051201639,
    1064877267001548860,
]
# game to use, can be Bf4 or Bf1 (capital letter B)
game = 'Bf4'
```
