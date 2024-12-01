// import { invoke } from "@tauri-apps/api/core";
import { onMount } from "solid-js";
import "./StartPage.css";
import { useNavigate } from "@solidjs/router"
import { invoke } from "@tauri-apps/api/core";

console.log("StartPage component rendering");

function StartPage() {
  const navigate = useNavigate();

  // onMount(async () => {
  //   try {
  //     const existingWallet = await invoke('get_stored_wallet');
  //     if (existingWallet) {
  //       navigate("/home");
  //     } else {
  //       navigate("/wallet-setup"); // or whatever route you use for wallet creation/import
  //     }
  //   } catch (error) {
  //     console.error("Error checking stored wallet:", error);
  //     navigate("/wallet-setup"); // Fallback to wallet setup on error
  //   }
  // });

  const newWallet = async () => {
    console.log("new wallet");
    // await invoke("create_wallet");
    navigate("/newWallet");
  };

  const importWallet = async () => {
    console.log("import wallet");
    // await invoke("import_wallet");
    navigate("/importWallet");
  };

  return (<div class="start-page">
    <div>
        <h1>Welcome to Laser Wallet</h1>
    </div>
    <div class="wallet-options">
      <button onClick={newWallet}>Create a new wallet</button>
      <button onClick={importWallet}>Import existing wallet</button>
      </div>
    </div>
  );
}

export default StartPage;
