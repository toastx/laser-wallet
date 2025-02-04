import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { PublicKey } from "@solana/web3.js";
import styles from "./Home.module.css";
import PortfolioViewer from "../ui/PortfolioViewer";
import WalletActions from "../ui/WalletActions";

function Home() {
    const [pubkey, setPubkey] = createSignal<any>(null);
    
    onMount(async () => {
        let pubkey1: any = await invoke("get_wallet");
        let publicKey = new PublicKey(pubkey1).toBase58();
        setPubkey(publicKey);
    });
    
    const tokens = [
        { symbol: "BTC", balance: 0.5, value: 20000, price: 40000 },
        { symbol: "ETH", balance: 2.0, value: 4000, price: 2000 },
        { symbol: "USDT", balance: 500, value: 500, price: 1 },
        { symbol: "SOL", balance: 10, value: 2000, price: 200 },
    ];
    const totalBalance = tokens.reduce((acc, token) => acc + token.value, 0);
    


    
    return (!pubkey() && 
        <div class={styles.homeContainer}>
            
            <WalletActions />
            <PortfolioViewer totalBalance={totalBalance} holdings={tokens} pubkey={ pubkey()} />
        </div>);
}

export default Home;