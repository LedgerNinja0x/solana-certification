import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Certasset } from "../target/types/certasset";

describe("certasset", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Certasset as Program<Certasset>;

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
    const applicant = anchor.web3.Keypair.generate();
    console.log("Generated Applicant: " + applicant.publicKey.toString());

    const tx = await program.methods.createRequest(authority.publicKey, "hola mundo")
      .accounts({
        request: applicant.publicKey
      })
      .signers([applicant]).rpc();
    console.log("Your transaction signature", tx);

    let signingRequest = await program.account.signingRequest.fetch(applicant.publicKey);

    expect(signingRequest.applicant).equal(applicant.publicKey);
    expect(signingRequest.authority).equal(authority.publicKey);
    expect(signingRequest.signed).equal(false);
    expect(signingRequest.uri).equal("hola mundo");
  });
});
