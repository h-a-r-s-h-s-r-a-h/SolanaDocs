# Solana Development Learning Path

This repository contains a comprehensive collection of Solana development examples and tutorials, organized into two main sections. Each section builds upon the previous one, creating a complete learning path for Solana development.

## 1. Introduction to Cryptography and Solana Clients

### 1.1 Cryptography and the Solana Network
This section covers fundamental concepts of cryptography and basic interactions with the Solana network. It's the foundation for understanding how Solana works.

#### Key Components:

1. **Keypair Generation** (`generate-keypair.ts`)
   - Creates new Solana keypairs using `@solana/web3.js`
   - Generates both public and private keys
   - Example usage:
     ```typescript
     const keypair = web3.Keypair.generate();
     console.log(`Public key: ${keypair.publicKey.toBase58()}`);
     ```

2. **Keypair Loading** (`loading-keypair.ts`)
   - Demonstrates loading existing keypairs from environment variables
   - Essential for wallet integration
   - Uses secure environment variable handling

3. **Balance Checking** (`check-balance.ts`)
   - Shows how to check SOL balance of any account
   - Connects to Solana network (devnet/mainnet)
   - Real-time balance monitoring

4. **SOL Transfer** (`transfer.ts`)
   - Implements SOL transfer functionality
   - Handles transaction creation and signing
   - Includes error handling and confirmation

### 1.2 Interact With Wallets
A Next.js application demonstrating modern wallet integration with Solana.

#### Features:

1. **Wallet Connection**
   - Supports multiple wallet providers (Phantom, Solflare)
   - Handles connection/disconnection events
   - Manages wallet state using React Context

2. **Transaction Management**
   - Real-time balance updates
   - Transaction history
   - Transaction signing and confirmation

3. **UI Components**
   - Modern, responsive design with Tailwind CSS
   - Loading states and error handling
   - Interactive transaction forms

## 2. Tokens and NFTs on Solana

### 2.1 Create Tokens With The Token Program
Basic token creation and management using Solana's Token Program.

#### Key Operations:

1. **Token Mint Creation** (`create-token-mint.ts`)
   - Creates new token mints with specified decimals
   - Sets up mint authority
   - Example:
     ```typescript
     const tokenMint = await createMint(
       connection,
       user,
       user.publicKey,
       null,
       2  // decimals
     );
     ```

2. **Token Account Management**
   - `create-token-account.ts`: Creates new token accounts
   - `create-friend-token-account.ts`: Creates token accounts for other users
   - Handles account initialization and rent exemption

3. **Token Operations**
   - `mint-tokens.ts`: Mints new tokens to specified accounts
   - `transfer-tokens.ts`: Transfers tokens between accounts
   - Includes proper error handling and transaction confirmation

4. **Token Metadata** (`create-token-metadata.ts`)
   - Adds metadata to tokens (name, symbol, URI)
   - Supports custom attributes
   - Integrates with Metaplex metadata program

### 2.2 Create Tokens With The Token Program (Client)
A Next.js application providing a user interface for token management.

#### Features:

1. **Token Creation Interface**
   - Form for creating new tokens
   - Decimal selection
   - Initial supply setting

2. **Account Management**
   - Token account creation
   - Balance checking
   - Account association

3. **Transaction Interface**
   - Token transfer forms
   - Transaction history
   - Real-time updates

4. **UI/UX**
   - Modern React components
   - Loading states
   - Error handling
   - Transaction notifications

### 2.3 Create Solana NFTs With Metaplex
NFT creation and management using the Metaplex protocol.

#### Key Operations:

1. **NFT Creation** (`create-metaplex-nft.ts`)
   - Creates new NFTs with metadata
   - Supports image uploads
   - Sets up NFT attributes

2. **Collection Management** (`create-metaplex-nft-collection.ts`)
   - Creates NFT collections
   - Manages collection relationships
   - Handles collection verification

3. **NFT Operations**
   - `update-nft.ts`: Updates NFT metadata
   - `verify-metaplex-nft.ts`: Verifies NFT ownership
   - Includes proper error handling

## Getting Started

### Prerequisites:
- Node.js (v14 or higher)
- Solana CLI tools
- A Solana wallet (Phantom, Solflare, etc.)
- Basic understanding of TypeScript
- Git

### Environment Setup:

1. **Clone the Repository**
   ```bash
   git clone [repository-url]
   cd solana-development-path
   ```

2. **Install Dependencies**
   ```bash
   # For each section you want to use:
   cd [section-directory]
   npm install
   ```

3. **Configure Environment**
   ```bash
   # Copy the example environment file
   cp .env.example .env
   
   # Edit .env with your credentials:
   SECRET_KEY=your_private_key
   RPC_ENDPOINT=https://api.devnet.solana.com
   ```

4. **Run the Examples**
   ```bash
   # For TypeScript files:
   ts-node [filename].ts
   
   # For Next.js applications:
   npm run dev
   ```

## Development Tips

1. **Network Selection**
   - Use devnet for testing
   - Switch to mainnet for production
   - Configure in `.env` file

2. **Error Handling**
   - All examples include proper error handling
   - Check transaction confirmations
   - Monitor network status

3. **Security Best Practices**
   - Never commit private keys
   - Use environment variables
   - Implement proper access controls

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions or issues:
1. Check the documentation
2. Open an issue
3. Join our community discussions 