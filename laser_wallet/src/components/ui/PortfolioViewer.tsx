import { createMemo, For } from "solid-js";
import styles from "./PortfolioViewer.module.css";

interface Token {
  symbol: string;
  balance: number;
  value: number;  // Value in portfolio currency (e.g., USD)
  price: number;  // Price of the token (e.g., USD per token)
}

interface PortfolioViewerProps {
  totalBalance: number;
  holdings: Token[];
}

const PortfolioViewer = (props: PortfolioViewerProps) => {
  // Sort holdings by value (highest to lowest)
  const sortedHoldings = createMemo(() =>
    [...props.holdings].sort((a, b) => b.value - a.value)
  );

  return (
    <div class={styles.container}>
      {/* Total Balance */}
      <div class={styles.header}>
        <h2 class={styles.totalBalance}>
          ${props.totalBalance.toFixed(2)}
        </h2>
      </div>

      {/* Token Holdings */}
      <div class={styles.tokenList}>
        <For each={sortedHoldings()}>
          {(token) => (
            <div class={styles.tokenRow}>
              <div class={styles.tokenPart}>
                <span class={styles.tokenSymbol}>{token.symbol}</span>
              </div>
              <div class={styles.tokenPart}>
                <span class={styles.tokenBalance}>
                  Balance: {token.balance.toFixed(4)}
                </span>
              </div>
              <div class={styles.tokenPart}>
                <span class={styles.tokenValue}>
                  ${token.value.toFixed(2)}
                </span>
              </div>
              <div class={styles.tokenPart}>
                <span class={styles.tokenPrice}>
                  ${token.price.toFixed(2)}
                </span>
              </div>
            </div>
          )}
        </For>
      </div>
    </div>
  );
};

export default PortfolioViewer;
