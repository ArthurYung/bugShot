import React from "react";
import { createRoot } from "react-dom/client";
import RecordingTip from "./RecordingTip";
import "./RecordingTip.css";

const root = createRoot(document.getElementById("recording-root"));
root.render(<RecordingTip />);