import React from 'react';
import {useCrate, useTakeEffect} from './utils/hooks';
import logo from './logo.svg';
import './App.css';

const App: React.FC = () => {
  const mod = useCrate();
  const [response, setResponse] = React.useState();

  useTakeEffect(() => {
    const resp = mod.greet('Hello', ['from', 'TypeScript']);
    setResponse(resp);
  }, [mod]);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>{response}</p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer">
          Learn React
        </a>
      </header>
    </div>
  );
};

export default App;
