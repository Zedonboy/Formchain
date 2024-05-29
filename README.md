# Formschain

Formschain is a Web3 Data Collection service designed to simplify the creation and management of forms for websites, particularly static sites that lack server-side functionality. By using Formschain, developers can easily set up contact forms, order forms, or email capture forms without needing to write any server-side code.

## Technical Summary:
Formschain is a data collection backend and API service that handles form submissions for websites. They provide an endpoint to which forms and data can be submitted, managing everything from spam filtering to email notifications and data storage. The service serves a wide range of clients, from freelancers and small businesses to large corporations like Amazon and IBM. its hosted on ICP Blockchain

## Problem Statement:
1. **No Server Code Required:** Traditionally, handling form submissions required setting up server-side code to process the data. Formschain eliminates this need by providing a ready-to-use backend, making it ideal for static sites that do not have server-side capabilities.

2. **Data Ownership** Data are saved in encrypted form inside blockchain. which removes single point of failure from centralized repositories

3. **Data Market** Data Stored with Formschain can be exchange for cryptocurrency tokens, means independent research or statistics firms can buy and sell data to Machine learning companies

4. **Forn Validation:** formschain offers built-in form validation and duplication checks, to protect forms from spam submissions.

5. **Email Notifications and Data Management:** The service automatically sends email notifications when a form is submitted and allows users to customize email templates. Submissions are stored in the Formschain dashboard, where users can view, analyze, and export data.

6. **Integration with Third-Party Services:** Formschain integrates with various third-party applications like Airtable, Mailchimp, Google Sheets, Slack, and more, making it easy to incorporate form data into existing workflows and tools.

7. **Ease of Use:** Setting up a form with Formschain is straightforward. Developers simply need to set their form's action attribute to a Formschain endpoint. This minimal setup makes it accessible for those with limited technical expertise.

Overall, Formschain is designed to make form handling and data collection simple and efficient, especially for static websites that require a quick and effective solution without the complexity of server-side development

Welcome to your new formchain project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with formchain, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd formchain/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
