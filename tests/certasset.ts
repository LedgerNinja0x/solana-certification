import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { Certasset } from "../target/types/certasset";

describe("certasset", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Certasset as Program<Certasset>;
  const provider = anchor.AnchorProvider.local();

  // Creates Authority and Applicant
  console.log("Generating Testing Keypairs ...");

  const authority = anchor.web3.Keypair.generate();
  console.log("Generated Authority: " + authority.publicKey.toString());

  const applicant = provider.wallet;
  console.log("Generated Applicant: " + applicant.publicKey.toString());

  const request_key = anchor.web3.Keypair.generate();
  console.log("Generated Request Key: " + request_key.publicKey.toString());

  before(async () => {
    console.log("Requesting Airdrop for Authority Test Wallet");
    const airdrop_tx = await provider.connection.requestAirdrop(
      authority.publicKey,
      10000000000
    );
    console.log("Successfully Requested with TX ID: " + airdrop_tx);
  });

  it("Ping the Program", async () => {
    console.log("Pinging Program ...");
    await program.methods.ping().rpc();
    console.log("Succesfull Ping!");
  });

  it("Creates a Signing Request", async () => {
    // Call the smart contract
    const tx = await program.methods
      .createRequest(authority.publicKey, "hola mundo")
      .accounts({
        request: request_key.publicKey,
        applicant: applicant.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([request_key])
      .rpc();
    console.log("Your transaction signature", tx);

    let signingRequest = await program.account.signingRequest.fetch(
      request_key.publicKey
    );

    assert.ok(signingRequest.applicant.equals(applicant.publicKey));
    assert.ok(signingRequest.authority.equals(authority.publicKey));
    assert.isFalse(signingRequest.signed);
    assert.equal(signingRequest.uri, "hola mundo");
  });

  it("Certificates a Signing Request", async () => {
    console.log("Generating PDA Public Key Address...");
    const [mint_key, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("certasset-rq"),
        request_key.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("Mint PDA: " + mint_key);
    console.log("Bump: " + bump);

    const cert_tx = await program.methods
      .signCertificate()
      .accounts({
        authority: authority.publicKey,
        request: request_key.publicKey,
        mint: mint_key,

        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram2022: new anchor.web3.PublicKey(
          "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        ),
      })
      .signers([authority])
      .rpc({
        skipPreflight: true,
      });
  
    console.log("Signing Request Sent and Processed!");

    let signedRequest = await program.account.signingRequest.fetch(
      request_key.publicKey
    );

    assert.isTrue(signedRequest.signed);
  });
});
