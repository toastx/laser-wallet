import "./App.css";
import StartPage from "./components/StartPage/StartPage";
import newWallet from "./components/newWallet/newWallet";
import importWallet from "./components/importWallet/importWallet";
import { Route, Router } from "@solidjs/router";

function App() {
  

  return (
    <div>
      <h1>Welcome to Laser Wallet</h1>
      <Router>
        <Route path="/" component={StartPage} />
        <Route path="/newWallet" component={newWallet} />
        <Route path="/importWallet" component={importWallet} />
      </Router>
      {/* {<StartPage />} */}
      </div>
      
  );
}

export default App;
