import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Twitter } from "../target/types/twitter";
import { PublicKey } from '@solana/web3.js';
import { assert } from "chai";
import crypto from "crypto";

const TWEET_SEED = "TWEET_SEED";
const COMMENT_SEED = "COMMENT_SEED";
const TWEET_REACTION_SEED = "TWEET_REACTION_SEED";

describe("twitter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Twitter as Program<Twitter>;

  const bob = anchor.web3.Keypair.generate();
  const alice = anchor.web3.Keypair.generate();

  const topic_bob1 = "Hello There";
  const content_bob1 = "This is my first tweet on this app, I like it here!";
  const comment_alice = "I don’t like you Bob!";

  describe("Initialize Tweet", () => {
    it("Bob creates a tweet", async () => {
      await airdrop(provider.connection, bob.publicKey);
      const [tweet_pkey, tweet_bump] = getTweetAddress(topic_bob1, bob.publicKey, program.programId);

      await program.methods.createTweet(topic_bob1, content_bob1).accounts({
        user: bob.publicKey,
        tweet: tweet_pkey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([bob]).rpc({ skipPreflight: true});

      await checkTweet(program, tweet_pkey, bob.publicKey, topic_bob1, content_bob1, 0, 0, tweet_bump);
    });
  });

  describe("Alice comments on Bob’s tweet", () => {
    it("Alice adds a comment", async () => {
      await airdrop(provider.connection, alice.publicKey);

      const [tweet_pkey] = getTweetAddress(topic_bob1, bob.publicKey, program.programId);
      const [comment_pkey, comment_bump] = getCommentAddress(comment_alice, alice.publicKey, tweet_pkey, program.programId);

      await program.methods.addComment(comment_alice).accounts({
        user: alice.publicKey,
        tweetComment: comment_pkey,
        tweet: tweet_pkey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([alice]).rpc({ skipPreflight: true});

      await checkComment(program, comment_pkey, alice.publicKey, tweet_pkey, comment_alice, comment_bump);
    });

    it("Alice removes her comment", async () => {
      const [tweet_pkey] = getTweetAddress(topic_bob1, bob.publicKey, program.programId);
      const [comment_pkey] = getCommentAddress(comment_alice, alice.publicKey, tweet_pkey, program.programId);

      await program.methods.removeComment().accounts({
        user: alice.publicKey,
        tweetComment: comment_pkey,
        tweet: tweet_pkey,
      }).signers([alice]).rpc({ skipPreflight: true});

      let should_fail = "This should fail";
      try {
        await program.account.comment.fetch(comment_pkey);
      } catch (error) {
        should_fail = "Failed";
        assert.isTrue(error.message.includes("Account does not exist or has no data"));
      }
      assert.strictEqual(should_fail, "Failed");
    });
  });
});

/** =======================
 *  📌 Helper Functions
 *  ======================= **/

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}

function getTweetAddress(topic: string, author: PublicKey, programID: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(TWEET_SEED),
      anchor.utils.bytes.utf8.encode(topic),
      author.toBuffer()
    ], programID);
}

function getCommentAddress(content: string, author: PublicKey, parent_tweet: PublicKey, programID: PublicKey) {
  let hexString = crypto.createHash('sha256').update(content, 'utf-8').digest('hex');
  let content_seed = Uint8Array.from(Buffer.from(hexString, 'hex'));

  return PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(COMMENT_SEED),
      author.toBuffer(),
      content_seed,
      parent_tweet.toBuffer(),
    ], programID);
}

async function checkTweet(
  program: anchor.Program<Twitter>,
  tweet: PublicKey,
  tweet_author?: PublicKey,
  topic?: string,
  content?: string,
  likes?: number,
  dislikes?: number,
  bump?: number,
) {
  let tweetData = await program.account.tweet.fetch(tweet);

  if (tweet_author) assert.strictEqual(tweetData.author.toString(), tweet_author.toString());
  if (topic) {
    const utf8ByteArray_topic = stringToUtf8ByteArray(topic);
    const paddedByteArray_topic = padByteArrayWithZeroes(utf8ByteArray_topic, 32);
    assert.strictEqual(tweetData.topic.toString(), paddedByteArray_topic.toString());
    assert.strictEqual(tweetData.topicLength.toString(), utf8ByteArray_topic.length.toString());
  }
  if (content) {
    const utf8ByteArray_content = stringToUtf8ByteArray(content);
    const paddedByteArray_content = padByteArrayWithZeroes(utf8ByteArray_content, 500);
    assert.strictEqual(tweetData.content.toString(), paddedByteArray_content.toString());
  }
  if (likes !== undefined) assert.strictEqual(tweetData.likes.toString(), new anchor.BN(likes).toString());
  if (dislikes !== undefined) assert.strictEqual(tweetData.dislikes.toString(), new anchor.BN(dislikes).toString());
  if (bump) assert.strictEqual(tweetData.bump.toString(), bump.toString());
}

async function checkComment(
  program: anchor.Program<Twitter>,
  comment: PublicKey,
  comment_author?: PublicKey,
  parent_tweet?: PublicKey,
  content?: string,
  bump?: number,
) {
  let commentData = await program.account.comment.fetch(comment);

  if (comment_author) assert.strictEqual(commentData.author.toString(), comment_author.toString());
  if (parent_tweet) assert.strictEqual(commentData.tweet.toString(), parent_tweet.toString());
  if (content) {
    const utf8ByteArray_content = stringToUtf8ByteArray(content);
    const paddedByteArray_content = padByteArrayWithZeroes(utf8ByteArray_content, 500);
    assert.strictEqual(commentData.content.toString(), paddedByteArray_content.toString());
    assert.strictEqual(commentData.contentLength.toString(), utf8ByteArray_content.length.toString());
  }
  if (bump) assert.strictEqual(commentData.bump.toString(), bump.toString());
}

function stringToUtf8ByteArray(inputString: string): Uint8Array {
  const encoder = new TextEncoder();
  return encoder.encode(inputString);
}

function padByteArrayWithZeroes(byteArray: Uint8Array, length: number): Uint8Array {
  if (byteArray.length >= length) return byteArray;
  const paddedArray = new Uint8Array(length);
  paddedArray.set(byteArray, 0);
  return paddedArray;
}
