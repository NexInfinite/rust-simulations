[![AI:W WRITE](https://img.shields.io/badge/AI-WRITE-blue?style=flat&logoSize=auto&labelColor=13171f)](https://0rwx.com)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/nexinfinite/rust-simulations)
![GitHub language count](https://img.shields.io/github/languages/top/nexinfinite/rust-simulations)

# Rust Simulations

This repository holds simulations and animations I've made in rust using bevy. This project was previously written in MacroQuad and later upgraded to bevy as it was a better fit for the work I am doing. Simulations made in this project are mainly for fun, however I am aiming to make a library that is easy to add new mathematical functions. Eventually I am to allow a series of differential equations to be read and converted into animations easily.

# Screenshots

<details>
  <summary>Simulation in Progress</summary>
  <img width="1280" height="720" alt="image" src="https://github.com/user-attachments/assets/bfd6fa5f-e4e4-42db-beb1-6dcd54f630a8" />
</details>
<details>
  <summary>Options Modal</summary>
  <img width="1280" height="720" alt="image" src="https://github.com/user-attachments/assets/f46c7d20-fc88-4b75-85e1-c296be2b7cea" />
</details>




# AI Usage

I've tagged this as "Write" from the [0rwx](https://0rwx.com) framework, however it is sometimes a grey area between "Read" and "Write". The framework aims an overview of how much AI wrote a project, however these things are much more granular in my opinion. 
> To err on the side of caution, I am choosing to use "Write" even if most of this is written by hand. 

There will eventually be problems, detailed below if they occur, in which I'd rather have an agent solve them than spend countless hours and burning myself out. This is a project for fun, not for production or work, and so I'd rather keep the use of AI to a minimal: but I can't guarantee it will never be used.

## Usages of AI Write:

- Used in copilot reviews, sometimes I'll let it submit the code
- Used to rework how movement was handled so that entities moved in lock step, not lagging behind. This was done in parallel with Copilot in VSCode, it gave ideas and wrote some areas and I implemented others.
- Used when migrating over to newer versions of bevy as some documentation changed that I could not figure out, once the agent wrote it there was no point in rewriting, however I did read it.
- Used for understanding bevy ui and writing some handlers for clicking outside the options popup, this would have taken many, many hours of head banging - with claude it only took 2 hours of head banging.
