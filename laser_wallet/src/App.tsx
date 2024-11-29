import "./App.css";
import StartPage from "./components/StartPage/StartPage";
import newWallet from "./components/newWallet/newWallet";
import importWallet from "./components/importWallet/importWallet";
import { Route, Router } from "@solidjs/router";
import Home from "./components/home/Home";

function App() {
  

  return (
    <div>
      
      <Router>
        <Route path="/" component={StartPage} />
        <Route path="/newWallet" component={newWallet} />
        <Route path="/importWallet" component={importWallet} />
        <Route path="/home" component={Home} />
      </Router>
      {/* {<StartPage />} */}
      </div>
      
  );
}

export default App;
