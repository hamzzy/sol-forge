
# SolForge

**A Solana development tool for fast, easy and reliable smart contract development.**

SolForge aims to provide a developer-friendly environment for Solana smart contract development, bridging the gap between the EVM tooling experience and Solana's unique ecosystem. It simplifies the workflow, automates repetitive tasks, and encourages best practices for writing and testing Solana programs with Anchor.

## Features

-   **Project Initialization:** Quickly set up a new Anchor project with a single command: `anchorforge init <project_name>`.
-   **Contract Compilation:** Build your Solana program using `cargo build-bpf` with a simple `anchorforge compile` command.
-   **Local Devnet:** Easily start and stop a local Solana development network with `anchorforge devnet start` and `anchorforge devnet stop` respectively.
-   **Deployment:** Deploy your compiled program to the local devnet with the `anchorforge deploy` command.
-   **Testing:** Run your Anchor program's tests with `anchorforge test`.
-   **Basic Scripting:** Automate tasks using Javascript or Typescript with `anchorforge script <script_name>`, this allows you to interact with your program using Javascript/Typescript.
-   **IDL and Typescript Bindings:** Generate IDL and Typescript types for your Solana programs for easy and type-safe interaction in scripts.

## Installation

1.  **Install Rust:** Make sure you have Rust installed on your system. If not, follow the instructions [here](https://www.rust-lang.org/tools/install).
2.  **Install Solana CLI:** Install the Solana CLI tools by following the steps [here](https://docs.solana.com/cli/install-solana-cli-tools).
3. **Install Node.js**: Install Node.js by following the instructions [here](https://nodejs.org/en/download/).
4. **Install yarn:** Install `yarn` by following the instructions [here](https://yarnpkg.com/getting-started/install).
5.  **Install SolForge:**

    ```bash
    git clone <your-repo-url>
    cd Solforge
    cargo install --path .
    ```

## Usage

Here's a quick rundown of the main commands:

### Project Initialization

To create a new Sol project:

```bash
Solforge init <your_project_name>
```

This command creates a folder with the name `<your_project_name>` and a basic Solana Sol project structure inside.

### Compilation

To compile your Solana program:

```bash
Solforge compile
```

This command compiles your program using the `cargo build-bpf` command.

### Local Devnet

To start the local Solana test network:

```bash
Solforge devnet start
```

To stop the local Solana test network:

```bash
Solforge devnet stop
```

**Note:** The devnet stop is not yet implemented, you will have to manually stop it for now.

### Deployment

To deploy your compiled program to the local test network:

```bash
Solforge deploy
```

### Testing

To run your Sol program's tests:

```bash
Solforge test
```

### Scripting

To run a Javascript or Typescript script:

```bash
Solforge script <your_script_name.js/ts>
```

Scripts can be created inside the `scripts` folder.

Inside the scripts, the following are available:

* `program_id`:  The address of your deployed program.
* `config`: The `Solforge.toml` configuration object.
* `anchor`: A pre-configured instance of `@coral-xyz/anchor`.
* `connection`: A pre-configured instance of `solana/web3.js`.

See the example below:

```javascript
    const {Connection, PublicKey} = require('@solana/web3.js');
    const anchor = require('@coral-xyz/anchor');
    const program = context.program_id;
    const config = context.config;
    console.log("Hello from AnchorForge!");
    console.log("Program ID:", program);
    console.log("Config:",config);
    console.log("Testing web3:", Connection, PublicKey);
    console.log("Testing anchor:", anchor);
```

## Configuration

SolForge uses a `Solforge.toml` file at the root of your project for storing configurations.
This file is automatically created when the project is initialized.

## Contributing

We welcome contributions! If you would like to contribute to the project, please refer to our [Contribution Guidelines](CONTRIBUTING.md) (create this file).

## License

This project is licensed under the [MIT License](LICENSE) (create this file).

## Acknowledgments

-   [Anchor](https://project-serum.github.io/anchor/): For providing the foundation for Solana smart contract development.
-   [Solana](https://solana.com/): For creating the Solana blockchain.
-  [Solana Labs](https://solanalabs.com/): For creating and maintaining the tooling around Solana.
-   [clap](https://github.com/clap-rs/clap): For handling the CLI parsing.
-   All other open source projects that we have used as dependecies.

## Roadmap

Here is a non-exhaustive list of things we plan on implementing:

-   [ ] Improve the testing framework.
-   [ ] Add support for a plugin system.
-   [ ] Improve the local development environment.
-   [ ] Provide more helper functions inside the scripts.
-   [ ] Add support for migrations.
-   [ ] Improve the deployment process.
-   [ ] Improve error handling and user experience.

## Support this project

If you like this project consider giving a star to the repository!
