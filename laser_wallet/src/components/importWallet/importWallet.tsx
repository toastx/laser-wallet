import { createSignal } from "solid-js";
import { Keypair } from '@solana/web3.js';
import {mnemonicToSeed} from '@scure/bip39';


import bs58 from 'bs58';


const importWallet = () => {
  const [seedPhrase, setSeedPhrase] = createSignal<string>("");
  const [keypair, setKeypair] = createSignal<Keypair | null>(null);
  const generateWallet = async () => {
    
    console.log('seedPhrase:', seedPhrase());
    const seed =  mnemonicToSeed(seedPhrase());
    console.log('seed:', seed);

    const keypair = Keypair.fromSeed((await seed).slice(0, 32))
    console.log(bs58.encode(keypair.secretKey))
    console.log('keypair:', keypair);
    setKeypair(keypair);
  };
  return (
    <div>
      <h2>Import Solana Wallet</h2>
      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          generateWallet();
        }}
      >
        <input
          id="greet-input"
          placeholder="Enter the seed phrase"
          onChange={(e) => setSeedPhrase(e.currentTarget.value)}
        />
        <button type="submit">Add</button>
      </form>
    {seedPhrase() && (
        <div >
          <h3>Wallet Details</h3>
          <p><strong>Public Key:</strong> {keypair()?.publicKey.toBase58()}</p>
        </div>
      )}
    </div>
  );
};

export default importWallet
