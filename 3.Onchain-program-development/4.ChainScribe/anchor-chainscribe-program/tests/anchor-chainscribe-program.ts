import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorChainscribeProgram } from "../target/types/anchor_chainscribe_program";
import { expect } from "chai";

describe("anchor-chainscribe-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .anchorChainscribeProgram as Program<AnchorChainscribeProgram>;

  const topic = {
    topic_id: "1000",
    topic_generator_name: "Harsh",
    topic_title: "BlockChain",
    topic_description: "About Blockchain",
  };
  const newTopic = {
    topic_generator_name: "newHarsh",
    topic_title: "newBlockChain",
    topic_description: "new About Blockchain",
  };
  const newBlog = "blockChain is best and rust is best!";

  const blog = {
    blogId: "01",
    blog: "BlockChain is interesting!",
    generatorName: "Dev",
  };

  const comment = {
    comment_id: "1111",
    comment_text: "This is best blog",
  };
  const newComment = "This is best blog i have ever seen!";

  const [commentPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("comment"),
      Buffer.from(comment.comment_id),
      Buffer.from(blog.blogId),
      provider.wallet.publicKey.toBuffer(),
    ],
    program.programId
  );

  const [topicPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("topic"), Buffer.from(topic.topic_id)],
    program.programId
  );

  const [blogPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("blog"),
      Buffer.from(topic.topic_id),
      Buffer.from(blog.blogId),
    ],
    program.programId
  );

  it("Add a topic!", async () => {
    try {
      const beforeTimestamp = Math.floor(Date.now() / 1000);
      await program.methods
        .createTopic(
          topic.topic_id,
          topic.topic_generator_name,
          topic.topic_title,
          topic.topic_description
        )
        .accounts({})
        .rpc();

      const account = await program.account.topicAccountState.fetch(topicPda);
      expect(account.topicGeneratorId.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(account.topicId).to.equal(topic.topic_id);
      expect(account.topicGeneratorName).to.equal(topic.topic_generator_name);
      expect(account.topicTitle).to.equal(topic.topic_title);
      expect(account.topicDescription).to.equal(topic.topic_description);
      expect(account.noOfBlog).to.equal(0);
      expect(account.likes).to.equal(0);
      expect(account.comments).to.equal(0);
      expect(account.isActive).to.equal(true);
      expect(account.isPublic).to.equal(true);

      const afterTimestamp = Math.floor(Date.now() / 1000);
      expect(account.lastUpdatedAt.toNumber()).to.be.within(
        beforeTimestamp - 1,
        afterTimestamp
      );
    } catch (error) {
      console.error("Error adding topic:", error);
      throw error;
    }
  });

  it("Add a Blog!", async () => {
    try {
      const beforeTimestamp = Math.floor(Date.now() / 1000);
      await program.methods
        .createBlog(topic.topic_id, blog.blogId, blog.generatorName, blog.blog)
        .accounts({})
        .rpc();

      const account = await program.account.blogAccountState.fetch(blogPda);

      const topicAccount = await program.account.topicAccountState.fetch(
        topicPda
      );

      expect(account.blogGenerator.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(account.topicId).to.equal(topic.topic_id);
      expect(account.blogGeneratorName).to.equal(blog.generatorName);
      expect(account.blogId).to.equal(blog.blogId);
      expect(account.blog).to.equal(blog.blog);
      expect(topicAccount.noOfBlog).to.equal(1);
      expect(account.likes).to.equal(0);
      expect(account.comments).to.equal(0);

      const afterTimestamp = Math.floor(Date.now() / 1000);
      expect(account.lastUpdatedAt.toNumber()).to.be.within(
        beforeTimestamp - 1,
        afterTimestamp
      );
      expect(topicAccount.lastUpdatedAt.toNumber()).to.be.within(
        beforeTimestamp - 1,
        afterTimestamp
      );
    } catch (error) {
      console.error("Error adding blog:", error);
      throw error;
    }
  });

  it("Update topic!", async () => {
    try {
      const beforeTimestamp = Math.floor(Date.now() / 1000);
      await program.methods
        .updateTopic(
          topic.topic_id,
          newTopic.topic_generator_name,
          newTopic.topic_title,
          newTopic.topic_description
        )
        .accounts({})
        .rpc();

      const account = await program.account.topicAccountState.fetch(topicPda);
      expect(account.topicGeneratorId.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(account.topicId).to.equal(topic.topic_id);
      expect(account.topicGeneratorName).to.equal(
        newTopic.topic_generator_name
      );
      expect(account.topicTitle).to.equal(newTopic.topic_title);
      expect(account.topicDescription).to.equal(newTopic.topic_description);
      expect(account.noOfBlog).to.equal(1);
      expect(account.likes).to.equal(0);
      expect(account.comments).to.equal(0);
      expect(account.isActive).to.equal(true);
      expect(account.isPublic).to.equal(true);

      const afterTimestamp = Math.floor(Date.now() / 1000);
      expect(account.lastUpdatedAt.toNumber()).to.be.within(
        beforeTimestamp - 1,
        afterTimestamp
      );
    } catch (error) {
      console.error("Error updating topic:", error);
      throw error;
    }
  });

  it("Update Blog!", async () => {
    try {
      const beforeTimestamp = Math.floor(Date.now() / 1000);
      await program.methods
        .updateBlog(topic.topic_id, blog.blogId, blog.blog)
        .accounts({})
        .rpc();

      const account = await program.account.blogAccountState.fetch(blogPda);

      const topicAccount = await program.account.topicAccountState.fetch(
        topicPda
      );

      expect(account.blogGenerator.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(account.topicId).to.equal(topic.topic_id);
      expect(account.blogGeneratorName).to.equal(blog.generatorName);
      expect(account.blogId).to.equal(blog.blogId);
      expect(account.blog).to.equal(blog.blog);
      expect(topicAccount.noOfBlog).to.equal(1);
      expect(account.likes).to.equal(0);
      expect(account.comments).to.equal(0);

      const afterTimestamp = Math.floor(Date.now() / 1000);
      expect(account.lastUpdatedAt.toNumber()).to.be.within(
        beforeTimestamp - 1,
        afterTimestamp
      );
      expect(topicAccount.lastUpdatedAt.toNumber()).to.be.within(
        beforeTimestamp - 1,
        afterTimestamp
      );
    } catch (error) {
      console.error("Error updating blog:", error);
      throw error;
    }
  });

  it("add likes in blog", async () => {
    try {
      await program.methods
        .addLikeToBlog(topic.topic_id, blog.blogId)
        .accounts({})
        .rpc();

      const account = await program.account.blogAccountState.fetch(blogPda);
      expect(account.likes).to.equal(1);
    } catch (error) {
      console.error("Error adding likes:", error);
      throw error;
    }
  });

  it("add likes to topic!", async () => {
    try {
      await program.methods.addLikeToTopic(topic.topic_id).accounts({}).rpc();

      const account = await program.account.topicAccountState.fetch(topicPda);
      expect(account.topicGeneratorId.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(account.topicId).to.equal(topic.topic_id);
      expect(account.topicGeneratorName).to.equal(
        newTopic.topic_generator_name
      );
      expect(account.topicTitle).to.equal(newTopic.topic_title);
      expect(account.topicDescription).to.equal(newTopic.topic_description);
      expect(account.noOfBlog).to.equal(1);
      expect(account.likes).to.equal(1);
      expect(account.comments).to.equal(0);
      expect(account.isActive).to.equal(true);
      expect(account.isPublic).to.equal(true);
    } catch (error) {
      console.error("Error liking topic:", error);
      throw error;
    }
  });

  it("add a comment", async () => {
    try {
      await program.methods
        .addComment(
          comment.comment_id,
          blog.blogId,
          topic.topic_id,
          comment.comment_text
        )
        .accounts({})
        .rpc();

      const account = await program.account.commentAccountState.fetch(
        commentPda
      );
      expect(account.commentId).to.equal(comment.comment_id);
      expect(account.commentText).to.equal(comment.comment_text);
      const blogAccount = await program.account.blogAccountState.fetch(blogPda);
      expect(blogAccount.comments).to.equal(1);
    } catch (error) {
      console.log("Error adding comment: ", error);
      throw error;
    }
  });

  it("update Comment", async () => {
    try {
      await program.methods
        .updateComment(
          comment.comment_id,
          blog.blogId,
          topic.topic_id,
          newComment
        )
        .accounts({})
        .rpc();

      const account = await program.account.commentAccountState.fetch(
        commentPda
      );
      expect(account.commentId).to.equal(comment.comment_id);
      expect(account.commentText).to.equal(newComment);
      const blogAccount = await program.account.blogAccountState.fetch(blogPda);
      expect(blogAccount.comments).to.equal(1);
    } catch (error) {
      console.log("Error updating comment: ", error);
      throw error;
    }
  });

  it("delete comment", async () => {
    try {
      await program.methods
        .deleteComment(comment.comment_id, blog.blogId, topic.topic_id)
        .accounts({})
        .rpc();

      const account = await program.account.commentAccountState.fetchNullable(
        commentPda
      );
      expect(account).to.be.null;
    } catch (error) {
      console.error("Error deleting comment:", error);
      throw error;
    }
  });
});
