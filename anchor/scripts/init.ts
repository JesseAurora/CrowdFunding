import * as anchor from '@coral-xyz/anchor'
import { CrowdFunding } from '../target/types/crowd_funding'
import idl from '../target/idl/crowd_funding.json'
import fs from 'fs'
import { log } from 'console'
import 'dotenv/config'
const { SystemProgram, PublicKey } = anchor.web3

const main = async (cluster: string) => {
    // Defines the clusters URLs
    const clusterUrls: any = {
        'mainnet-beta': 'https://api.mainnet-beta.solana.com',
        testnet: 'https://api.testnet.solana.com',
        devnet: 'https://api.devnet.solana.com',
        localhost: 'http://localhost:8899',
    }

    // Creates a connection to the cluster
    const connection = new anchor.web3.Connection(
        clusterUrls[cluster],
        'confirmed'
    )

    // Load the wallet from the deployer's keypair file
    const keypairPath = `${process.env.HOME}/.config/solana/id.json`
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf-8'))
    const wallet = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(keypairData))

    // Create a provider
    const provider = new anchor.AnchorProvider(
        connection,
        new anchor.Wallet(wallet),
        {
            commitment: 'confirmed',
        }
    )

    anchor.setProvider(provider)

    // Load the program
    const program = new anchor.Program<CrowdFunding>(idl as any, provider)
    const [programStatePda] = PublicKey.findProgramAddressSync(
        [Buffer.from('program_state')],
        program.programId
    )

    try {
        const state = await program.account.programState.fetch(programStatePda)
        console.log(`Program already initialized, status: ${state.initalized}`)
    } catch (error) {
        const tx = await program.methods
            .initalize()
            .accountsPartial({
                programState: programStatePda,
                deployer: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc()

        await connection.confirmTransaction(tx, 'finalized')
        console.log('Program initialized successfully.', tx)
    }
}

const cluster: string = process.env.NEXT_PUBLIC_CLUSTER || 'localhost'
main(cluster).catch((error) => console.log(error))
