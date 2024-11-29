import { createSignal } from "solid-js";
import { Keypair } from "@solana/web3.js";
import { mnemonicToSeed } from "@scure/bip39";
import bs58 from "bs58";

import "./importWallet.css";

const ImportWallet = () => {
  const [wordCount, setWordCount] = createSignal<12 | 24>(12);
  const [phraseArray, setPhraseArray] = createSignal<string[]>(Array(12).fill(""));
  const [keypair, setKeypair] = createSignal<Keypair | null>(null);

  const handleWordCountChange = (count: 12 | 24) => {
    setWordCount(count);
    setPhraseArray(Array(count).fill("")); 
  };

  const updateWord = (index: number, value: string) => {
    const words = value.split(/[\s\n]+/); 
    const updatedPhrase = [...phraseArray()];
  
    
    for (let i = 0; i < words.length; i++) {
      if (index + i < updatedPhrase.length) {
        updatedPhrase[index + i] = words[i].toLowerCase().trim();
      }
    }
  
    setPhraseArray(updatedPhrase);
  };

  const resetPhrase = () => {
    setPhraseArray(Array(wordCount()).fill(""));
  };
  
  const generateWallet = async () => {
    const phrase = phraseArray().join(" ").trim();
    try {
      const seed = await mnemonicToSeed(phrase);
      const generatedKeypair = Keypair.fromSeed(seed.slice(0, 32));
      setKeypair(generatedKeypair);
      console.log("Seed Phrase:", phrase);
      console.log("Public Key:", generatedKeypair.publicKey.toBase58());
      console.log("Private Key (Base58):", bs58.encode(generatedKeypair.secretKey));
    } catch (error) {
      alert!(`Failed to generate wallet: ${error}`);
    }
  };

  return (
    <div class="import-wallet-container">
      <h2>Import Solana Wallet</h2>
      <div class="word-count-selector">
        <label>
          <input
            type="radio"
            checked={wordCount() === 12}
            onChange={() => handleWordCountChange(12)}
          />
          12 words
        </label>
        <label>
          <input
            type="radio"
            checked={wordCount() === 24}
            onChange={() => handleWordCountChange(24)}
          />
          24 words
        </label>
      </div>
      <form
        class="wallet-form"
        onSubmit={(e) => {
          e.preventDefault();
          generateWallet();
        }}
      >
        <div class="seed-phrase-input">
          {phraseArray().map((word, index) => (
            <div class="seed-phrase-input-item">
              <input
                type="text"
                value={word}
                onInput={(e) => updateWord(index, e.currentTarget.value)}
              />
              <span>{index + 1}</span>
            </div>
          ))}
        </div>
        <button type="submit">Generate Wallet</button>
        <button type="button" onClick={resetPhrase}>Reset</button>
      </form>
      {keypair() && (
        <div>
          <h3>Wallet Details</h3>
          <p>
            <strong>Public Key:</strong> {keypair()?.publicKey.toBase58()}
          </p>
        </div>
      )}
    </div>
  );
};

export default ImportWallet;
