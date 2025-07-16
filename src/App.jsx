import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [recording, setRecording] = useState(false);
  const [events, setEvents] = useState([]);
  const eventsRef = useRef([]);

  // 记录事件
  const recordEvent = (type, e) => {
    const event = {
      type,
      x: e.clientX,
      y: e.clientY,
      time: Date.now(),
      target: e.target.tagName,
    };
    eventsRef.current.push(event);
    setEvents([...eventsRef.current]);
  };

  // 监听事件
  const startCapture = () => {
    setRecording(true);
    eventsRef.current = [];
    setEvents([]);
    window.addEventListener("click", clickHandler, true);
    window.addEventListener("mousemove", hoverHandler, true);
    invoke("start_recording", {});
  };

  const stopCapture = async () => {
    setRecording(false);
    window.removeEventListener("click", clickHandler, true);
    window.removeEventListener("mousemove", hoverHandler, true);
    await invoke("stop_recording", { events: eventsRef.current });
  };

  const clickHandler = (e) => recordEvent("click", e);
  let lastHover = 0;
  const hoverHandler = (e) => {
    // 只每100ms记录一次hover，减少数据量
    if (Date.now() - lastHover > 100) {
      recordEvent("hover", e);
      lastHover = Date.now();
    }
  };

  useEffect(() => {
    // 启动时检测 ffmpeg
    invoke("check_ffmpeg_installed").then((ok) => {
      if (!ok) {
        alert("未检测到 ffmpeg，请先安装后再使用本软件！");
        // 你也可以弹窗引导用户安装
      }
    });
  }, []);

  return (
    <main className="container">
      <h1>BugShot 桌面录屏</h1>
      <div style={{ margin: "20px 0" }}>
        {recording ? (
          <button onClick={stopCapture}>停止录制</button>
        ) : (
          <button onClick={startCapture}>开始录制</button>
        )}
      </div>
      <div>
        <h3>已记录事件数：{events.length}</h3>
        <ul style={{ maxHeight: 200, overflow: "auto", fontSize: 12 }}>
          {events.slice(-10).map((ev, i) => (
            <li key={i}>
              [{ev.type}] {ev.x},{ev.y} {ev.target} {ev.time}
            </li>
          ))}
        </ul>
      </div>
    </main>
  );
}

export default App;
