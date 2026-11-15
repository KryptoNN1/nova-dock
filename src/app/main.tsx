import React from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import "../design/tokens.css";

function Dock() {
  return (
    <div className="dock-stage">
      <div className="dock" role="toolbar" aria-label="NovaDock">
        <span className="dock-empty">Uygulamalar Aşama 2'de eklenecek</span>
        <button className="dock-btn" onClick={() => invoke("quit_and_restore")}>
          Görev çubuğunu geri yükle ve çık
        </button>
      </div>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById("root")!).render(<Dock />);
