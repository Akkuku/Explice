# explice
explice is a CLI tool for creating reusable GPT assistants with chat completions in your terminal.

## How to use
Before the first real use, app must initialize config with OpenAi key:
```shell
explice init
```

Then you are ready for your first chat session:
```shell
explice chat
```

## Features
- [x] Initialize chat loop, with history for the session,
- [x] Add assistants with different models, behaviours, and parameters,
- [x] Insert file content into the prompts,  
    `: explain this code: {main.py}?`  
    `: how to use this function: {TryYourself.cs 21:37}?`  
    `: extend this docker compose with postgres service: {home/usr/my_project/docker-compose.yml}`
- [x] Path completion on tab or right arrow click