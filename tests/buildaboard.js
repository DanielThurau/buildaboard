const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async () => {
    console.log("Starting test...");

    const provider = anchor.Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Buildaboard;
    const baseAccount = anchor.web3.Keypair.generate();

    let tx = await program.rpc.startStuffOff({
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
    });


    console.log("Your transaction signature", tx);

    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Pic Count', account.totalPics.toString());

    await program.rpc.addPic(
        
        "https://build-a-board-bucket.s3.us-west-1.amazonaws.com/rmLI9kg_ppE.jpg",
        {
            accounts: {
                baseAccount: baseAccount.publicKey,
                user: provider.wallet.publicKey,
            },
        }
    );

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Pic Count', account.totalPics.toString());
    console.log('Pic List', account.picList);

    await program.rpc.upvotePic(
        "https://build-a-board-bucket.s3.us-west-1.amazonaws.com/rmLI9kg_ppE.jpg",
        {
            accounts: {
                baseAccount: baseAccount.publicKey,
            },
        }
    );

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Pic Count', account.totalPics.toString());
    console.log('Pic List', account.picList);
};


const runMain = async () => {
    try {
        await main();
        process.exit(0);
    } catch (error) {
        console.log(error);
        process.exit(1);
    }
};

runMain();