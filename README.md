# Discord Dice Roller
A simple dice roller bot made in rust with [serenity](https://github.com/serenity-rs/serenity).

## How to use

The bot uses Discord's slash commands.   
Base command to roll dice is `/roll`  

__Here are some examples with the behavior you can expect:__   

- `/roll 1d20` or `/roll d20` will throw one die of 20 faces
- `/roll 4d6` will throw 4 dice of 6 faces

**Threshold system:** *Can be added optionally*

- `/roll <4` will throw one die of 20 faces, with a success rate that must be less or equal to 4.
- `/roll 7d6<3` will throw 7 dice of 6 faces. If a die value is less or equal 3, the throw is a success.   
When throwing multiple dice, the bot's answer will show how many rolls were successful out of the total number of rolls.

## How to run
- Create your sqlite database *(A sqlite.db file)*
- Add an `.env` file to the root where the project will run with the following content :
```
TOKEN="your_discord_token_here"
DATABASE_URL="file_name.db"
```
To get a bot token, check the [discord developers documentation](https://discord.com/developers/applications/) for more help

## Internationalization
You can own your own languages files in the `locales` folder. The language must be supported by discord, and the file name must be the [language code used by discord](https://discord.com/developers/docs/reference#locales).
   
**To note:** Its my first time coding with Rust. The code is surely perfectible.
