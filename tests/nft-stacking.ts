import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  generateSigner,
  keypairIdentity,
  KeypairSigner,
  percentAmount,
  signerIdentity,
  Umi,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { NftStacking } from "../target/types/nft_stacking";

const airdropSOL = async (
  to: anchor.web3.PublicKey,
  provider: anchor.AnchorProvider,
  amount: number
) => {
  try {
    const tx = await provider.connection.requestAirdrop(
      to,
      anchor.web3.LAMPORTS_PER_SOL * amount
    );

    await provider.connection.confirmTransaction(tx, "confirmed");
  } catch (e) {
    console.log(`U got an error while trying to airdrop 'SOL: ${e}`);
  }
};

const nftSetup = async (
  provider: anchor.AnchorProvider,
  user: anchor.web3.Keypair
) => {
  // Mint acc
  // NFT ATA
  // Create NFT
  // collection
  // metadata
  // edition (Master Account)
  try {
    const umi = createUmi(provider.connection);
    let nftMint = generateSigner(umi);
    let collectionMint = generateSigner(umi);

    let createrKeypair = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
    let createrSigner = createSignerFromKeypair(umi, createrKeypair); // signer for user/creater
    umi.use(signerIdentity(createrSigner));
    umi.use(mplTokenMetadata());

    const collection = new anchor.web3.PublicKey(
      collectionMint.publicKey.toString()
    );
    const nftMintAcc = new anchor.web3.PublicKey(nftMint.publicKey.toString());
    await nftCollection(umi, collectionMint);
  } catch (e) {
    console.log(`Error while trying to setup NFTs`);
  }
};

const nftCollection = async (umi: Umi, collectionMint: KeypairSigner) => {
  try {
    console.log(
      `🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥 This thing is working 🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥`
    );

    let trx = await createNft(umi, {
      mint: collectionMint,
      name: "Bhudha",
      uri: "https://lavender-worthy-duck-16.mypinata.cloud/ipfs/bafkreidexwujuv7rfdkdkk3ibwldy7gitmksnmpe2ti24nlfonnjo7kcfa",
      sellerFeeBasisPoints: percentAmount(5),
    }).sendAndConfirm(umi);

    console.log(`✅ Your collection is ${collectionMint.publicKey}`);
  } catch (e) {
    console.log(`Error while trying to create NFT Collection ${e}`);
  }
};

describe("nft-stacking", () => {
  let provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.NftStacking as Program<NftStacking>;

  let admin: anchor.web3.Keypair;
  let user: anchor.web3.Keypair;
  let configPDA: anchor.web3.PublicKey;
  let rewardMintAcc: anchor.web3.PublicKey;
  let userPDA: anchor.web3.PublicKey;
  let stackPDA;
  let userNFTATA: anchor.web3.PublicKey;
  let NFTMintAcc: anchor.web3.PublicKey;

  // arguments
  let maxStack = 10;
  let rewardPerStack = 20;
  let freezPeriod = 100; // 100 days

  before("Setting up for testing", async () => {
    try {
      admin = anchor.web3.Keypair.generate();
      user = anchor.web3.Keypair.generate();

      await airdropSOL(admin.publicKey, provider, 1000);
      await airdropSOL(user.publicKey, provider, 1000);

      configPDA = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("config_stack")],
        program.programId
      )[0];

      rewardMintAcc = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("reward_mint"), configPDA.toBuffer()],
        program.programId
      )[0];

      userPDA = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), configPDA.toBuffer(), user.publicKey.toBuffer()],
        program.programId
      )[0];

      // stackPDA = anchor.web3.PublicKey.findProgramAddressSync(
      //   [Buffer.from("user_nft"), userPDA.toBuffer()],
      //   program.programId
      // );

      await nftSetup(provider, user);
    } catch (e) {
      console.log(`Error Occured in Testing setup ${e}`);
    }
  });

  it("Is initialized!", async () => {
    try {
      let trx = await program.methods
        .initialize(freezPeriod, maxStack, rewardPerStack)
        .accountsStrict({
          admin: admin.publicKey,
          configPda: configPDA,
          rewardMint: rewardMintAcc,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([admin])
        .rpc();

      console.log(`Your Trasaction is sucessfull ${trx.toString()}`);
    } catch (e) {
      console.log(`You got error while trying to test 1st test case ${e} `);
    }
  });

  // it("Initialize User", async () => {
  //   try {
  //     let trx = await program.methods
  //       .initializeUser()
  //       .accountsStrict({
  //         configPda: configPDA,
  //         user: user.publicKey,
  //         userPda: userPDA,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([user])
  //       .rpc();

  //     console.log(`Successfully tested user initialization ${trx.toString()}`);
  //   } catch (e) {
  //     console.log(
  //       `You got error while testing Initialize User instruction ${e}`
  //     );
  //   }
  // });

  // it("Test staking NFT", async () => {
  //   try {
  //     await program.methods
  //       .stackNft()
  //       .accountsStrict({
  //         user: user.publicKey,
  //         configAccount: configPDA,
  //         userAccount: userPDA,
  //       })
  //       .signers([user])
  //       .rpc();
  //   } catch (e) {
  //     console.log(`You got error while trying to test staking NFT ${e}`);
  //   }
  // });
});
