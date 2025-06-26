



### Overview

CrowdFunding is a decentralized  platform built on the Solana  blockchain. 

It enables users to create, manage, and contribute to crowdfunding campaigns in a trustless and transparent manner. The  program is written in Rust using the Anchor framework, while the  frontend is developed with Next.js and TypeScript. also integrate Phantom Wallet for seamless user authentication.



### Program Features

```shell
// choose user 
// deployer who deploy this program to solana
// creator  who create campaigns
toggleProvider = (user: 'deployer' | 'creator')

// deployer can get platform_fee when user withdraw 
// deployer can change platform_fee default 5% 
it('update a platform fee', async () => {
  provider = toggleProvider('deployer')

// when user use wallet like Phantom connected
// user can create update delete self campaign
// and user can withdraw others peoples donations to self wallet
it('create a campaign', async () => {
  provider = toggleProvider('creator')

it('update a campaign', async () => {
  provider = toggleProvider('creator')
  
it('delete a campaign', async () => {
  provider = toggleProvider('creator')
  
it('withdraw from campaign', async () => {
  provider = toggleProvider('creator')


// donate to other people
it('donate to campaign', async () => {
  provider = toggleProvider('deployer')
```



### Frontend

```shell
npm install 
npm run build
npm run dev
```



### Anchor Build

```shell
anchor keys sync
anchor build
```



### Anchor Deploy

```shell
solana config set -ud
solana airdrop 5
anchor deploy
```

