import { createMemo, For, createSignal } from "solid-js";
import styles from "./PortfolioViewer.module.css";
import { FiActivity, FiRepeat, FiSend } from "solid-icons/fi";

interface Token {
  symbol: string;
  balance: number;
  value: number;
  price: number;
}

interface PortfolioViewerProps {
  totalBalance: number;
  pnl: { amount: number; percent: number };
  holdings: Token[];
  pubkey: string;
}

const PortfolioViewer = (props: PortfolioViewerProps) => {
  const [hoveredToken, setHoveredToken] = createSignal<string | null>(null);

  // Fallback to defaults if pnl is missing or incomplete
  const pnlAmount = props.pnl?.amount ?? 0;
  const pnlPercent = props.pnl?.percent ?? 0;

  // Sort holdings by value (highest to lowest)
  const sortedHoldings = createMemo(() =>
    [...props.holdings].sort((a, b) => b.value - a.value)
  );

  return (
    <div class={styles.container}>
      {/* Header with Total Balance */}
      <div class={styles.header}>
        <h2 class={styles.pubkey}>{props.pubkey}</h2>
        <h2 class={styles.totalBalance}>${props.totalBalance.toFixed(2)}</h2>
        <div class={styles.pnl}>
          <span class={styles.pnlAmount}>
            {pnlAmount > 0 ? "+" : ""}
            ${pnlAmount.toFixed(2)}
          </span>
          <span
            class={`${styles.pnlPercent} ${
              pnlAmount >= 0 ? styles.green : styles.red
            }`}
          >
            ({pnlPercent > 0 ? "+" : ""}
            {pnlPercent.toFixed(2)}%)
          </span>
        </div>
      </div>

      {/* Token Holdings */}
      <div class={styles.tokenList}>
        <For each={sortedHoldings()}>
          {(token) => (
            <div
              class={styles.tokenRow}
              onMouseEnter={() => setHoveredToken(token.symbol)}
              onMouseLeave={() => setHoveredToken(null)}
            >
              <div class={styles.tokenRowInner}>
              {/* Token Symbol and Balance */}
              <div class={styles.tokenPart}>
                <span class={styles.tokenSymbol}>{token.symbol}</span>
                <span class={styles.tokenBalance}>
                  Balance: {token.balance.toFixed(4)}
                </span>
              </div>

              {/* Token Price */}
              <div class={styles.tokenPart}>
                <span class={styles.tokenPrice}>
                  ${token.price.toFixed(2)}
                </span>
                {/* Placeholder for price chart */}
                <div class={styles.tokenChartPlaceholder}>
                  [Chart Placeholder]
                </div>
              </div>

              {/* Token Value */}
              <div class={styles.tokenPart}>
                <span class={styles.tokenValue}>
                  ${token.value.toFixed(2)}
                </span>
                </div>
              </div>

              {/* Dock Actions */}
              <div class={styles.tokenDockWrapper}>
              {hoveredToken() === token.symbol && (
                <div class={styles.tokenDock}>
                  <button class={styles.actionButton}>
                    <FiActivity size={16} /> Activity
                  </button>
                  <button class={styles.actionButton}>
                    <FiRepeat size={16} /> Swap
                  </button>
                  <button class={styles.actionButton}>
                    <FiSend size={16} /> Send
                  </button>
                </div>
                )}
              </div>
            </div>
          )}
        </For>
      </div>
    </div>
  );
};

export default PortfolioViewer;
