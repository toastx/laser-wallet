import { createSignal } from "solid-js";
import { Keypair } from '@solana/web3.js';
import { generateMnemonic, mnemonicToSeedSync} from '@scure/bip39';
import { wordlist } from '@scure/bip39/wordlists/english';

import bs58 from 'bs58';


const newWallet = () => {
  const [wallet, setWallet] = createSignal<{ publicKey: string; seedPhrase: string } | null>({publicKey: "", seedPhrase: ""});

  const generateWallet = () => {
    const seedPhrase = generateMnemonic(wordlist);
    console.log('seedPhrase:', seedPhrase);

  // Convert mnemonic to seed
    const seed = mnemonicToSeedSync(seedPhrase);
    console.log('seed:', seed);

    const keypair = Keypair.fromSeed(seed.slice(0, 32))
    console.log(bs58.encode(keypair.secretKey))
    console.log('keypair:', keypair);
    setWallet({
      publicKey: keypair.publicKey.toBase58(),
      seedPhrase: seedPhrase,
    });
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text).then(() => {
      alert('Copied to clipboard!');
    });
  };

  return (
    <div>
      <h2>Generate Solana Wallet</h2>
      <button onClick={generateWallet} style={{ padding: '10px 20px', cursor: 'pointer' }}>
        Generate Wallet
      </button>

      {wallet() && (
        <div >
          <h3>Wallet Details</h3>
          <p><strong>Public Key:</strong> {wallet()!.publicKey}</p>
          <p>
            <strong>Seed Phrase:</strong>{' '}
            <span >{wallet()!.seedPhrase}</span>
          </p>
          <button
            onClick={() => copyToClipboard(wallet()!.seedPhrase)}
          >
            Copy Seed Phrase
          </button>
        </div>
      )}
    </div>
  );
};

export default newWallet;
