# ChainScribe - Decentralized Blogging Platform on Solana

ChainScribe is a decentralized blogging platform built on the Solana blockchain using the Anchor framework. It enables users to create, manage, and interact with topics and blogs in a transparent and immutable way.

## 🌟 Features

- **Topic Management**
  - Create and update topics
  - Like topics
  - Track topic statistics (blogs, likes, comments)
  - Public/private topic visibility

- **Blog Management**
  - Create and update blogs within topics
  - Like blogs
  - Track blog statistics (comments, likes)
  - Timestamp tracking for updates

- **Comment System**
  - Add, update, and delete comments on blogs
  - Track comment history
  - Timestamp tracking for comments

## 🏗️ Architecture

The project is built using the following components:

### Smart Contract (Solana Program)
Located in `anchor-chainscribe-program/programs/anchor-chainscribe-program/`

#### State Accounts
1. **TopicAccountState**
```rust
#[account]
pub struct TopicAccountState {
    pub topic_generator_id: Pubkey,      // Public key of the topic creator
    pub topic_generator_name: String,    // Name of the topic creator
    pub topic_id: String,                // Unique identifier for the topic
    pub topic_title: String,             // Title of the topic
    pub topic_description: String,       // Description of the topic
    pub no_of_blog: u32,                 // Number of blogs in the topic
    pub likes: u32,                      // Number of likes on the topic
    pub comments: u32,                   // Number of comments on the topic
    pub is_active: bool,                 // Topic status
    pub is_public: bool,                 // Topic visibility
    pub last_updated_at: i64,            // Last update timestamp
}
```

2. **BlogAccountState**
```rust
#[account]
pub struct BlogAccountState {
    pub blog_generator: Pubkey,          // Public key of the blog creator
    pub blog_generator_name: String,     // Name of the blog creator
    pub topic_id: String,                // Associated topic ID
    pub blog_id: String,                 // Unique identifier for the blog
    pub blog: String,                    // Blog content
    pub comments: u32,                   // Number of comments
    pub likes: u32,                      // Number of likes
    pub last_updated_at: i64,            // Last update timestamp
}
```

3. **CommentAccountState**
```rust
#[account]
pub struct CommentAccountState {
    pub commenter: Pubkey,               // Public key of the commenter
    pub comment_id: String,              // Unique identifier for the comment
    pub topic_id: String,                // Associated topic ID
    pub blog_id: String,                 // Associated blog ID
    pub comment_text: String,            // Comment content
    pub last_updated_at: i64,            // Last update timestamp
}
```

### Program Instructions
The smart contract provides the following instructions:

1. **Topic Management**
```rust
// Create a new topic
pub fn create_topic(
    ctx: Context<CreateTopic>,
    topic_id: String,
    topic_generator_name: String,
    topic_title: String,
    topic_description: String,
) -> Result<()> {
    // Implementation details
}

// Update an existing topic
pub fn update_topic(
    ctx: Context<UpdateTopic>,
    topic_id: String,
    topic_generator_name: String,
    topic_title: String,
    topic_description: String,
) -> Result<()> {
    // Implementation details
}

// Like a topic
pub fn add_like_to_topic(
    ctx: Context<UpdateTopic>,
    topic_id: String,
) -> Result<()> {
    // Implementation details
}
```

2. **Blog Management**
```rust
// Create a new blog
pub fn create_blog(
    ctx: Context<CreateBlog>,
    topic_id: String,
    blog_id: String,
    blog_generator_name: String,
    blog: String,
) -> Result<()> {
    // Implementation details
}

// Update a blog
pub fn update_blog(
    ctx: Context<UpdateBlog>,
    topic_id: String,
    blog_id: String,
    blog: String,
) -> Result<()> {
    // Implementation details
}

// Like a blog
pub fn add_like_to_blog(
    ctx: Context<UpdateBlog>,
    topic_id: String,
    blog_id: String,
) -> Result<()> {
    // Implementation details
}
```

3. **Comment Management**
```rust
// Add a new comment
pub fn add_comment(
    ctx: Context<AddComment>,
    comment_id: String,
    blog_id: String,
    topic_id: String,
    comment_text: String,
) -> Result<()> {
    // Implementation details
}

// Update a comment
pub fn update_comment(
    ctx: Context<UpdateComment>,
    comment_id: String,
    blog_id: String,
    topic_id: String,
    comment_text: String,
) -> Result<()> {
    // Implementation details
}

// Delete a comment
pub fn delete_comment(
    ctx: Context<DeleteComment>,
    comment_id: String,
    blog_id: String,
    topic_id: String,
) -> Result<()> {
    // Implementation details
}
```

## 🚀 Getting Started

### Prerequisites
- Solana CLI tools
- Anchor Framework
- Node.js and npm/yarn
- Rust toolchain

### Installation

1. Clone the repository:
```bash
git clone https://github.com/h-a-r-s-h-s-r-a-h/ChainScribe.git
cd ChainScribe
```

2. Install dependencies:
```bash
cd anchor-chainscribe-program
yarn install
```

3. Build the program:
```bash
anchor build
```

4. Deploy the program:
```bash
anchor deploy
```

### Development

1. Start a local Solana validator:
```bash
solana-test-validator
```

2. Run tests:
```bash
anchor test
```

## 📝 Usage Examples

### Creating a Topic
```typescript
// Initialize the program
const program = new Program(IDL, PROGRAM_ID, provider);

// Create a new topic
const topicId = "topic-1";
const topicGeneratorName = "John Doe";
const topicTitle = "My First Topic";
const topicDescription = "This is a description";

// Generate PDA for the topic account
const [topicPda] = await PublicKey.findProgramAddress(
  [
    Buffer.from("topic"),
    Buffer.from(topicId),
  ],
  program.programId
);

// Create the topic
await program.methods
  .createTopic(
    topicId,
    topicGeneratorName,
    topicTitle,
    topicDescription
  )
  .accounts({
    topicAccount: topicPda,
    topicGenerator: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

console.log("Topic created successfully!");
```

### Creating a Blog
```typescript
// Create a new blog
const blogId = "blog-1";
const blogContent = "This is my blog content...";

// Generate PDA for the blog account
const [blogPda] = await PublicKey.findProgramAddress(
  [
    Buffer.from("blog"),
    Buffer.from(topicId),
    Buffer.from(blogId),
  ],
  program.programId
);

// Create the blog
await program.methods
  .createBlog(
    topicId,
    blogId,
    "John Doe",
    blogContent
  )
  .accounts({
    blogAccount: blogPda,
    topicAccount: topicPda,
    blogGenerator: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

console.log("Blog created successfully!");
```

### Adding a Comment
```typescript
// Add a new comment
const commentId = "comment-1";
const commentText = "Great article!";

// Generate PDA for the comment account
const [commentPda] = await PublicKey.findProgramAddress(
  [
    Buffer.from("comment"),
    Buffer.from(topicId),
    Buffer.from(blogId),
    Buffer.from(commentId),
  ],
  program.programId
);

// Add the comment
await program.methods
  .addComment(
    commentId,
    blogId,
    topicId,
    commentText
  )
  .accounts({
    commentAccount: commentPda,
    blogAccount: blogPda,
    topicAccount: topicPda,
    commenter: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

console.log("Comment added successfully!");
```

### Liking a Blog
```typescript
// Like a blog
await program.methods
  .addLikeToBlog(
    topicId,
    blogId
  )
  .accounts({
    blogAccount: blogPda,
    topicAccount: topicPda,
    liker: provider.wallet.publicKey,
  })
  .rpc();

console.log("Blog liked successfully!");
```

## 🔒 Security Features

1. **PDA (Program Derived Address) Usage**
```rust
// Example of PDA derivation for topic accounts
pub fn find_topic_pda(topic_id: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"topic",
            topic_id.as_bytes(),
        ],
        program_id,
    )
}
```

2. **Access Control**
```rust
// Example of access control check
require!(
    ctx.accounts.topic_account.topic_generator_id == ctx.accounts.topic_generator.key(),
    ErrorCode::UnauthorizedAccess
);
```

3. **Input Validation**
```rust
// Example of input validation
require!(
    topic_title.len() <= MAX_TITLE_LENGTH,
    ErrorCode::TitleTooLong
);
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Here's how you can contribute:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Solana Team for the amazing blockchain platform
- Anchor Framework team for the development framework
- All contributors and users of ChainScribe 