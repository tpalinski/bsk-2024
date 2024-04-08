# Computer systems Security - electronic signature
Implementation of XADES compliant electronic signature system in rust with leptos

## Description
This is an implementation of a mock system allowing users to sign documents and verify other users' signatures. The system consists of two parts, a mock Trusted Third Party server responsible for registering user and storing their RSA keys and a client application allowing users to interact with the system

## Requirements
Both applications require cargo + rust to run. Additionally, [surrealdb](https://github.com/surrealdb/surrealdb) needs to be installed on the machine running the ttp server

## Running the app
Currently, production build is not supported yet. To run the system, both `client` and `ttp-server` need to be running. You can start them using `npm run dev` in their respective directories

## Usage
First step is to create an account on [ttp-server page](http://localhost:3000) and generate keys. Then, you are able to log in to [client app](http://localhost:2137), which in turn allows you to sign and verify signatures.
