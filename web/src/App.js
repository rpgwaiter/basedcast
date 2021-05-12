import React from 'react';
import { Button, GlobalStyle, ThemeProvider, Video } from '@react95/core';
import styled from 'styled-components';

import logo from './windows95_logo.png';
import bg from './bg.gif';

const Centered = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background: url(${bg});
  background-size: cover;

  height: 100%;
`;

function App() {
  return (
    <ThemeProvider>
      <GlobalStyle></GlobalStyle>
      <Centered>
        <img alt="logo" src={logo} style={{ width: 200 }} />
        <Video src="https://cast.based.zone"></Video>
        <Button>Based</Button>
      </Centered>
    </ThemeProvider>
  );
}

export default App;
