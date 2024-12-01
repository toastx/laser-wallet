import { createSignal } from "solid-js";
import { entropyToMnemonic } from "@scure/bip39";
import { wordlist } from "@scure/bip39/wordlists/english";
import styles from "./newWallet.module.css";
import { useNavigate } from "@solidjs/router";
import { invoke } from "@tauri-apps/api/core";
import { PublicKey } from "@solana/web3.js";


const newWallet = () => {
  const navigate = useNavigate();
  const [wallet, setWallet] = createSignal<{ publicKey: string; seedPhrase: string;} | null>({publicKey: "", seedPhrase: ""});

  const generateWallet = async () => {

      console.log("Generating wallet...");
    try {
      
      const generatedWallet: { seed: any, public_key: string } = await invoke('generate_wallet');
      const seed_phrase = new Uint8Array(generatedWallet.seed);
      const seedPhrase = entropyToMnemonic(seed_phrase, wordlist);
      const publicKey = new PublicKey(generatedWallet.public_key);
      
      setWallet({
        publicKey: publicKey.toBase58(),
        seedPhrase: seedPhrase,
        });
      }
      catch (error) {
        console.log("Error generating wallet:", error);
      }
  };

  const storeWallet = async () => {
    // await invoke('store_wallet', {
    //   wallet: {
    //     publicKey: wallet()!.publicKey,
    //     privateKey: wallet()!.privateKey,
    //   }
    // });
    navigate("/home");
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text).then(() => {
      alert('Copied to clipboard!');
    });
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
          <button onClick={storeWallet} class={styles.homeButton}>Continue</button>
        </div>

      )}
    </div>

  );
};

export default newWallet;
