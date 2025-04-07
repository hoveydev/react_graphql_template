# _CHANGE_ME_PROJECT_SLUG

The main folder of your new application/website (this will have been renamed if you already ran `rgql setup`)

This folder contains a global `package.json` configured for basic `eslint` and `prettier` rules. It also contains a `.projectroot` file. *Please do not remove this! It is required for the RGQL CLI to work properly.*

This folder also contains `client` and `server` folders which contain all the logic for react and GraphQL respectively. These applications can be run separately with `npm` or, they can be run together with the use of the RGQL CLI. Please see the [repository documentation](../README.md) for more information on how to run it.

## client

This folder contains all of the code for the frontend (ReactJS)

Aside from all of the `vite` and `typescript` configuration files, this folder also contains mostly boilerplate `React` code (included in the `src` folder). An additional `.env` file has been added to determine the `PORT` needed for running the `client` application. This value will change when specifying a specific port with the `rgql start` command.

### src

This folder contains mostly boilerplate `React` code, but also has some additional scaffolding that may be utilized during the development process.

- `assets` - folder that contains all of the media associated with the client including images, videos, audio, etc.
- `components` - this may or may not be utilized but was included as a foundation for a custom component structure. This folder is currently empty.
- `graphql` - folder that would contain any frontend specific logic for `GraphQL`.
- `pages` - folder structure for all of the web pages contained in the project. Currently, a simple error view exists, but this can be expanded to eventually organize all of your pages.
- `styles` - would be used to organize all of your CSS and style related files.

>
> Note: The above folders are included only as an option for organization, but these can easily be removed and substituted for another structure of your choice.
>

- `App.tsx` - the entry point for `GraphQL` and the first page of the application.
- `index.css` - the main entry point for CSS files. This is also where `tailwindCSS` is imported.
- `main.tsx` - the injection point for `Apollo Client` (GraphQL) and `RouterProvider` (react-router-dom).
- `router.tsx` - where all the application routes are defined.

## server

The server folder contains all of the code for the backend (GraphQL)

Aside from the `typescript` configuration file, there is a `codegen.ts` file which is used to generate types after `rgql setup` is run.

### src

The `src` folder contains all the boilerplate code needed for a simple `GraphQL` implementation.

- `datasources` - contains all of the files used to retrieve data from all datasources.
- `schemas` - separated into `types` and `query.gql` and is used for defining all `gql` files. This eventually could expand to add `mutation.gql`
- `context.ts` - context that defines all datasources in one place for easy type conformance. This is used in conjunction with the `codegen.ts` file
- `index.ts` - the entry point for the `Apollo Server`. This file compiles all of the resources and starts the server.
- `resolvers.ts` - defines all the resolvers used for each query.
- `types.ts` - the fully generated types needed for the entirety of the `GraphQL` project. This will need to be generated again if any code changes are made to the `GraphQL` server.

>
> Note: `types.ts` will only be seen after you run `rgql setup` for the first time
>

# Docs

The folder that contains all of the more specific documentation for the repo (it’s also where this one lives 🙂)

This folder also contains an `images` folder which contains all of the image-type files needed for any/all of the documentation.

# scripts

The folder that contains all the scripts for the RGQL CLI

Since the CLI is written in Rust, you will notice a `Cargo.toml` and `Cargo.lock` file as well as a `src` folder

## src

The folder that contains all the Rust files needed for the CLI. Learn more about these files and what they do [here](RGQL.md)
