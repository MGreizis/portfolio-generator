// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";
import Sidebar from './components/Sidebar';
import TopNavbar from './components/TopNavbar';

function App() {
  return (
    <>
      <TopNavbar />
      <main className="bg-bgblack">
        <Sidebar />
      </main>
    </>
  );
}

export default App;
