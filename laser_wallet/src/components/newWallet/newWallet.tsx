import { createSignal } from "solid-js";
import { Keypair, PublicKey } from '@solana/web3.js';
import { generateMnemonic, mnemonicToSeedSync} from '@scure/bip39';
import { wordlist } from '@scure/bip39/wordlists/english';
import { Connection } from "@solana/web3.js";
import styles from "./newWallet.module.css";
import { useNavigate } from "@solidjs/router";


const connection = new Connection("https://mainnet.helius-rpc.com/?api-key=05c3bb84-2447-46fa-b7bf-038ace8035a4");
const newWallet = () => {
  const navigate = useNavigate();
  const [wallet, setWallet] = createSignal<{ publicKey: string; seedPhrase: string} | null>({publicKey: "", seedPhrase: ""});

  const home = async () => {
    navigate("/home");
  };

  const generateWallet = async () => {

      console.log("Generating wallet...");
      try {
        const seedPhrase = generateMnemonic(wordlist);
        const seed = mnemonicToSeedSync(seedPhrase);
        
        const keypair = Keypair.fromSeed(seed.slice(0, 32))
        if (await checkKeypair(keypair.publicKey.toBase58())) {
          setWallet({
            publicKey: keypair.publicKey.toBase58(),
            seedPhrase: seedPhrase,
          });
        }
      
      } catch (error) {
        console.log("Error generating wallet:", error);
        
      }
    
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text).then(() => {
      alert('Copied to clipboard!');
    });
  };

  const checkKeypair = async (publicKey: string) => {
    const accountInfo = await connection.getAccountInfo(new PublicKey(publicKey));
    if (accountInfo) {
      console.log("Keypair already associated with an account.");
      console.log(accountInfo.owner.toBase58());
      return false;
    } else {
      
      return true;
    }
  };

  const downloadSeedPhrase = () => {
    const element = document.createElement("a");
    const file = new Blob([wallet()!.seedPhrase], {type: 'text/plain'});
    element.href = URL.createObjectURL(file);
    element.download = `${wallet()!.publicKey}.txt`;
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
  };

  return (
    <div class={styles.container}>
      <h2>Generate Solana Wallet</h2>
      <button onClick={generateWallet} class={styles.generateButton}>
        Generate Wallet
      </button>

      {wallet()?.publicKey && wallet()?.seedPhrase && (
        <div class={styles.walletDetails}>
          <h3>Wallet Details</h3>
          <p><strong>Public Key:</strong> {wallet()!.publicKey}</p>
          <div class={styles.seedPhraseContainer}>
            <strong>Seed Phrase:</strong>
            <div class={styles.seedPhraseGrid}>
              {wallet()!.seedPhrase.split(' ').map((word, index) => (
                <div class={styles.seedWord}>
                  <span class={styles.wordIndex}>{index + 1}</span>
                  {word}
                </div>
              ))}
            </div>
          </div>
          <div class={styles.buttonGroup}>
            <button
              onClick={() => copyToClipboard(wallet()!.seedPhrase)}
              class={styles.actionButton}
            >
              Copy Seed Phrase
            </button>
            <button
              onClick={downloadSeedPhrase}
              class={styles.actionButton}
            >
              Download Seed Phrase
            </button>
          </div>
          <div class={styles.warning}>
            ⚠️ Warning: Please save your seed phrase securely. 
            You won't be able to recover it once you leave this page!
          </div>
          <button onClick={home} class={styles.homeButton}>Continue</button>
        </div>

      )}
    </div>

  );
};

export default newWallet;
