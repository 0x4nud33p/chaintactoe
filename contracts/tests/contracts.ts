import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Contracts } from "../target/types/contracts";

describe("contracts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.contracts as Program<Contracts>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

// -----------------------------------------------------------------------------
// CustomError IDL validation tests (focus on errors.rs diff)
// - Verifies presence, codes, messages, uniqueness, and conventions
// -----------------------------------------------------------------------------

describe("CustomError (IDL validation)", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.contracts as Program<Contracts>;

  // Expected mapping derived from errors.rs diff

  const expected = {
    GameNotOpen: {
      code: 6000,
      msg: "The game is not open for joining.",
    },
    GameAlreadyHasPlayerO: {
      code: 6001,
      msg: "The game already has a player O.",
    },
  } as const;

  it("IDL is present and exposes errors list", () => {
    expect(program).to.exist;
    expect(program.idl).to.exist;
    const errors = program.idl.errors ?? [];
    expect(errors.length).to.be.greaterThan(0);
  });

  it("contains GameNotOpen with correct code and message", () => {
    const errors = program.idl.errors ?? [];
    const found = errors.find((e) => e.name === "GameNotOpen");
    expect(found, "GameNotOpen missing in IDL").to.exist;
    expect(found?.code).to.equal(expected.GameNotOpen.code);
    expect(found?.msg).to.equal(expected.GameNotOpen.msg);
  });

  it("contains GameAlreadyHasPlayerO with correct code and message", () => {
    const errors = program.idl.errors ?? [];
    const found = errors.find((e) => e.name === "GameAlreadyHasPlayerO");
    expect(found, "GameAlreadyHasPlayerO missing in IDL").to.exist;
    expect(found?.code).to.equal(expected.GameAlreadyHasPlayerO.code);
    expect(found?.msg).to.equal(expected.GameAlreadyHasPlayerO.msg);
  });

  it("error codes are unique for the defined variants", () => {
    const codes = Object.values(expected).map((e) => e.code);
    expect(new Set(codes).size).to.equal(codes.length);
  });

  it("error codes start at 6000 and are sequential for these variants", () => {
    const sorted = Object.values(expected)
      .map((e) => e.code)
      .sort((a, b) => a - b);
    expect(sorted[0]).to.equal(6000);
    for (let i = 1; i < sorted.length; i++) {
      expect(sorted[i]).to.equal(sorted[i - 1] + 1);
    }
  });

  it("messages are clear, properly capitalized, and punctuated", () => {
    Object.values(expected).forEach(({ msg }) => {
      expect(msg).to.match(/^[A-Z].+\.$/); // Starts with capital, ends with period
      expect(msg).to.not.match(/  /); // No double spaces
      expect(msg.length).to.be.greaterThan(10);
    });
  });

  it("error names use PascalCase and begin with 'Game'", () => {
    Object.keys(expected).forEach((name) => {
      expect(name).to.match(/^[A-Z][A-Za-z0-9]*$/);
      expect(name.startsWith("Game")).to.be.true;
    });
  });

  it("IDL contains no mismatched code->name or code->message for these errors", () => {
    const errors = program.idl.errors ?? [];
    const byCode = new Map(errors.map((e) => [e.code, e]));
    Object.values(expected).forEach(({ code, msg }) => {
      const entry = byCode.get(code);
      expect(entry, `No IDL entry for code ${code}`).to.exist;
      expect(entry?.msg).to.equal(msg);
    });
  });
});