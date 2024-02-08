# MacOS Auto Clicker

Allows you to autoclick

![Captura de Tela 2024-02-08 às 02 50 06](https://github.com/alysonvilela/rust-autoclicker-tauri/assets/22202745/0e5d18ef-edbb-4021-884a-f429ce20d92e)

## Roadmap
- [X] Auto click given a start and end position
- [ ] Define a interval between start/end
- [ ] Stop the app by clicking in an option
- [ ] Randomize next positions
- [ ] Automatically stop given a selected time


## Tech
This app was built with Tauri ecossystem.
- Rust Lang
- Vite - reactjs module with typescript
- Lucide React

## How to test
You need Node and Cargo installed on your machine.

- Clone the project
- Install app deps
  - `pnpm install`
- Run the project
  - `pnpm tauri dev`
 
## Build
With all the dependencies installed (like how to test section), just run `pnpm tauri build`
