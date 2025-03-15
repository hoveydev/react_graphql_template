import { useState } from 'react';

import { useQuery, gql } from '@apollo/client';

import reactLogo from './assets/react.svg';

import viteLogo from '/vite.svg';
import './App.css';

const _CHANGE_ME_FIRST_QUERY = gql`
  query _CHANGE_ME_FIRST_TYPE {
    type {
      fieldOne
      fieldTwo
    }
  }
`;

const DisplayData = () => {
  const { loading, error, data } = useQuery(_CHANGE_ME_FIRST_QUERY);

  if (loading) {
    return <p>Loading...</p>;
  }
  if (error) {
    return <p>Error: {error.message}</p>;
  }

  return (
    <div>
      <div className="field-one">
        <b>{data.type.fieldOne}</b>
      </div>
      <div className="field-two">
        <b>{data.type.fieldTwo}</b>
      </div>
    </div>
  );
};

const App = () => {
  const [count, setCount] = useState(0);
  const incrementClick = () => {
    return setCount((prevCount) => {
      return prevCount + 1;
    });
  };

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={incrementClick}>count is {count}</button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <div className="card">
        This is where the placeholder GQL data will go!
        <DisplayData />
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
};

export default App;
