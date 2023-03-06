import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Certasset } from "../target/types/certasset";

describe("certasset", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Certasset as Program<Certasset>;
  const provider = anchor.AnchorProvider.local();

  it("Ping the Program", async () => {
    console.log("Pinging Program ...");
    await program.methods.ping().rpc();
    console.log("Succesfull Ping!")
  })

  it("Creates a Signing Request", async () => {
    // Creates Authority and Applicant
    console.log("Generating Testing Keypairs ...");
    
    const authority = anchor.web3.Keypair.generate();
    console.log("Generated Authority: " + authority.publicKey.toString());

    const applicant = provider.wallet;
    console.log("Generated Applicant: " + applicant.publicKey.toString());

    const request_key = anchor.web3.Keypair.generate();
    console.log("Generated Request Key: " + request_key.publicKey.toString());

    // Call the smart contract

    const tx = await program.methods.createRequest(authority.publicKey, "hola mundo")
      .accounts({
        request: request_key.publicKey,
        applicant: applicant.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([request_key])
      .rpc();
    console.log("Your transaction signature", tx);

    let signingRequest = await program.account.signingRequest.fetch(applicant.publicKey);

    expect(signingRequest.applicant).equal(applicant.publicKey);
    expect(signingRequest.authority).equal(authority.publicKey);
    expect(signingRequest.signed).equal(false);
    expect(signingRequest.uri).equal("hola mundo");
  });
});
