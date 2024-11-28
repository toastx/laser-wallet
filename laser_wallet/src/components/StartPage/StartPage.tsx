// import { invoke } from "@tauri-apps/api/core";
import "./StartPage.css";
import { useNavigate } from "@solidjs/router"

console.log("StartPage component rendering");

function StartPage() {
  const navigate = useNavigate();

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
    <div class="wallet-options">
      <button onClick={newWallet}>Create a new wallet</button>
      <button onClick={importWallet}>Import existing wallet</button>
      </div>
    </div>
  );
}

export default StartPage;
