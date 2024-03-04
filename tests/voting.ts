import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import {Voting} from "../target/types/voting";
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY} from "@solana/web3.js";

describe("voting", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const delayTimeCount = 1000;

    const program = anchor.workspace.Voting as Program<Voting>;

    let connection = anchor.AnchorProvider.env().connection;

    const USER_VOTED_ACCOUNT_PREFIX: string = "USER_VOTED";
    const VOTING_CONFIG_ACCOUNT_PREFIX: string = "VOTING_CONFIG";

    const mainSignerPubKey: PublicKey = anchor.AnchorProvider.env().wallet.publicKey;
    const ownerSignerKeypair: Keypair = Keypair.generate();
    const userSignerKeypair: Keypair = Keypair.generate();

    // PDA and Bumps
    let votingConfigPDA: PublicKey;
    let votingConfigBump: number;

    let userVotedConfigPDA: PublicKey;
    let userVotedConfigBump: number;

    const topic: string = "Best Crypto";
    const topicOptions: string[] = ["BTC", "ETH", "SOL"];

    it("setup signers accounts", async () => {
        await connection.requestAirdrop(ownerSignerKeypair.publicKey, 20 * LAMPORTS_PER_SOL);
        await delay(delayTimeCount);
        console.log("owner signer account: ", ownerSignerKeypair.publicKey.toBase58());
        console.log("owner signer account sol balance: ", (await connection.getBalance(ownerSignerKeypair.publicKey)) / LAMPORTS_PER_SOL);

        await connection.requestAirdrop(userSignerKeypair.publicKey, 20 * LAMPORTS_PER_SOL);
        await delay(delayTimeCount);
        console.log("user signer account: ", userSignerKeypair.publicKey.toBase58());
        console.log("user signer account sol balance: ", (await connection.getBalance(userSignerKeypair.publicKey)) / LAMPORTS_PER_SOL);
    });

    it("Create PDA Addresses!", async () => {
        [votingConfigPDA, votingConfigBump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from(VOTING_CONFIG_ACCOUNT_PREFIX),
                    Buffer.from(topic)
                ],
                program.programId
            );

        console.log("voting config account pda: ", votingConfigPDA.toBase58());
        console.log("voting claim config account bump: ", votingConfigBump);

        [userVotedConfigPDA, userVotedConfigBump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from(USER_VOTED_ACCOUNT_PREFIX),
                    votingConfigPDA.toBuffer(),
                    userSignerKeypair.publicKey.toBuffer()
                ],
                program.programId
            );

        console.log("user voted config account pda: ", userVotedConfigPDA.toBase58());
        console.log("user voted claim config account bump: ", userVotedConfigBump);

    });


    it("Initialize voting topic", async () => {
        const tx = await program.methods.initializeVotingTopic(topicOptions.length, topic, topicOptions)
            .accounts({
                payer: mainSignerPubKey,
                owner: ownerSignerKeypair.publicKey,
                votingConfig: votingConfigPDA,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([ownerSignerKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);
    });

    it("set voting enable", async () => {
        const tx = await program.methods.setVotingEnable(topic, votingConfigBump, true)
            .accounts({
                payer: mainSignerPubKey,
                owner: ownerSignerKeypair.publicKey,
                votingConfig: votingConfigPDA,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([ownerSignerKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);
    });

    it("Cast vote", async () => {
        const tx = await program.methods.castVote(topic, votingConfigBump, "BTC")
            .accounts({
                payer: mainSignerPubKey,
                user: userSignerKeypair.publicKey,
                votingConfig: votingConfigPDA,
                userVoted: userVotedConfigPDA,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            })
            .signers([userSignerKeypair])
            .rpc();
        console.log("Your transaction signature", tx);

        await delay(delayTimeCount);
    });

});

function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
