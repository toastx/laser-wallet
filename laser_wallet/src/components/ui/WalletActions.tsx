import styles from "./WalletActions.module.css";

function WalletActions() {
    return <div class={styles.walletActions}>
        <button>Send</button>
        <button>Receive</button>
    </div>
}

export default WalletActions;