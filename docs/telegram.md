# Telegram

Toby's Telegram bot can notify you about success/failure of jobs.

## Notifications

### Job Started

> ⌛️ Job **#29** for project **foo** triggered by webhook (travis)...

### Job successful

> ☀️ Job for project **foo** completed successfully.

### Job failed

> 💔 Job for project **foo** failed.  
> `Command failed: No such file or directory (os error 2)`

## Guide

### Step 1: Obtain a token

Using the [BotFather](https://t.me/BotFather) create a new bot (`/newbot`).
This will give you a token with which toby can access Telegram's API.

### Step 2: Edit Configuration

Add the newly obtained token to your toby config in `/etc/toby/toby.toml`:

The `[telegram]` section should already exist in your config file and is simply commented out. Remove the commenting and change the token.

```
# The bot can notify you about success/failure of jobs through telegram.
[telegram]
token = "YOUR BOT TOKEN GOES HERE"
```

### Step 3: Authorize chat

Run the command `toby telegram-setup`. It will give you a command that you can send to your bot in whatever chat you would like to receive notifications (requires adding the bot to the chat first).

Follow the instructions from the command. After the command completes successfully you need to restart the `tobyd` process.